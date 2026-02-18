# 🛡️ Aethel: Sovereign Security Layer for AI Agents

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Status: Alpha](https://img.shields.io/badge/Status-Alpha-red.svg)](#)

**Aethel** is a local-first hardening substrate designed to rescue autonomous AI agents from the "Lethal Trifecta" of 2026 security risks: Private Data leaks, Untrusted Content execution, and Unauthorized External calls.

Specifically engineered to secure **OpenClaw** instances against the **ClawHavoc** malware campaign and **CVE-2026-25253**.

---

## 🚀 Features

- **The Aethel Gate:** A high-performance Rust sidecar proxy that intercepts all outbound traffic, requiring hardware-bound signatures (WebAuthn) for execution.
- **The Aethel Vault:** Encrypted storage using SQLCipher (AES-256-GCM) to migrate and protect vulnerable plaintext configuration files.
- **Deterministic Logic Gating:** Enforces strict whitelisting of permitted libraries and domains, preventing "Sleeper Agent" skills from phoning home.
- **Sovereign Scorecard:** Real-time attestation of your agent's security posture.

## 🛠️ Installation

### Prerequisites

- **Rust Toolchain** (2024 Edition or later)
- **Python 3.11+** (for legacy migration scripts)
- **A Hardware Security Key** (e.g., YubiKey or TouchID)

### Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/aethel.git
   cd aethel
   ```
