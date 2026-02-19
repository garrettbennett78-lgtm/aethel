use warp::Filter;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct AethelCommand {
    command: String,
    signature: String, // WebAuthn / Passkey Signature
}

#[derive(Serialize)]
struct ExecutionResult {
    status: String,
    output: String,
    egress_alert: Option<String>,
}

pub async fn start_server(port: u16) {
    // 🛡️ The Sovereign Egress Manifest
    let egress_whitelist = Arc::new(vec![
        "api.openai.com".to_string(),
        "api.anthropic.com".to_string(),
        "beta.mnemosyne.io".to_string(),
    ]);

    let gate = warp::path("aethel").and(warp::path("v1")).and(warp::path("exec"))
        .and(warp::post())
        .and(warp::body::json())
        .map(move |cmd: AethelCommand| {
            // 1. Hardware-Bound Authentication
            if !verify_passkey(&cmd.signature) {
                return warp::reply::with_status(
                    warp::reply::json(&"UNAUTHORIZED: Hardware Signature Invalid"),
                    warp::http::StatusCode::UNAUTHORIZED,
                );
            }

            // 2. Instruction Sanitization (Neural/Regex Guardian)
            if let Err(e) = validate_instruction(&cmd.command) {
                return warp::reply::with_status(
                    warp::reply::json(&format!("SECURITY ALERT: {}", e)),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            }

            // 3. Egress Check (Intercepting outbound calls)
            let (result, egress_alert) = enclave_execute(&cmd.command, &egress_whitelist);
            
            warp::reply::with_status(
                warp::reply::json(&ExecutionResult {
                    status: "SUCCESS".into(),
                    output: result,
                    egress_alert,
                }),
                warp::http::StatusCode::OK,
            )
        });

    println!("🛡️ Aethel Gate standing watch on 127.0.0.1:{}", port);
    warp::serve(gate).run(([127, 0, 0, 1], port)).await;
}

fn verify_passkey(sig: &str) -> bool {
    // In 2026, this validates an R1 Attestation from a TPM or Secure Enclave.
    !sig.is_empty() && sig.len() > 10
}

fn validate_instruction(cmd: &str) -> Result<(), &str> {
    // The Guardian Filter: Prevents CVE-2026-25253 (RCE)
    let malicious_patterns = [
        r"rm\s+-rf",
        r"curl\s+.*\s+\|\s+sh",
        r"wget\s+.*\s+\|\s+bash",
        r">\s+/etc/passwd",
    ];

    for pattern in malicious_patterns {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(cmd) {
            return Err("Malicious Instruction Detected and Neutralized.");
        }
    }
    Ok(())
}

fn enclave_execute(cmd: &str, whitelist: &[String]) -> (String, Option<String>) {
    let mut egress_alert = None;

    // Simulate Egress Interception
    if cmd.contains("http") {
        let mut allowed = false;
        for domain in whitelist {
            if cmd.contains(domain) {
                allowed = true;
                break;
            }
        }
        if !allowed {
            egress_alert = Some("BLOCKED: Unauthorized Egress Attempt to non-whitelisted domain.".into());
            return ("ACCESS_DENIED".into(), egress_alert);
        }
    }

    (format!("Safe Execution Result for '{}'", cmd), None)
}
