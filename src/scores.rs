use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt, io};
#[cfg(not(feature = "vhs"))]
use std::{env, fs, path::Path, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Initials {
    chars: [char; 6],
    len: usize,
}

impl Default for Initials {
    fn default() -> Self {
        Self::new()
    }
}

impl Initials {
    pub const MAX_LEN: usize = 6;

    pub fn new() -> Self {
        Self {
            chars: [' '; Self::MAX_LEN],
            len: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut initials = Self::new();
        for c in s.chars().take(Self::MAX_LEN) {
            initials.push(c);
        }
        initials
    }

    pub fn push(&mut self, c: char) -> bool {
        if self.len < Self::MAX_LEN && (c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            self.chars[self.len] = c.to_ascii_uppercase();
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> bool {
        if self.len > 0 {
            self.len -= 1;
            self.chars[self.len] = ' ';
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_chars(&self) -> &[char] {
        &self.chars[..self.len]
    }
}

impl fmt::Display for Initials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &c in &self.chars[..self.len] {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Serialize for Initials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Initials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub initials: Initials,
    pub score: usize,
    pub lines: usize,
    pub level: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HighScores {
    pub modes: HashMap<String, Vec<ScoreEntry>>,
}

impl HighScores {
    pub const TOP_LIMIT: usize = 5;

    #[cfg(not(feature = "vhs"))]
    pub fn storage_path() -> PathBuf {
        if let Ok(appdata) = env::var("APPDATA") {
            Path::new(&appdata).join("tetrus").join("scores.json")
        } else if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
            Path::new(&xdg_data).join("tetrus").join("scores.json")
        } else if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            Path::new(&home)
                .join(".local")
                .join("share")
                .join("tetrus")
                .join("scores.json")
        } else {
            PathBuf::from("scores.json")
        }
    }

    #[cfg(not(feature = "vhs"))]
    pub fn load() -> Self {
        let path = Self::storage_path();
        if let Ok(content) = fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    #[cfg(feature = "vhs")]
    pub fn load() -> Self {
        Self::fake_vhs_data()
    }

    #[cfg(feature = "vhs")]
    pub fn fake_vhs_data() -> Self {
        let mut modes = HashMap::new();
        modes.insert(
            "endless".to_string(),
            vec![
                ScoreEntry {
                    initials: Initials::from_str("CZSMIL"),
                    score: 4_300,
                    lines: 48,
                    level: 5,
                },
                ScoreEntry {
                    initials: Initials::from_str("DOREMY"),
                    score: 3_120,
                    lines: 32,
                    level: 4,
                },
                ScoreEntry {
                    initials: Initials::from_str("PROZ"),
                    score: 1_000,
                    lines: 22,
                    level: 3,
                },
                ScoreEntry {
                    initials: Initials::from_str("GARBO"),
                    score: 200,
                    lines: 15,
                    level: 2,
                },
            ],
        );
        Self { modes }
    }

    #[cfg(not(feature = "vhs"))]
    pub fn save(&self) -> io::Result<()> {
        let path = Self::storage_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    #[cfg(feature = "vhs")]
    pub fn save(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn check_qualification(&self, mode: &str, score: usize) -> Option<usize> {
        if score == 0 {
            return None;
        }

        let entries = self.modes.get(mode);
        match entries {
            None => Some(1),
            Some(list) => {
                if list.is_empty() {
                    Some(1)
                } else if list.len() < Self::TOP_LIMIT {
                    let rank = list.iter().filter(|e| e.score > score).count() + 1;
                    Some(rank)
                } else if score > list.last().map(|e| e.score).unwrap_or(0) {
                    let rank = list.iter().filter(|e| e.score > score).count() + 1;
                    Some(rank)
                } else {
                    None
                }
            }
        }
    }

    pub fn insert(&mut self, mode: &str, entry: ScoreEntry) -> usize {
        let entries = self.modes.entry(mode.to_string()).or_default();
        let target_entry = entry.clone();
        entries.push(entry);
        entries.sort_by(|a, b| b.score.cmp(&a.score));

        let rank = entries
            .iter()
            .position(|e| e == &target_entry)
            .map(|pos| pos + 1)
            .unwrap_or(1);

        entries.truncate(Self::TOP_LIMIT);
        let _ = self.save();
        rank
    }

    pub fn get_top_5(&self, mode: &str) -> &[ScoreEntry] {
        self.modes.get(mode).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
