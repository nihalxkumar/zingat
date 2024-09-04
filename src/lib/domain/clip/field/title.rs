use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if title.trim().is_empty() {
                    Self(None)
                } else {
                    Self(Some(title))
                }
            }
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
