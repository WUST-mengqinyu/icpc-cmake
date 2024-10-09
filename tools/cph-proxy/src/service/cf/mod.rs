use crate::service::ProblemMetaWithTestCaseHandler;
use crate::service::{create_files_if_absent, get_unknown_problem_id};
use crate::{cfg, model::*, service::context};
use anyhow::Context;
use log::*;
use std::sync::Arc;
mod cmake_gen;

pub struct CodeforcesHandler {}
type CodeforcesContext = Arc<context::Context<u32, String>>;

impl CodeforcesHandler {
    fn parse_contest_and_problem_id_from_url(url: &str) -> (u32, String) {
        static CODEFORCES_URL_RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| {
                regex::Regex::new(
                    r"codeforces\.com/(?:problemset/problem|contest)/(\d+)/(?:problem/)?(\w+)",
                )
                .expect("codeforces url regex not correct")
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

    fn get_context(data: &ProblemMetaWithTestCase) -> anyhow::Result<CodeforcesContext> {
        let (contest_id, mut problem_id) = Self::parse_contest_and_problem_id_from_url(&data.url);
        if contest_id == 0 {
            problem_id =
                get_unknown_problem_id(cfg::get_global_cfg().codeforces_project_path.as_path())
                    .with_context(|| {
                        format!("get contest_id 0 and problem_id failed: {}", &data.url)
                    })?
                    .to_string();
        }
        let rt = cfg::get_global_cfg().codeforces_project_path.clone();
        let home_dir = rt.join(contest_id.to_string()).join(&problem_id);
        Ok(Arc::new(context::Context {
            home_dir,
            contest_id,
            problem_id,
        }))
    }
}

#[async_trait::async_trait]
impl ProblemMetaWithTestCaseHandler for CodeforcesHandler {
    fn detecte(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<String> {
        Ok({
            let contest = &Self::get_context(data)?.contest_id;
            format!("cf_{}", contest)
        })
    }
    async fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        let cx = Self::get_context(data).with_context(|| {
            format!("failed to get context from metadata: {}", data.url.as_str())
        })?;
        info!(
            "start write to contest: {}, problem: {}",
            cx.contest_id, cx.problem_id,
        );

        // prepare dir
        std::fs::create_dir_all(cx.home_dir.join("cases")).with_context(|| {
            format!(
                "failed to create all dir in {}",
                cx.home_dir.join("cases").display()
            )
        })?;

        // testcase dump
        create_files_if_absent(
            &data
                .tests
                .iter()
                .enumerate()
                .flat_map(|(i, case)| {
                    [
                        (
                            cx.home_dir.join("cases").join(format!("{}.in", i)),
                            case.input.as_str(),
                        ),
                        (
                            cx.home_dir.join("cases").join(format!("{}.out", i)),
                            case.output.as_str(),
                        ),
                    ]
                })
                .chain(
                    [{
                        let mut metadata = data.clone();
                        metadata.tests = vec![];
                        (
                            cx.home_dir.join("info.toml"),
                            toml::to_string(&metadata)
                                .with_context(|| "metadata tostring failed")?
                                .as_str(),
                        )
                    }]
                    .into_iter(),
                )
                .collect::<Vec<_>>(),
        )
        .await
        .with_context(|| "failed to dump test cases and metadata")?;

        // cmake project prepare
        Self::cmake_gen(cx, data)
            .await
            .with_context(|| "failed to cmake_gen")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_codeforces_contest_and_problem_id() {
        let urls = [
            (
                "https://codeforces.com/problemset/problem/2010/C2",
                2010,
                "C2",
            ),
            ("https://codeforces.com/contest/2010/problem/C2", 2010, "C2"),
        ];

        for (url, contest_id, problem_id) in urls {
            assert_eq!(
                (contest_id, problem_id.to_owned()),
                CodeforcesHandler::parse_contest_and_problem_id_from_url(url)
            );
        }
    }
}
