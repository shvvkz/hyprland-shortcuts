#[derive(Debug)]
pub struct Command {
    pub comment: Option<String>,
}

impl Command {
    pub fn from_line(line: &str) -> Option<Self> {
        let replaced = line.replace("bind =", "").replace("bindm =", "");
        let cleaned_line = replaced.trim();

        // Séparation du commentaire
        let (_, comment) = match cleaned_line.split_once('#') {
            Some((_, cmt)) => ("", Some(cmt.trim().to_string())),
            None => ("", None),
        };

        Some(Self { comment })
    }
}


