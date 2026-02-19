# Aethel & Mnemosyne: Roadmap 2.0

## Phase 1: Foundation (Current)

* [x] Standardize Repository Structure (Rust Layout).
* [x] Establish Project Philosophy (Whitepaper).
* [ ] Deploy initial Rust Proxy (axum/tokio) with basic logging.
* [ ] Integrate SQLCipher for encrypted "Sovereign Memory" shards.

## Phase 2: Security Hardening (Q2 2026)

* **Instruction Validation**: Implement the first-pass parser for LLM tool call validation.
* **IPI Defense**: Add signature-based validation for context-derived instructions (Indirect Prompt Injection).
* **OS-Integrated Key Management**: Use macOS `security-framework` to bind the Aethel Vault to the Secure Enclave/Keychain.
* **Egress Manifest**: Create the configuration layer for whitelisting domains.
* **Initial Audit**: Open the $500 USDC bounty pool for security researchers on Avalanche C-Chain.

## Phase 3: Monetization & Gating (Q3 2026)

* **ERC-8004 Integration**: Link the Aethel proxy to the Mnemosyne portal for token-gated access.
* **Beta Portal Launch**: Open `beta.mnemosyne.io` to the first 500 Surge Hackathon invitees.
* **Memory Sharding**: Enable decentralized storage of encrypted memory shards.

## Phase 4: The Sovereign Node (2027)

* Transition Aethel from a local-only proxy to a fully decentralized "Trusted Node."
* Scale the bounty ecosystem to include continuous automated red-teaming.
