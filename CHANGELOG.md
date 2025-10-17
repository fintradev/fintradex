# Changelog
All notable changes to **FintraDex** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Guidelines**
> - List only things that actually landed on `main` and/or shipped in a tagged release.
> - Use these sections: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.
> - Put **plans/roadmap** in the README, not here.

---

## [Unreleased 5.0.0]
_Major governance and architecture upgrade with OpenGov v2 integration._

### Added
- **OpenGov v2 Governance**: Integrated advanced governance system with referenda, ranked collective, and fellowship pallets
- **Ranked Collective**: Merit-based governance with fellowship rankings and voting power
- **Core Fellowship**: Professional governance body with rank-based decision making
- **Conviction Voting**: Stake-weighted voting with time-locked token commitments
- **Enhanced Treasury**: Community-controlled funding with ranked collective oversight for proposal rejection

### Changed
- **Governance Architecture**: Upgraded from legacy democracy/council system to modern OpenGov v2
- **Treasury Configuration**: Enhanced treasury with ranked collective rejection authority
- **Runtime Configuration**: Streamlined pallet configuration for better performance and security

### Removed
- **Legacy Democracy Pallet**: Replaced with modern OpenGov v2 referenda system
- **Council Collective**: Replaced with merit-based ranked collective governance

### Fixed
- **Treasury Governance**: Resolved RejectOrigin configuration with proper ranked collective integration
- **Compilation Issues**: Fixed trait bound errors in governance pallet configurations

---

## [4.1.0] - 2024-12-XX
_Configuration refinements and governance improvements._

### Added
- **Transaction Payment**: Enhanced transaction payment configuration with detailed multiplier documentation
- **Treasury Governance**: Implemented ranked collective oversight for treasury proposal rejection

### Changed
- **Documentation**: Comprehensive comments added to all chain specification parameters
- **Treasury RejectOrigin**: Updated to use ranked collective members (rank 5+) for proposal rejection

### Fixed
- **Compilation Errors**: Resolved EnsureMember trait bound issues in treasury configuration
- **Import Dependencies**: Cleaned up unused imports and added required trait imports

---

## [4.0.0] - 2024-12-XX
_Major upgrade with cross-chain messaging capabilities._

### Added
- **Hyperbridge Integration**: Integrated Hyperbridge protocol for secure cross-chain messaging and interoperability on Paseo Parachain
- **ISMP Protocol**: Interoperable State Machine Protocol for cross-chain communication
- **Token Gateway**: Cross-chain asset transfer capabilities via Hyperbridge

### Changed
- **Runtime Architecture**: Upgraded to support Hyperbridge messaging infrastructure
- **Parachain Architecture**: Updated to enable trustless cross-chain operations

---

## [Unreleased 5.1.0]
_Latest configuration updates and chain specification improvements._

### Added
- **Chain Specification**: Updated plain_chain_spec.json with comprehensive nomination pools configuration

### Changed
- **Documentation**: Comprehensive comments added to all chain specification parameters

### Fixed
- **Documentation**: Enhanced chain specification with detailed parameter explanations

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

