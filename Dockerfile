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

# Copy test targets referenced by Cargo.toml's [[test]] sections.
COPY tests/ tests/

# Fetch all dependencies (cached layer — only invalidated by Cargo.toml/lock changes).
RUN cargo fetch --locked

# Copy the full source tree and build the simulator binary.
# --no-default-features skips the `sound` feature (rodio/audio) which has
# extra system library requirements not needed for headless test runs.
COPY build.rs ./
COPY data/ data/
COPY src/ src/
RUN cargo build --release --bin wow-sim --no-default-features --locked \
    && strip /build/target/release/wow-sim

# =============================================================================
# BlizzardUI Stage — sparse-checkout from Gethe/wow-ui-source
# =============================================================================
FROM alpine/git AS blizzard-ui

ARG BLIZZARD_UI_TAG=12.0.5
RUN git clone --filter=blob:none --no-checkout --depth=1 --branch ${BLIZZARD_UI_TAG} \
        https://github.com/Gethe/wow-ui-source.git /wow-ui-source \
    && cd /wow-ui-source \
    && git sparse-checkout init --cone \
    && git sparse-checkout set Interface/AddOns \
    && git checkout ${BLIZZARD_UI_TAG} \
    && rm -rf /wow-ui-source/.git

# =============================================================================
# Runtime Stage
# =============================================================================
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy stripped binary from build stage.
COPY --from=builder /build/target/release/wow-sim /app/wow-sim

# Copy data directories from the build context.
# These are read at runtime and are NOT compiled into the binary.
#
# BlizzardUI: Blizzard's base UI Lua/XML (loaded before addons)
COPY --from=blizzard-ui /wow-ui-source/Interface/AddOns/ /app/Interface/BlizzardUI/
# TestFramework: assertion library loaded automatically by `run-tests`
COPY Interface/AddOns/TestFramework/ /app/Interface/AddOns/TestFramework/

# Skip SavedVariables loading — no WTF directory is available in the image.
ENV WOW_SIM_NO_SAVED_VARS=1

ENTRYPOINT ["/app/wow-sim"]
