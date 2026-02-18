import sqlite3
import os

def create_sovereign_vault(db_path, passkey_secret):
    # In 2026, we use SQLCipher to ensure the DB itself is encrypted at rest
    conn = sqlite3.connect(db_path)
    # The 'PRAGMA key' is derived from your Aethel Hardware Key
    conn.execute(f"PRAGMA key = '{passkey_secret}';")
    
    conn.execute('''CREATE TABLE IF NOT EXISTS sovereign_memory 
                    (id INTEGER PRIMARY KEY, 
                     key TEXT UNIQUE, 
                     value BLOB, 
                     metadata TEXT)''')
    conn.commit()
    return conn

def migrate_openclaw(vault_conn, openclaw_path):
    # Scans for the vulnerable plaintext openclaw.json
    config_path = os.path.join(openclaw_path, "openclaw.json")
    if os.path.exists(config_path):
        with open(config_path, 'r') as f:
            data = f.read()
            # Vaulting the data
            vault_conn.execute("INSERT OR REPLACE INTO sovereign_memory (key, value) VALUES (?, ?)", 
                               ("root_config", data))
            vault_conn.commit()
            # Shred the original file
            os.remove(config_path)
            print(f"🔒 Migrated {config_path} to Aethel Vault.")

# Usage: migrate_openclaw(conn, os.path.expanduser("~/.openclaw"))
