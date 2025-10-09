# Changelog
All notable changes to **FintraDex** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Guidelines**
> - List only things that actually landed on `main` and/or shipped in a tagged release.
> - Use these sections: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.
> - Put **plans/roadmap** in the README, not here.

---

## [Unreleased 4.0.0]
_Major upgrade with cross-chain messaging capabilities._

### Added
- **Hyperbridge Integration**: Integrated Hyperbridge protocol for secure cross-chain messaging and interoperability on Paseo Parachain.
- ISMP (Interoperable State Machine Protocol) pallet configuration for cross-chain communication.
- Enhanced cross-chain asset transfer capabilities via Hyperbridge.

### Changed
- Runtime upgraded to support Hyperbridge messaging infrastructure.
- Updated parachain architecture to enable trustless cross-chain operations.

---

## [Unreleased]
### Added
- Repository docs: `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`.
- CI workflow for `fmt`, `clippy`, `test`, and release build.
- Network info: Paseo Testnet (Parachain ID **4910**), RPC `wss://testnet.fintra.network`.

### Changed
- Consistent branding: **FintraDex** across README and repo.

### Fixed
- Minor build warnings resolved via `clippy` cleanups.

### Security
- Coordinated disclosure policy; added (optional) PGP channel for encrypted reports.

---

## [0.1.0-alpha] - 2025-10-08
_First public alpha suitable for testnet experimentation._

### Added
- Initial parachain runtime and node skeleton buildable via `cargo build --release`.
- Dev/local run support (`--dev`).
- Chain configuration and basic documentation to connect to Paseo Testnet.

---

## Legend
- **Added** for new features.
- **Changed** for changes in existing functionality.
- **Deprecated** for soon-to-be removed features.
- **Removed** for removed features.
- **Fixed** for any bug fixes.
- **Security** for vulnerability reports and fixes.

---

## Maintenance notes (for maintainers)
- When cutting a release:
  1. Update the relevant section above with changes since last tag.
  2. Create a tag (e.g., `v0.1.1`) and a GitHub Release with artifacts (runtime `.wasm`, chain spec JSON).
  3. Move items from **Unreleased** into the new version block with the release date.
- Keep roadmap/milestones in `README.md` or GitHub Projects — do not place future plans here.

