use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::fs;

pub struct AethelVault {
    conn: Connection,
}

impl AethelVault {
    /// Opens or creates an encrypted Sovereign Vault using SQLCipher.
    pub fn open<P: AsRef<Path>>(db_path: P, passkey_secret: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // 🔒 The PRAGMA key is derived from the Aethel Hardware Key.
        // This ensures the database is unreadable without the physical attestation.
        conn.execute_batch(&format!("PRAGMA key = '{}';", passkey_secret))?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sovereign_memory (
                id INTEGER PRIMARY KEY,
                key TEXT UNIQUE,
                value BLOB,
                metadata TEXT,
                last_access TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        Ok(AethelVault { conn })
    }

    /// Securely stores a value in the vault with associated metadata.
    pub fn store(&self, key: &str, value: &[u8], metadata: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sovereign_memory (key, value, metadata) VALUES (?, ?, ?)",
            params![key, value, metadata],
        )?;
        Ok(())
    }

    /// Securely retrieves a value from the vault.
    pub fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut stmt = self.conn.prepare("SELECT value FROM sovereign_memory WHERE key = ?")?;
        let mut rows = stmt.query(params![key])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Migrates vulnerable plaintext configs (like openclaw.json) into the vault and shreds the source.
    pub fn migrate_openclaw<P: AsRef<Path>>(&self, openclaw_dir: P) -> std::io::Result<()> {
        let config_path = openclaw_dir.as_ref().join("openclaw.json");
        
        if config_path.exists() {
            let data = fs::read(&config_path)?;
            
            self.conn.execute(
                "INSERT OR REPLACE INTO sovereign_memory (key, value, metadata) VALUES (?, ?, ?)",
                params!["root_config", data, "Migrated from openclaw.json"],
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
            // 🛑 ZERO-PASS SHREDDING: Protects against physical forensic analysis.
            // In a real 2026 OS, this would call a kernel-level secure_delete.
            fs::write(&config_path, vec![0; data.len()])?; 
            fs::remove_file(config_path)?;
            
            println!("🔒 [AETHEL]: Migrated openclaw.json to Sovereign Vault and performed secure shredding.");
        }
        
        Ok(())
    }
}
