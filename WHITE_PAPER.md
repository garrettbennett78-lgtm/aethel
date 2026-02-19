# Aethel: The Sovereign Substrate
>
> *Neutralizing the Lethal Trifecta of AI Security Risks*

## 1. Introduction

Project Aethel is a local-first, Rust-hardened security layer designed for AI agents, specifically within the OpenClaw ecosystem. Our mission is to return sovereignty to the user by providing a unified substrate for memory encryption, instruction validation, and egress control.

## 2. The Lethal Trifecta

AI agents currently face three critical vulnerabilities that Project Aethel is built to solve:

1. **Memory Exposure**: Current agents store sensitive session logs, prompt history, and API keys in plaintext (`~/.openclaw`). A single local breach means total identity and data theft.
2. **Unvalidated Logic Gating (RCE)**: LLMs are prone to "Prompt Injection" and "Sleeper Agent" attacks. Without a deterministic gate, an agent can be tricked into executing malicious shell commands (e.g., CVE-2026-25253).
3. **Data Bleeding (Egress Control)**: Agents frequently communicate with external APIs. Without strict whitelisting, sensitive user data can be silently exfiltrated to unauthorized domains via "hidden" tool calls.
4. **Indirect Prompt Injection (IPI)**: Malicious instructions can be hidden in seemingly harmless context (e.g., an agent reading a webpage that contains hidden commands). Aethel must validate all context-derived tool calls.

## 3. Core Architecture

Aethel operates as a high-performance **Rust Zero-Trust Proxy**.

### 3.1 Encapsulated Memory Shards

All session-specific memory is partitioned into encrypted shards. Using **AES-256-GCM** (via SQLCipher), we ensure that even if the host machine is compromised, the "emotional and cognitive core" of the AI remains inaccessible without proper hardware-bound attestation.

### 3.2 Egress Whitelisting

Aethel maintains a strict manifest of authorized domains. Any attempt by an agent to send data to an unmanifested endpoint is instantly blocked, preventing "data bleeding."

### 3.3 **Solution: The Aethel Gate.** By leveraging TPMs and TEEs (specifically macOS Secure Enclave), Aethel ensures that every instruction passed to the system is signed and validated against a known-safe whitelist, neutralizing the risk of arbitrary code execution

## 4. The Mnemosyne Integration

Aethel serves as the technical engine for the **Mnemosyne Portal**. Through ERC-8004 attestation, users can monetize their "AI Memory" assets while maintaining absolute privacy and security through the Aethel layer.

---
*Project Aethel is currently in Beta. For more information, visit beta.mnemosyne.io.*
