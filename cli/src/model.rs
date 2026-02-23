use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Inbox,
    Kept,
    Archived,
    Deleted,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Inbox => write!(f, "inbox"),
            Status::Kept => write!(f, "kept"),
            Status::Archived => write!(f, "archived"),
            Status::Deleted => write!(f, "deleted"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inbox" => Ok(Status::Inbox),
            "kept" => Ok(Status::Kept),
            "archived" => Ok(Status::Archived),
            "deleted" => Ok(Status::Deleted),
            _ => Err(format!("unknown status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct Flicker {
    pub meta: Frontmatter,
    pub body: String,
}

impl Flicker {
    pub fn to_file_content(&self) -> String {
        let yaml = serde_yaml::to_string(&self.meta).unwrap();
        format!("---\n{}---\n\n{}", yaml, self.body)
    }

    pub fn from_file_content(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = content.trim_start_matches('\n');
        if !content.starts_with("---\n") {
            return Err("missing frontmatter".into());
        }
        let rest = &content[4..];
        let end = rest.find("\n---\n").ok_or("missing frontmatter end")?;
        let yaml = &rest[..end];
        let body = rest[end + 5..].trim_start_matches('\n').to_string();
        let meta: Frontmatter = serde_yaml::from_str(yaml)?;
        Ok(Flicker { meta, body })
    }
}
