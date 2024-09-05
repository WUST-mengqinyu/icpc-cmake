mod cmake_gen;
mod context;

use super::model::*;
use context::Context;
use fs2::FileExt;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn store_data(metadata: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
    let platform = CompetitvePlatform::parse_from_url(&metadata.url);
    let real_handler = platform.dispatch();
    tracing::info!("handle platform: {:?}", platform);
    real_handler.as_ref().handle(metadata)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompetitvePlatform {
    Codeforces,
    Atcoder,
    Unknown(String),
}

impl CompetitvePlatform {
    fn parse_from_url(url: &str) -> Self {
        static MAP: std::sync::LazyLock<
            std::collections::HashMap<&'static str, CompetitvePlatform>,
        > = std::sync::LazyLock::new(|| {
            std::collections::HashMap::from([
                ("codeforces.com", CompetitvePlatform::Codeforces),
                ("atcoder.jp", CompetitvePlatform::Atcoder),
            ])
        });

        for (substr, platform) in MAP.iter() {
            if url.contains(substr) {
                return platform.clone();
            }
        }
        static HOST_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            regex::Regex::new(r"^(?:https?:\/\/)?(?:[^@\/\n]+@)?(?:www\.)?([^:\/\n]+)")
                .expect("host regex not correct")
        });
        CompetitvePlatform::Unknown(
            HOST_RE
                .captures(url)
                .map(|c| c.get(1).map_or("unknown_platform", |m| m.as_str()))
                .unwrap_or("unknown_platform")
                .replace(".", "_")
                .to_owned(),
        )
    }

    fn dispatch(&self) -> Box<dyn ProblemMetaWithTestCaseHandler> {
        match self {
            CompetitvePlatform::Codeforces => Box::new(CodeforcesHandler {}),
            CompetitvePlatform::Atcoder => Box::new(AtcoderHandler {}),
            CompetitvePlatform::Unknown(_) => Box::new(DefaultHandler {}),
        }
    }
}

trait ProblemMetaWithTestCaseHandler {
    fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()>;
}

pub struct CodeforcesHandler {}

impl CodeforcesHandler {
    fn parse_contest_and_problem_id_from_url(url: &str) -> (u32, String) {
        static CODEFORCES_URL_RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| {
                regex::Regex::new(r"problem/(\d+)/(\w+)").expect("codeforces url regex not correct")
            });
        CODEFORCES_URL_RE
            .captures(url)
            .map(|c| match (c.get(1), c.get(2)) {
                (Some(contest_id), Some(problem_id)) => (
                    contest_id.as_str().parse().unwrap_or(0),
                    problem_id.as_str().to_owned(),
                ),
                _ => (0, "0".to_owned()),
            })
            .unwrap_or((0, "0".to_owned()))
    }

    fn get_unknown_problem_id(project_dir: PathBuf) -> anyhow::Result<u32> {
        let lockfile = project_dir.join(".cnt.lock");
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&lockfile)?;
        for _ in 0..super::cfg::GLOBAL_CFG.lock_file_max_try {
            if f.try_lock_exclusive().is_ok() {
                let res = Self::get_unknown_problem_id_within_locked(lockfile.as_path(), &mut f);
                f.unlock()?;
                f.sync_all()?;
                return res;
            }
            std::thread::sleep(std::time::Duration::from_secs(
                super::cfg::GLOBAL_CFG.lock_file_delay_seconds,
            ));
        }
        anyhow::bail!(
            "get lock failed, check lock file in: {}",
            lockfile.display()
        );
    }

    fn get_unknown_problem_id_within_locked(
        path: &Path,
        f: &mut std::fs::File,
    ) -> anyhow::Result<u32> {
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

    fn get_context(data: &ProblemMetaWithTestCase) -> anyhow::Result<context::CodeforcesContext> {
        let (contest_id, mut problem_id) = Self::parse_contest_and_problem_id_from_url(&data.url);
        if contest_id == 0 {
            problem_id = Self::get_unknown_problem_id(
                super::cfg::GLOBAL_CFG.codeforces_project_path.clone(),
            )?
            .to_string();
        }
        let rt = super::cfg::GLOBAL_CFG.codeforces_project_path.clone();
        let home_dir = rt.join(contest_id.to_string()).join(&problem_id);
        Ok(Arc::new(Context {
            home_dir,
            contest_id,
            problem_id,
        }))
    }
}

impl ProblemMetaWithTestCaseHandler for CodeforcesHandler {
    fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        let cx = Self::get_context(data)?;
        tracing::info!(
            "start write to contest: {}, problem: {}",
            cx.contest_id,
            cx.problem_id,
        );
        {}
        std::fs::create_dir_all(cx.home_dir.join("cases"))?;
        for (i, test) in data.tests.iter().enumerate() {
            let path = cx.home_dir.clone();
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path.join("cases").join(format!("{}.in", i)))?;
            f.write_all(test.input.as_bytes())?;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path.join("cases").join(format!("{}.out", i)))?;
            f.write_all(test.output.as_bytes())?;
        }
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(cx.home_dir.join("info.toml"))?;
        let mut metadata = data.clone();
        metadata.tests = vec![];
        f.write_all(toml::to_string(&metadata)?.as_bytes())?;

        cmake_gen::cmake_gen(cx, data)?;
        Ok(())
    }
}

pub struct AtcoderHandler {}
impl ProblemMetaWithTestCaseHandler for AtcoderHandler {
    fn handle(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct DefaultHandler {}
impl ProblemMetaWithTestCaseHandler for DefaultHandler {
    fn handle(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_url() {
        let url = "https://codeforces.com/contest/1";
        let platform = CompetitvePlatform::parse_from_url(url);
        assert_eq!(platform, CompetitvePlatform::Codeforces);

        let url = "https://atcoder.jp/contest/2";
        let platform = CompetitvePlatform::parse_from_url(url);
        assert_eq!(platform, CompetitvePlatform::Atcoder);

        let url = "https://leetcode.com/contest/3";
        let platform = CompetitvePlatform::parse_from_url(url);
        assert_eq!(
            platform,
            CompetitvePlatform::Unknown("leetcode_com".to_owned())
        );

        let url = "https://unknown.com";
        let platform = CompetitvePlatform::parse_from_url(url);
        assert_eq!(
            platform,
            CompetitvePlatform::Unknown("unknown_com".to_owned())
        );

        let url = "not_a_url";
        let platform = CompetitvePlatform::parse_from_url(url);
        assert_eq!(
            platform,
            CompetitvePlatform::Unknown("not_a_url".to_owned())
        );
    }
}
