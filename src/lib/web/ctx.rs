use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}


#[derive(Debug, Serialize)]
pub struct Home{}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for Home {
    fn title(&self) -> &str {
        "Zingat - pastebin in rs"
    }
    fn template_path(&self) -> &str {
        "home"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::ShortCode,
    
}

impl PageContext for PasswordRequired {
    fn title(&self) -> &str {
        "Password Required"
    }
    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn parent(&self) -> &str {
        "base"
    }
}
