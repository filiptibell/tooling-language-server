use std::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ManifestToolSpecError {
    #[error("missing tool author")]
    MissingAuthor,
    #[error("missing tool name")]
    MissingName,
    #[error("missing tool version")]
    MissingVersion,
    #[error("tool author contains invalid character '{0}'")]
    InvalidAuthor(char),
    #[error("tool name contains invalid character '{0}'")]
    InvalidName(char),
    #[error("tool version contains invalid character '{0}'")]
    InvalidVersion(char),
}

#[derive(Debug, Default, Clone)]
pub struct ManifestToolSpec {
    pub author: String,
    pub name: String,
    pub version: String,
}

impl fmt::Display for ManifestToolSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}@{}", self.author, self.name, self.version)
    }
}

impl FromStr for ManifestToolSpec {
    type Err = ManifestToolSpecError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx_slash = s.chars().enumerate().find(|(_, c)| c == &'/');
        if idx_slash.is_none() {
            validate_author(s)?;
            return Err(ManifestToolSpecError::MissingAuthor);
        }

        let idx_slash = idx_slash.unwrap().0;
        let tool_author = validate_author(&s[..idx_slash])?;

        let idx_at = s.chars().enumerate().find(|(_, c)| c == &'@');
        if idx_at.is_none() {
            if idx_slash > s.len() - 1 {
                validate_name(&s[idx_slash + 1..])?;
            }
            return Err(ManifestToolSpecError::MissingName);
        }

        let idx_at = idx_at.unwrap().0;
        let tool_name = validate_name(&s[idx_slash + 1..idx_at])?;

        if idx_at == s.len() - 1 {
            return Err(ManifestToolSpecError::MissingVersion);
        }

        let tool_version = validate_version(&s[idx_at + 1..])?;

        Ok(Self {
            author: tool_author,
            name: tool_name,
            version: tool_version,
        })
    }
}

fn is_valid_naming_char(char: char) -> bool {
    char == '-' || char == '_' || char.is_ascii_alphanumeric()
}

fn is_valid_version_char(char: char) -> bool {
    char == '.' || char == '-' || char == '_' || char.is_ascii_alphanumeric()
}

fn validate_author(author: impl AsRef<str>) -> Result<String, ManifestToolSpecError> {
    let author = author.as_ref();
    if let Some(invalid_char) = author.chars().find(|c| !is_valid_naming_char(*c)) {
        Err(ManifestToolSpecError::InvalidAuthor(invalid_char))
    } else {
        Ok(author.to_string())
    }
}

fn validate_name(name: impl AsRef<str>) -> Result<String, ManifestToolSpecError> {
    let name = name.as_ref();
    if let Some(invalid_char) = name.chars().find(|c| !is_valid_naming_char(*c)) {
        Err(ManifestToolSpecError::InvalidName(invalid_char))
    } else {
        Ok(name.to_string())
    }
}

fn validate_version(version: impl AsRef<str>) -> Result<String, ManifestToolSpecError> {
    let version = version.as_ref();
    if let Some(invalid_char) = version.chars().find(|c| !is_valid_version_char(*c)) {
        Err(ManifestToolSpecError::InvalidVersion(invalid_char))
    } else {
        // TODO: Verify semver version
        Ok(version.to_string())
    }
}