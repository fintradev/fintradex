# Security Policy

We take security seriously and welcome good-faith reports. Thank you for helping keep FintraDex users and operators safe. 🛡️

---

## How to Report

**Do NOT open public GitHub issues for vulnerabilities.**

Preferred channels:
- **GitHub “Report a vulnerability”** (Private Vulnerability Reporting), if enabled on this repo.
- **Email:** security@fintradex.io  (PGP available below)

Acknowledgment target: **≤ 48 hours**  
Status update target: **≤ 7 days**

### What to include
- Clear description and **impact**
- Affected components (paths, pallet names, runtime/node version, tag/commit)
- Minimal reproduction steps and any required configuration
- PoC or logs (if available)
- Suggested severity and CVSS vector (optional)

---

## Safe Harbor (Good-Faith Research)

We will not pursue legal action or enforcement against researchers who:

- Make a **good-faith** effort to follow this policy,
- Avoid privacy violations, data destruction, and service disruption,
- Do not access, modify, or transfer funds/secrets,
- Report promptly and do not misuse vulnerabilities,
- Conduct testing on **local/testnet** unless we explicitly authorize mainnet testing.

If unsure whether your approach is permitted, **email security@fintradex.io first**.

---

## Allowed / Not Allowed Testing

**Allowed**
- Local/dev nodes and testnets
- Read-only probing; non-destructive fuzzing
- Using **your own** test accounts and tokens

**Not allowed**
- Impacting **mainnet** funds or user data
- Denial of Service against public RPCs/collators/validators
- Social engineering, phishing, or physical attacks
- Excessive automated scanning that degrades service

---

## Scope

**In scope**
- Parachain runtime & pallets (`/runtime`, `/pallets`)
- Node implementation (`/node`, networking, RPC)
- Cross-chain glue & interfaces (XCM / ISMP / Hyperbridge)
- CI/CD build scripts that produce chain artifacts
- Public endpoints we operate (e.g., RPC/bootnodes), where applicable

**Out of scope**
- Third-party dependencies (please report upstream; notify us if it affects us)
- Issues requiring physical access to user devices
- Social engineering
- DoS findings that do not cause lasting impact
- Purely testnet-only issues **without** plausible mainnet impact (still appreciated; typically triaged lower)

---

## Severity & Target Timelines (CVSS v3.1)

| Severity | CVSS Range | Target Fix Window |
|:-------:|:----------:|:------------------|
| Critical | ≥ 9.0 | 1–7 days |
| High     | 7.0–8.9  | 7–30 days |
| Medium   | 4.0–6.9  | 30–90 days |
| Low      | < 4.0    | Best effort |

We aim to provide a **status update within 7 days** of acknowledgment.

---

## Our Response Process

1. **Acknowledge** receipt (≤ 48h).  
2. **Triage & assess** impact/severity (may request more info).  
3. **Mitigate & fix**; prepare tests and backports if needed.  
4. **Operator coordination (if chain-critical):**  
   - Embargoed notice to collators/validators,  
   - Signed runtime/node release and upgrade guidance,  
   - Rollout and network-health verification.  
5. **Coordinated disclosure** with reporter; publish a GitHub Security Advisory and changelog notes.  
6. **Credit** the reporter (Hall of Fame), unless anonymity is requested.

---

## Coordinated Disclosure

We use a coordinated disclosure model.  
Default embargo for critical issues: **up to 90 days** (shortened/extended based on exploitation risk and fix complexity).

---

## Secure Communication Options

**Primary Contact**: security@fintradex.io

For highly sensitive vulnerabilities, we offer multiple secure communication channels:

### GitHub Security Advisory (Recommended)
Create a private security advisory directly on GitHub:
- **Link**: [Create Private Security Advisory](https://github.com/fintradev/fintradex/security/advisories/new)
- **Benefits**: Structured reporting, private until disclosed, built-in collaboration

### Encrypted Communication
For real-time sensitive discussions:
- **Signal/Telegram**: Contact us via email first to exchange secure messaging details
- **Encrypted Email**: ProtonMail and other encrypted providers accepted

### Response Commitment
- We'll respond within **48 hours** to arrange secure communication if needed
- For critical vulnerabilities, we can set up an immediate secure channel

---

## Security Best Practices for Contributors

- Do **not** commit secrets (keys, passwords, tokens).  
- Run local checks regularly:
  - `cargo fmt` / `cargo clippy -D warnings`
  - `cargo audit` for Rust dependency advisories
  - `gitleaks` to catch secrets in git history
- Request a security review for changes touching:
  - consensus, balances, or asset movement,
  - order-matching/settlement logic,
  - ZK/proof verification,
  - cross-chain logic (XCM/ISMP/Hyperbridge).

---

## Audits

- **Pre-mainnet:** third-party audit planned; report published post-remediation.  
- **Post-mainnet:** periodic audits; targeted reviews for ZK/Risc0 components and cross-chain integrations.

Audit reports will be published publicly after remediation.

---

## Security.txt (recommended)

If you operate **fintradex.io**, consider publishing `/.well-known/security.txt`:

    Contact: mailto:security@fintradex.io
    Policy: https://github.com/fintradev/fintradex/blob/main/SECURITY.md
    Encryption: https://github.com/fintradev/fintradex/raw/main/PGP_PUBLIC_KEY.asc
    Preferred-Languages: en

---

## Contacts

- **Security:** security@fintradex.io  
- **General:** team@fintradex.io  
- **Website:** https://fintradex.io

---