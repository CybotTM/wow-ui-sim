# wow-ui-sim Dockerfile
#
# Build: docker build -t ghcr.io/osso/wow-ui-sim .
# Usage: docker run --rm -v ./MyAddon:/app/Interface/AddOns/MyAddon ghcr.io/osso/wow-ui-sim run-tests MyAddon

# =============================================================================
# Build Stage
# =============================================================================
FROM rust:1.92-bookworm AS builder

# Install system build dependencies:
# - clang + mold: fast linker configured in .cargo/config.toml
# - git: needed by some build scripts
# - pkg-config + cmake: required by C-backed Rust crates (mlua vendored Lua, wgpu)
RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    mold \
    git \
    pkg-config \
    cmake \
    libicu-dev \
    fonts-dejavu-core \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy dependency manifests first so Docker can cache the dep-fetch layer.
# iced-wgpu-patched/ is a [patch.crates-io] override and must be present
# before `cargo fetch` or any build step.
COPY Cargo.toml Cargo.lock ./
COPY .cargo/config.toml .cargo/config.toml
COPY iced-wgpu-patched/ iced-wgpu-patched/
COPY iced-dynamic/ iced-dynamic/
COPY xtask/ xtask/
# Cargo resolves workspace/dev-dependency manifests even for binary-only builds.
COPY crates/test-harness/ crates/test-harness/

# Cargo validates explicitly declared targets before fetching dependencies.
# Derive empty sources from the manifest so adding targets cannot stale a list.
COPY scripts/docker-stub-cargo-targets.sh /tmp/docker-stub-cargo-targets.sh
RUN sh /tmp/docker-stub-cargo-targets.sh /build/Cargo.toml && cargo fetch --locked

# Copy the full source tree and build the simulator binary.
# --no-default-features skips sound, GUI, and CASC dependencies not needed for
# headless test runs. client-retail still selects the required runtime profile.
COPY build.rs ./
# Retail selects retail-12-1-5, including the native ICU build/runtime dependency.
COPY build/ build/
COPY native/ native/
COPY data/ data/
# Overwrite every binary stub with real sources. Test/example/bench stubs are
# manifest-only inputs: the binary-only build below never compiles them.
COPY src/ src/
RUN cargo build --release --bin wow-sim --no-default-features --features client-retail --locked \
    && strip /build/target/release/wow-sim \
    && mkdir /icu-runtime \
    && cp -L /usr/lib/x86_64-linux-gnu/libicui18n.so.72 \
             /usr/lib/x86_64-linux-gnu/libicuuc.so.72 \
             /usr/lib/x86_64-linux-gnu/libicudata.so.72 /icu-runtime/

# =============================================================================
# BlizzardUI Stage — sparse-checkout from Gethe/wow-ui-source
# =============================================================================
FROM alpine/git AS blizzard-ui

# Selected Retail 12.1.5.70448 source; see docs/updating-blizzard-ui-to-a-new-patch.md.
# Never publish a cache whose tree differs from the compiled Retail manifest.
COPY data/blizzard-ui-files/retail.txt /tmp/retail.txt
RUN git init /wow-ui-source \
    && cd /wow-ui-source \
    && git remote add origin https://github.com/Gethe/wow-ui-source.git \
    && git fetch --depth=1 origin b9be9379719047977fca4dba1bb1d9988e8840b3 \
    && git sparse-checkout init --cone \
    && git sparse-checkout set Interface/AddOns \
    && git checkout --detach FETCH_HEAD \
    && test "$(git rev-parse HEAD)" = b9be9379719047977fca4dba1bb1d9988e8840b3 \
    && git ls-tree -r --name-only HEAD Interface/AddOns/ | cut -d/ -f3- | LC_ALL=C sort > /tmp/actual.txt \
    && LC_ALL=C sort /tmp/retail.txt > /tmp/expected.txt \
    && diff -u /tmp/expected.txt /tmp/actual.txt \
    && while IFS= read -r entry; do test -f "Interface/AddOns/$entry" || exit 1; done < /tmp/retail.txt \
    && manifest_sha256="$(sha256sum /tmp/retail.txt | cut -d' ' -f1)" \
    && printf 'profile=retail\nversion=12.1.5.70448\nsource=gethe-image-build\ngethe_revision=b9be9379719047977fca4dba1bb1d9988e8840b3\nmanifest_sha256=%s\nfallback=none\n' "$manifest_sha256" > Interface/AddOns/.wow-ui-sim-blizzard-ui-provenance \
    && touch Interface/AddOns/.wow-ui-sim-blizzard-ui-complete \
    && rm -rf /wow-ui-source/.git

# =============================================================================
# Runtime Stage
# =============================================================================
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy stripped binary from build stage.
COPY --from=builder /build/target/release/wow-sim /app/wow-sim

# Debian 12 ICU code and data libraries, from the same builder package set.
COPY --from=builder /icu-runtime/ /usr/lib/x86_64-linux-gnu/

# Copy data directories from the build context.
# These are read at runtime and are NOT compiled into the binary.
#
# BlizzardUI: Blizzard's base UI Lua/XML, placed in the cache path the
# simulator checks. The .wow-ui-sim-blizzard-ui-complete marker is
# created in the blizzard-ui stage and tells the runtime the cache
# is ready, skipping the CASC sync attempt.
COPY --from=blizzard-ui /wow-ui-source/Interface/AddOns/ /root/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/

# TestFramework: assertion library loaded automatically by `run-tests`
COPY Interface/AddOns/TestFramework/ /app/Interface/AddOns/TestFramework/

# DejaVu fonts for text shaping fallback when CASC is unavailable.
# fontdb::load_system_fonts() picks these up from the standard path.
COPY --from=builder /usr/share/fonts/truetype/dejavu/ /usr/share/fonts/truetype/dejavu/

# Skip SavedVariables loading — no WTF directory is available in the image.
ENV WOW_SIM_NO_SAVED_VARS=1

ENTRYPOINT ["/app/wow-sim"]
