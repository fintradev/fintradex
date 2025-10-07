# Contributing to FintraDex

Thank you for your interest in contributing to **FintraDex**! 🚀  
We welcome contributions of all kinds — features, fixes, docs, tests, and reviews.

---

## 📋 Table of Contents
- Code of Conduct
- Getting Started
- Development Workflow
- Coding Standards
- Commit Messages
- Pull Request Process
- Testing Guidelines
- Documentation
- Community
- Recognition
- License

---

## 📜 Code of Conduct
This project follows the Contributor Covenant (see `CODE_OF_CONDUCT.md`).  
By participating, you agree to uphold it.

• Conduct/Community issues: team@fintradex.io  
• Security vulnerabilities: security@fintradex.io (see `SECURITY.md`) — do **not** open a public issue.

---

## 🚀 Getting Started

### Prerequisites
• Rust: 1.87.0+ (or rely on repo `rust-toolchain.toml` if present)  
• Cargo: latest via `rustup`  
• System deps  
  – Ubuntu/Debian: `build-essential cmake pkg-config libssl-dev`  
  – macOS: Xcode Command Line Tools  
  – Windows: Visual Studio Build Tools

### Environment Setup
    # clone your fork
    git clone https://github.com/YOUR_USERNAME/fintradex.git
    cd fintradex

    # toolchain & targets
    rustup default 1.87.0
    rustup target add wasm32-unknown-unknown
    rustup component add rust-src rustfmt clippy

    # optional helpers
    cargo install --locked staging-chain-spec-builder@10.0.0
    cargo install --locked polkadot-omni-node@0.5.0
    # optional local quality tools
    cargo install --locked cargo-audit gitleaks cargo-nextest

    # build
    cargo build --release -p fintradex-node || cargo build --release

    # run tests
    cargo test --all --locked

---

## 🔄 Development Workflow

### 1) Create a branch
Use clear prefixes:
• `feature/*` – new features  
• `fix/*` – bug fixes  
• `docs/*` – docs updates  
• `refactor/*` – refactoring  
• `test/*` – tests only  
• `chore/*` – maintenance

    git checkout -b feature/your-feature-name

### 2) Make changes
• Keep PRs focused and reasonably small  
• Add/adjust tests with your code  
• Update docs as needed

### 3) Test locally
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all --locked

    # Benchmarks (if applicable)
    cargo build --release --features runtime-benchmarks

### 4) Commit
Follow the Conventional Commits spec (see “Commit Messages”):
    git add .
    git commit -m "feat(orderbook): add iceberg order type"

### 5) Push & open PR
    git push origin feature/your-feature-name
Then open a Pull Request on GitHub.

---

## 💻 Coding Standards

### Rust style
• Always format & lint before committing:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
• Avoid `unwrap()` / `expect()` in production paths; use robust error handling  
• Document public APIs with `///` doc comments  
• Keep functions focused; avoid `unsafe` unless necessary and well-documented

### Code quality tips
• Prefer small, composable modules and clear ownership boundaries  
• Use explicit types in public interfaces  
• Add comments for non-obvious invariants and assumptions

### Documentation style (example)
    /// Brief description.
    ///
    /// # Arguments
    /// * `arg1` - ...
    /// * `arg2` - ...
    ///
    /// # Returns
    /// ...
    ///
    /// # Examples
    /// ```
    /// // usage example
    /// ```
    ///
    /// # Panics
    /// ...
    pub fn example_function(arg1: Type1, arg2: Type2) -> ReturnType {
        /* ... */
    }

---

## 📝 Commit Messages

We use **Conventional Commits**.

Format:
    <type>(<scope>): <subject>

    <body>

    <footer>

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`, `ci`

Examples:
    feat(ismp): add token gateway configuration

    Implement Token Gateway pallet configuration for cross-chain asset transfers
    with Hyperbridge integration.

    Closes #123

    fix(evm): correct gas estimation in precompile handler

    Adjust weight calculation for complex transactions.

    Fixes #456

---

## 🔍 Pull Request Process

### Before submitting
1. ✅ All tests pass (`cargo test --all --locked`)  
2. ✅ `cargo fmt` and `cargo clippy` clean  
3. ✅ Docs updated (README or `/docs`)  
4. ✅ Tests added/updated for new logic  
5. ✅ `CHANGELOG.md` updated if user-facing  
6. ✅ Rebased on latest `main`

### PR template (paste into description)
    ## Description
    Brief summary

    ## Type of Change
    - [ ] Bug fix
    - [ ] New feature
    - [ ] Breaking change
    - [ ] Documentation update

    ## Testing
    Commands, scenarios, logs

    ## Checklist
    - [ ] `cargo fmt` / `cargo clippy` clean
    - [ ] Tests added/updated and passing
    - [ ] Docs updated
    - [ ] CHANGELOG updated (if user-facing)
    - [ ] Rebased on latest `main`

### Review
• CI runs (fmt, clippy, tests)  
• ≥1 maintainer approval required  
• Reviewers may test changes on testnet

Target timelines: initial review ≤ **7 days**; follow-ups **3–5 days**; urgent fixes ≤ **48 hours**.

---

## 🧪 Testing Guidelines

### Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn it_works() {
            // Arrange
            // Act
            // Assert
        }
    }

### Integration tests (in `tests/`)
    #[test]
    fn integration_flow() {
        // exercise multiple components together
    }

### Benchmarks
    #[cfg(feature = "runtime-benchmarks")]
    mod benchmarks {
        use super::*;
        use frame_benchmarking::benchmarks;

        benchmarks! {
            my_bench {
                // setup
            }: {
                // dispatch call
            }
            verify {
                // postconditions
            }
        }
    }

---

## 📚 Documentation

Types of docs:
1) Code comments — complex logic/invariants  
2) API docs — doc comments for public items  
3) README — keep high-level docs current  
4) Technical docs — ADRs, architecture, diagrams in `/docs`  
5) Examples — runnable usage examples where helpful

Build local docs:
    cargo doc --open --no-deps

---

## 🤝 Community

Communication channels:
• Issues: https://github.com/fintradev/fintradex/issues  
• Discussions: https://github.com/fintradev/fintradex/discussions  
• X (Twitter): https://x.com/FintraDex  
• Telegram: https://t.me/fintradex

Getting help:
• Search existing issues before creating a new one  
• Include logs, steps to reproduce, and environment details

Ways to contribute:
• 🐛 Report bugs  
• 💡 Propose features  
• 📝 Improve docs  
• 🧪 Add tests  
• 🔍 Review PRs  
• 🌐 Translate docs

---

## 🏆 Recognition
Contributors may be:
• Listed in `CONTRIBUTORS.md`  
• Credited in release notes  
• Mentioned in community updates  
• Eligible for community rewards (post-TGE)

---

## 📄 License
By contributing to FintraDex, you agree your contributions are licensed under the [Apache License 2.0](LICENSE).
