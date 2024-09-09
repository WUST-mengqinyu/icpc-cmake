use crate::cfg::get_global_cfg;
use anyhow::{Context, Ok, Result};
use fs2::FileExt;
use log::debug;
use std::{
    io::{Read, Write},
    path::Path,
};
use tokio::io::AsyncWriteExt as _;

pub(crate) async fn create_files_if_absent<P: AsRef<Path>, T: AsRef<[u8]>>(
    mp: &[(P, T)],
) -> Result<()> {
    for (path, content) in mp {
        debug!(
            "start to create file if absent in path: {}",
            path.as_ref().display()
        );
        let f = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .await;
        if f.as_ref()
            .is_err_and(|e| e.kind() == std::io::ErrorKind::AlreadyExists)
        {
            debug!(
                "success to create file if absent in path for already_exists: {}",
                path.as_ref().display()
            );
            continue;
        }
        let mut f = f.with_context(|| {
            format!(
                "failed to create if not exist in path: {}",
                path.as_ref().display()
            )
        })?;
        f.write_all(content.as_ref()).await.with_context(|| {
            format!(
                "failed to create if not exist on writing in path: {}",
                path.as_ref().display()
            )
        })?;
        f.sync_all().await?;
        debug!(
            "success to create file if absent in path: {}",
            path.as_ref().display()
        );
    }
    Ok(())
}

pub trait LoopReplace {
    fn replace_loop(self, n: u32) -> String;
}

impl LoopReplace for &str {
    fn replace_loop(self, n: u32) -> String {
        if let Some(start_pos) = self.find("{start_loop}") {
            if let Some(end_pos) = self[start_pos..].find("{end_loop}") {
                let end_pos = start_pos + end_pos;
                let res_m = &self[start_pos + "{start_loop}".len()..end_pos];
                let mut res = self[..start_pos].to_owned();
                for i in 0..n {
                    res.push_str(res_m.replace("{i}", i.to_string().as_str()).as_str());
                }
                res.push_str(&self[end_pos + ("{end_loop}".len()) + 1..]);
                return res;
            }
        }
        self.to_owned()
    }
}

// FIXME: cannot lock and add problem_id correctly
pub(crate) fn get_unknown_problem_id(lock_dir: &Path) -> anyhow::Result<u32> {
    let lockfile = lock_dir.join(".cnt.lock");
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&lockfile)?;
    for _ in 0..get_global_cfg().lock_file_max_try {
        if f.try_lock_exclusive().is_ok() {
            let res = get_unknown_problem_id_within_locked(lockfile.as_path(), &mut f);
            f.unlock()?;
            f.sync_all()?;
            return res;
        }
        std::thread::sleep(std::time::Duration::from_secs(
            get_global_cfg().lock_file_delay_seconds,
        ));
    }
    anyhow::bail!(
        "get lock failed, check lock file in: {}",
        lockfile.display()
    );
}

fn get_unknown_problem_id_within_locked(path: &Path, f: &mut std::fs::File) -> anyhow::Result<u32> {
    let mut buf = [0u8; 4];
    let read_n = f.read(&mut buf)?;
    if read_n != buf.len() {
        anyhow::bail!(
            "lock file check failed, please delete or check it manully: {}",
            path.display(),
        );
    }
    let id = u32::from_be_bytes(buf) + 1;
    f.write_all(&id.to_be_bytes())?;
    Ok(id)
}

lazy_static::lazy_static! {
    static ref RECENT_RUNNING_UUID: tokio::sync::Mutex<String> = tokio::sync::Mutex::new(String::new());
}

pub async fn recreated_ref_in_running<T: AsRef<str>, P: AsRef<Path>>(
    uuid: T,
    src_path: P,
    name: T,
) -> anyhow::Result<()> {
    debug!(
        "start to recreated_ref_in_running in src uuid: {}, path: {}, name: {}",
        uuid.as_ref(),
        src_path.as_ref().display(),
        name.as_ref(),
    );
    let cfg = get_global_cfg();
    if !cfg.is_running() {
        return Ok(());
    }
    if let Some(mode) = cfg.running_mode {
        let mut recent_running_uuid = RECENT_RUNNING_UUID.lock().await;
        if mode.remove_old_linkers && !recent_running_uuid.as_str().ne(uuid.as_ref()) {
            debug!("start to remove old linkers");
            *recent_running_uuid = uuid.as_ref().to_owned();
            remove_old_linkers(
                get_global_cfg()
                    .running_mode
                    .ok_or(anyhow::anyhow!("not in running mode"))?
                    .running_path
                    .as_path(),
            )
            .await?;
            debug!("success to remove old linkers");
        }
        debug!(
            "start to create linkers, src:[ {} ], dest:[ {} ]",
            src_path.as_ref().display(),
            mode.running_path.join(name.as_ref()).display(),
        );
        tokio::fs::symlink(src_path.as_ref(), mode.running_path.join(name.as_ref())).await?;
        debug!(
            "success to create linkers, src:[ {} ], dest:[ {} ]",
            src_path.as_ref().display(),
            mode.running_path.join(name.as_ref()).display(),
        );
    }
    Ok(())
}

async fn remove_old_linkers(running_path: &Path) -> anyhow::Result<()> {
    while let Some(v) = tokio::fs::read_dir(running_path)
        .await?
        .next_entry()
        .await?
    {
        let meta = v.metadata().await.with_context(|| {
            format!(
                "read file metadata in running path failed: {}",
                v.path().display()
            )
        })?;
        if meta.is_symlink() {
            tokio::fs::remove_file(v.path())
                .await
                .with_context(|| format!("remove symlinke file failed: {}", v.path().display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use icpc_project_gen::PROJECT_DIR;

    use crate::{cfg::tests::set_mock_global_cfg, model::tests::mock_problem_meta_with_test_case};

    use super::*;

    // TODO: move it to suite test
    #[tokio::test]
    async fn test_symlink() {
        let mut mock_cfg = crate::cfg::Config::default();
        let mock_running_dir = temp_dir::TempDir::new().unwrap();
        let mock_cf_dir = temp_dir::TempDir::new().unwrap();
        mock_cfg.codeforces_project_path = mock_cf_dir.path().to_owned();
        mock_cfg.running_mode = mock_cfg.running_mode.map(|mut mode| {
            mode.running_path = mock_running_dir.path().to_owned();
            mode
        });
        set_mock_global_cfg(mock_cfg);
        println!("path: {}", mock_running_dir.path().display());
        let _data = mock_problem_meta_with_test_case();
        assert_eq!(
            recreated_ref_in_running(
                "1234",
                Path::new(
                    format!("{}{}", PROJECT_DIR, "/archive/codeforces/2008/A/main.h").as_str()
                ),
                "a.cc",
            )
            .await
            .is_ok(),
            true
        );
        println!("success");
    }
}
