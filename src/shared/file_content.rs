use std::fs::read_to_string;

pub struct FileContent(pub String);

impl std::str::FromStr for FileContent {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Somewhat of a hack, but saves effort:
        let file_content = read_to_string(s)?;
        Ok(FileContent(file_content))
    }
}

impl std::ops::Deref for FileContent {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for FileContent {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl std::fmt::Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}