use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Command {
    pub main_key: Option<String>,
    pub key: String,
    pub action: String,
    pub arguments: Option<String>,
    pub comment: Option<String>,
}

impl Command {
    pub fn from_line(line: &str) -> Option<Self> {
        let line = line.replace("bind =", "").replace("bindm =", "");
        let cleaned_line = line.trim();

        // Séparation du commentaire
        let (command_part, comment) = match cleaned_line.split_once('#') {
            Some((cmd, cmt)) => (cmd.trim(), Some(cmt.trim().to_string())),
            None => (cleaned_line, None),
        };

        let parts: Vec<&str> = command_part.split(',').map(|s| s.trim()).collect();
        if parts.len() < 3 {
            return None;
        }

        let main_key = if parts[0].is_empty() {
            None
        } else {
            Some(parts[0].to_string())
        };

        let key = parts[1].to_string();
        let action = parts[2].to_string();

        let arguments = if parts.len() > 3 {
            Some(parts[3..].join(", ").trim().to_string())
        } else {
            None
        };

        Some(Self {
            main_key,
            key,
            action,
            arguments,
            comment,
        })
    }
}
