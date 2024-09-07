use crate::cfg;
use crate::handler::context;
use crate::handler::ProblemMetaWithTestCaseHandler;
use crate::handler::{create_files_if_absent, get_unknown_problem_id};
use crate::model::*;
use anyhow::Context as _;
use log::*;
use std::sync::Arc;
mod cmake_gen;

pub struct AtcoderHandler {}

type AtcoderContext = Arc<context::Context<(AtcoderContestType, u32), String>>;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
enum AtcoderContestType {
    ABC,
    ARC,
    AGC,
    Unkonwn,
}

impl std::fmt::Display for AtcoderContestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AtcoderContestType::ABC => "abc",
            AtcoderContestType::ARC => "arc",
            AtcoderContestType::AGC => "arc",
            _ => "unknown",
        })
    }
}

impl From<&str> for AtcoderContestType {
    fn from(s: &str) -> Self {
        match s {
            "abc" => Self::ABC,
            "arc" => Self::ARC,
            "agc" => Self::AGC,
            _ => Self::Unkonwn,
        }
    }
}

impl AtcoderHandler {
    fn parse_contest_and_problem_id_from_url(url: &str) -> (AtcoderContestType, u32, String) {
        static ATCODER_URL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            regex::Regex::new(r"atcoder\.jp/contests/(abc|arc|agc)(\w+)/tasks/\w+_(\w+)")
                .expect("atcoder url regex not correct")
        });
        ATCODER_URL_RE
            .captures(url)
            .map(|c| match (c.get(1), c.get(2), c.get(3)) {
                (Some(contest_type), Some(contest_id), Some(problem_id)) => (
                    AtcoderContestType::from(contest_type.as_str()),
                    contest_id.as_str().parse().unwrap_or(0),
                    problem_id.as_str().to_owned(),
                ),
                _ => (AtcoderContestType::Unkonwn, 0, "0".to_owned()),
            })
            .unwrap_or((AtcoderContestType::Unkonwn, 0, "0".to_owned()))
    }

    fn get_context(data: &ProblemMetaWithTestCase) -> anyhow::Result<AtcoderContext> {
        let (contest_type, contest_id, mut problem_id) =
            Self::parse_contest_and_problem_id_from_url(&data.url);
        if contest_type == AtcoderContestType::Unkonwn {
            problem_id = get_unknown_problem_id(&cfg::get_global_cfg().atcoder_project_path)
                .with_context(|| {
                    format!(
                        "get contest_type unknown and problem_id failed: {}",
                        &data.url
                    )
                })?
                .to_string();
        }
        let mut home_dir = cfg::get_global_cfg()
            .atcoder_project_path
            .join(contest_type.to_string());
        if contest_type != AtcoderContestType::Unkonwn {
            home_dir.push(contest_id.to_string());
        }
        Ok(Arc::new(context::Context {
            home_dir: home_dir.join(&problem_id),
            contest_id: (contest_type, contest_id),
            problem_id,
        }))
    }
}

impl ProblemMetaWithTestCaseHandler for AtcoderHandler {
    fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        let cx = Self::get_context(data).with_context(|| {
            format!("failed to get context from metadata: {}", data.url.as_str())
        })?;
        info!(
            "start write to contest: {}{}, problem: {}",
            cx.contest_id.0, cx.contest_id.1, cx.problem_id,
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
        .with_context(|| "failed to dump test cases and metadata")?;

        // cmake project prepare
        Self::cmake_gen(cx, data).with_context(|| "failed to cmake_gen")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_url() {
        assert_eq!(
            (AtcoderContestType::ABC, 369, "a".to_owned()),
            AtcoderHandler::parse_contest_and_problem_id_from_url(
                "https://atcoder.jp/contests/abc369/tasks/abc369_a"
            )
        );
        assert_eq!(
            (AtcoderContestType::AGC, 123, "b".to_owned()),
            AtcoderHandler::parse_contest_and_problem_id_from_url(
                "https://atcoder.jp/contests/agc123/tasks/agc123_b"
            )
        );
    }
}
