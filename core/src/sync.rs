use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::config;
use crate::model::{Flicker, Frontmatter, Status};
use crate::storage;

#[derive(Serialize, Deserialize)]
struct FlickerRow {
    id: String,
    created_at: String,
    updated_at: String,
    source: String,
    audio_file: Option<String>,
    status: String,
    body: String,
}

impl FlickerRow {
    fn from_flicker(f: &Flicker) -> Self {
        FlickerRow {
            id: f.meta.id.clone(),
            created_at: f.meta.created_at.to_rfc3339(),
            updated_at: f.meta.updated_at.to_rfc3339(),
            source: f.meta.source.clone(),
            audio_file: f.meta.audio_file.clone(),
            status: f.meta.status.to_string(),
            body: f.body.clone(),
        }
    }

    fn to_flicker(&self) -> Option<Flicker> {
        let created_at = self.created_at.parse::<DateTime<Utc>>().ok()?;
        let updated_at = self.updated_at.parse::<DateTime<Utc>>().ok()?;
        let status: Status = self.status.parse().ok()?;
        Some(Flicker {
            meta: Frontmatter {
                id: self.id.clone(),
                created_at,
                updated_at,
                source: self.source.clone(),
                audio_file: self.audio_file.clone(),
                status,
            },
            body: self.body.clone(),
        })
    }
}

pub struct SyncClient {
    client: reqwest::blocking::Client,
    base_url: String,
    anon_key: String,
}

impl SyncClient {
    pub fn new(base_url: &str, anon_key: &str) -> Self {
        SyncClient {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            anon_key: anon_key.to_string(),
        }
    }

    pub fn from_config() -> Option<Self> {
        let cfg = config::load();
        let url = cfg.supabase_url?;
        let key = cfg.supabase_anon_key?;
        if url.is_empty() || key.is_empty() {
            return None;
        }
        Some(Self::new(&url, &key))
    }

    fn headers(&self) -> Vec<(&str, String)> {
        vec![
            ("apikey", self.anon_key.clone()),
            ("Authorization", format!("Bearer {}", self.anon_key)),
        ]
    }

    /// Pull remote changes newer than `since`, write to local storage.
    /// Returns number of flickers pulled.
    fn pull(&self, since: Option<DateTime<Utc>>) -> Result<usize, String> {
        let mut url = format!("{}/rest/v1/flickers?select=*", self.base_url);
        if let Some(ts) = since {
            url.push_str(&format!("&updated_at=gt.{}", ts.to_rfc3339().replace("+", "%2B")));
        }

        let mut req = self.client.get(&url);
        for (k, v) in self.headers() {
            req = req.header(k, v);
        }

        let resp = req.send().map_err(|e| format!("pull request failed: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!("pull failed: HTTP {}", resp.status()));
        }

        let rows: Vec<FlickerRow> = resp.json().map_err(|e| format!("pull parse failed: {e}"))?;
        let mut count = 0;

        for row in &rows {
            let Some(remote) = row.to_flicker() else { continue };
            let local = storage::read_one(&remote.meta.id);

            let should_write = match &local {
                None => true,
                Some(l) => remote.meta.updated_at > l.meta.updated_at,
            };

            if should_write {
                // Write directly without stamping updated_at (preserve remote timestamp)
                let dir = storage::flickers_dir();
                std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir failed: {e}"))?;
                std::fs::write(
                    dir.join(format!("{}.md", remote.meta.id)),
                    remote.to_file_content(),
                )
                .map_err(|e| format!("write failed: {e}"))?;
                count += 1;

            }
        }

        Ok(count)
    }

    /// Push local changes newer than `since` to remote.
    /// Returns number of flickers pushed.
    fn push(&self, since: Option<DateTime<Utc>>) -> Result<usize, String> {
        let all = storage::read_all();
        let to_push: Vec<&Flicker> = match since {
            Some(ts) => all.iter()
                .filter(|f| f.meta.updated_at > ts)
                .filter(|f| f.meta.status != Status::Deleted)
                .collect(),
            None => all.iter()
                .filter(|f| f.meta.status != Status::Deleted)
                .collect(),
        };

        if to_push.is_empty() {
            return Ok(0);
        }

        let url = format!("{}/rest/v1/flickers", self.base_url);
        let mut count = 0;

        for flicker in to_push {
            let row = FlickerRow::from_flicker(flicker);
            let body = serde_json::to_string(&row).map_err(|e| format!("serialize failed: {e}"))?;

            let mut req = self.client.post(&url)
                .body(body)
                .header("Content-Type", "application/json")
                .header("Prefer", "resolution=merge-duplicates");

            for (k, v) in self.headers() {
                req = req.header(k, v);
            }

            let resp = req.send().map_err(|e| format!("push request failed: {e}"))?;
            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().unwrap_or_default();
                return Err(format!("push failed for {}: HTTP {} — {}", flicker.meta.id, status, body));
            }
            count += 1;
        }

        Ok(count)
    }

    /// Full pull-then-push sync cycle.
    pub fn sync(&self) -> Result<(usize, usize), String> {
        let last_synced = crate::sync_state::load_last_synced();

        let pulled = self.pull(last_synced)?;
        let pushed = self.push(last_synced)?;

        crate::sync_state::save_last_synced(Utc::now());

        Ok((pulled, pushed))
    }
}
