# Release ICU packaging

Current Retail builds require native ICU even without GUI/CASC. Mists does not.
The native API contract lives in [native Intl linking](specs/intl-native-linking.md);
[PTR provisioning](ptr-icu-build.md) owns the shared pinned Windows manifest.

## Native releases

[Release workflow](../.github/workflows/release.yml) preserves the six
Retail/Mists platform builds and their `cargo release-*` aliases:
`--release --no-default-features --features sound,gui,client-<profile>`, explicit
platform target, both `wow-sim` and `wow-cli`, CI Cargo configuration.

- Linux explicitly installs `pkg-config`, `libicu-dev`, ALSA and udev headers.
  Retail reports both ICU package versions and requires >=72. Tarballs retain
  shared linkage: recipient hosts need compatible glibc, GUI/audio dependencies
  and the exact ICU SONAMEs recorded by `ldd`; this is not universal portability.
- Windows Retail checks out the vcpkg revision from `vcpkg.json`, bootstraps it,
  installs `x64-windows-static-md` into that checkout's `installed/`, and exports
  `VCPKG_ROOT` plus `VCPKGRS_TRIPLET`. ICU/data are static; MSVC CRT remains dynamic.
- macOS Retail provisions Homebrew ICU. Before signing,
  [bundle-macos-icu.py](../scripts/bundle-macos-icu.py) follows actual ICU imports,
  copies the transitive ICU dylib closure into `Contents/Frameworks`, rewrites
  imports/IDs to bundle-relative paths, and rejects unresolved non-system imports.
  Sign dylibs before the outer app, verify the signature, then launch the packaged
  executable with `--version`. Mists has no ICU bundling step.

## Docker

[Dockerfile](../Dockerfile) builds headless Retail with Debian Bookworm ICU
headers. It copies the builder's dereferenced `libicui18n.so.72`, `libicuuc.so.72`
and `libicudata.so.72` into the amd64 distroless Debian12 runtime. Changing the
architecture or distro requires re-proving this explicit runtime closure.

The UI source stage checks out Gethe revision
`b9be9379719047977fca4dba1bb1d9988e8840b3` (selected Retail `12.1.5.70448`),
compares its complete AddOns path set against the committed
[Retail manifest](../data/blizzard-ui-files/retail.txt), and checks every file
before stamping completion/provenance. Provenance records the revision and
manifest SHA256. It does not claim CASC origin. Future profile updates must
move the source revision and manifest together; mismatches fail the image build.

[Docker workflow](../.github/workflows/docker.yml) loads one local image,
then [verify_docker_image.py](../.github/scripts/verify_docker_image.py) checks
`--version`, exact `[]` startup JSON, and two executed tiny-addon tests (Retail
interface epoch and frame-size round-trip). Containers have no network and a
90-second command limit. Zero tests cannot pass. Publication tags/pushes that
same tested image, never a second build. This is not Wowless or general addon
compatibility proof.

## Non-publishing rehearsal

On a remotely available implementation branch, with these workflow definitions
registered on the repository's default branch:

```text
gh workflow run release.yml --repo Osso/wow-ui-sim --ref release-icu-fixes
gh workflow run docker.yml --repo Osso/wow-ui-sim --ref release-icu-fixes
```

Neither workflow needs inputs. Every `workflow_dispatch`, including one on a
tag ref, is non-publishing. Release metadata uses `v0.0.0-ci`; the entire release
publisher requires a `push` event on `refs/tags/v*`. Docker login/push require a
`push` event on a tag. Build/artifact/cache uploads remain allowed. Do not create
a tag to rehearse; if registration blocks dispatch, resolve registration in a
separately authorized repository/fork rather than bypassing these gates.

## Proof limits

Local acceptance must separately record actual Linux release compilation,
resolved `ldd` imports and isolated headless startup; final Docker image startup,
addon count, filesystem ICU libraries and manifest/provenance; and native Windows
vcpkg build, release build, `dumpbin /DEPENDENTS` and bounded headless startup.
[Helper tests](../scripts/tests/test_release_helpers.py) exercise rejection and
synthetic closure behavior, not native loaders.

macOS is verified only by CI dispatch, not locally. Builder-host launch is not
clean-recipient portability proof; clean macOS execution without Homebrew ICU
remains separate. Existing ad-hoc signing is not notarization. Distributed ICU
requires its matching full license/third-party notices; provisioning libraries
alone is not a redistribution-notice audit.
