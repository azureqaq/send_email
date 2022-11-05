use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    fmt::Display,
    fs::{read_to_string, File},
    path::Path,
};
use toml;

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    name: Option<String>,
    email: String,
    pwd: String,
}

impl UserConfig {
    pub fn new(name: Option<String>, email: String, pwd: String) -> Self {
        Self { email, pwd, name }
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn pwd(&self) -> &str {
        &self.pwd
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            name: Some("tnicl242".into()),
            email: "default_email".into(),
            pwd: "default_pwd".into(),
        }
    }
}

impl Display for UserConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sender[{}-{}]",
            match self.name() {
                Some(s) => s,
                None => "None",
            },
            self.email()
        )
    }
}

pub fn get_config<P>(path: P) -> Result<UserConfig>
where
    P: AsRef<Path>,
{
    // let f = File::open(path)?;
    let Ok(content) = read_to_string(&path) else {
        // 写入默认
        let userc = UserConfig::default();
        let con = toml::to_string_pretty(&userc)?;
        let mut f = File::create(&path)?;
        write!(f, "{}", con)?;
        return Err(anyhow!("写入默认配置: {}", path.as_ref().display()));
    };
    let res = toml::from_str(&content)?;
    Ok(res)
}
