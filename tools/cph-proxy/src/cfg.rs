use anyhow::Result;
use icpc_project_gen::*;
use once_cell::sync::Lazy;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub listen_host: (IpAddr, u32),
    pub to_host: Option<(IpAddr, u32)>,
    pub to_host_proxy: Option<(IpAddr, u32)>,
    pub codeforces_project_path: PathBuf,
    pub atcoder_project_path: PathBuf,
    pub others_project_path: PathBuf,
    pub lock_file_delay_seconds: u64,
    pub lock_file_max_try: u64,
}

impl Config {
    fn dump(&self) -> Result<()> {
        let cfg_dir = dirs::config_dir().expect("not found config dir");
        let cfg_file_path = cfg_dir.join("cph-proxy/config.toml");
        std::fs::create_dir_all(cfg_dir.join("cph-proxy"))?;
        Ok(std::fs::write(
            cfg_file_path.clone(),
            toml::to_string(self)?,
        )?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_host: (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 10045),
            to_host: None,
            to_host_proxy: None,
            codeforces_project_path: std::path::PathBuf::from(PROJECT_DIR)
                .join("archive/codeforces"),
            atcoder_project_path: std::path::PathBuf::from(PROJECT_DIR).join("archive/atcoder"),
            others_project_path: std::path::PathBuf::from(PROJECT_DIR).join("archive/others"),
            lock_file_delay_seconds: 2,
            lock_file_max_try: 3,
        }
    }
}

pub static GLOBAL_CFG: Lazy<Config> = Lazy::new(|| {
    let cfg_dir = dirs::config_dir().expect("not found config dir");
    let cfg_file_path = cfg_dir.join("cph-proxy/config.toml");
    if !cfg_file_path.exists() {
        let res = Config::default();
        res.dump().expect("dump default config failed");
        return res;
    }
    std::fs::read_to_string(cfg_file_path.clone()).map_or_else(
        |_| Config::default(),
        |s| {
            toml::from_str(&s).unwrap_or_else(|e| {
                panic!(
                    "parsed config failed: {e}, check your file in: {}",
                    cfg_file_path
                        .as_path()
                        .to_str()
                        .expect("get cfg file path failed")
                )
            })
        },
    )
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cfg = Config::default();
        let format_res = toml::to_string(&cfg);
        assert_eq!(format_res.is_ok(), true);
        println!("{}", &format_res.clone().unwrap());
        let parse_res: Result<Config, toml::de::Error> = toml::from_str(&format_res.unwrap());
        assert_eq!(parse_res.is_ok(), true);
    }
}
