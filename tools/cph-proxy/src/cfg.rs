use anyhow::{anyhow, Result};
use icpc_project_gen::*;
use log::*;
use once_cell::sync::Lazy;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    sync::RwLock,
    thread::JoinHandle,
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
    pub must_self_host: bool,
    running_mode: Option<bool>,
    #[serde(skip)]
    setted_by: ConfigSettedBy,
}

impl Config {
    #[allow(dead_code)]
    fn is_running(&self) -> bool {
        self.running_mode.unwrap_or(false)
    }

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
            must_self_host: false,
            running_mode: None,
            setted_by: ConfigSettedBy::Default,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum ConfigSettedBy {
    SysCfgPath,
    WorkspaceCfgPath,
    #[default]
    Default,
}

lazy_static::lazy_static!(
    static ref  GLOBAL_CFG: RwLock<Config> = RwLock::new(Default::default());
);

fn read_config_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let cfg_file_path = path.as_ref().join("cph-proxy/config.toml");
    if !cfg_file_path.exists() {
        return Err(anyhow!("file not exist"));
    }
    std::fs::read_to_string(cfg_file_path)
        .map_err(|e| anyhow!(e))
        .and_then(|s| toml::from_str(&s).map_err(|e| anyhow!(e)))
}

pub fn init_refresh_global_cfg() -> JoinHandle<()> {
    static SYS_CFG_PATH: Lazy<PathBuf> =
        Lazy::new(|| dirs::config_dir().expect("not found config dir"));
    static WORKSPACE_CFG_PATH: Lazy<std::io::Result<PathBuf>> =
        Lazy::new(|| std::env::current_dir().map(|v| v.join(".config")));

    if read_config_from_file(SYS_CFG_PATH.as_path()).is_err() {
        Config::default()
            .dump()
            .expect("dump default config failed");
    }

    std::thread::spawn(|| {
        let mut hotwatch =
            hotwatch::blocking::Hotwatch::new().expect("hotwatch failed to initialize!");

        hotwatch
            .watch(
                SYS_CFG_PATH.as_path().join("cph-proxy/config.toml"),
                |event: hotwatch::Event| {
                    if let hotwatch::EventKind::Modify(_) = event.kind {
                        match read_config_from_file(SYS_CFG_PATH.as_path()) {
                            Ok(mut cfg) => {
                                let mut global_cfg =
                                    (GLOBAL_CFG.write()).expect("lock global_cfg failed");
                                if ConfigSettedBy::WorkspaceCfgPath.eq(&global_cfg.setted_by) {
                                    return hotwatch::blocking::Flow::Continue;
                                }
                                cfg.setted_by = ConfigSettedBy::SysCfgPath;
                                *global_cfg = cfg;
                            }
                            Err(err) => {
                                error!(
                                    "watch cfg file changed, but read or parse failed!: {}",
                                    err
                                );
                            }
                        }
                    };
                    hotwatch::blocking::Flow::Continue
                },
            )
            .unwrap_or_else(|e| {
                panic!(
                    "watch sys config file failed: {e} in: {}",
                    SYS_CFG_PATH.display()
                )
            });

        if let Ok(workspace_cfg_path) = WORKSPACE_CFG_PATH.as_ref() {
            if workspace_cfg_path.join("cph-proxy/config.toml").exists() {
                hotwatch
                    .watch(
                        workspace_cfg_path.as_path().join("cph-proxy/config.toml"),
                        |event: hotwatch::Event| {
                            match event.kind {
                                hotwatch::EventKind::Modify(_) | hotwatch::EventKind::Create(_) => {
                                    match read_config_from_file(workspace_cfg_path.as_path()) {
                                        Ok(mut cfg) => {
                                            cfg.setted_by = ConfigSettedBy::WorkspaceCfgPath;
                                            let mut global_cfg = (GLOBAL_CFG.write())
                                                .expect("lock global_cfg failed");
                                            *global_cfg = cfg;
                                        }
                                        Err(err) => {
                                            error!(
                                        "watch cfg file changed, but read or parse failed!: {}",
                                        err
                                    );
                                        }
                                    }
                                }
                                _ => {}
                            };
                            hotwatch::blocking::Flow::Continue
                        },
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "watch sys config file failed: {e} in: {}",
                            workspace_cfg_path.display()
                        )
                    });
            }
        }
        hotwatch.run();
    })
}

pub fn get_global_cfg() -> Config {
    GLOBAL_CFG.read().unwrap().clone()
}

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
