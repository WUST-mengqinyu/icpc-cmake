use std::sync::LazyLock;

use crate::{cfg::get_global_cfg, service::ProblemMetaWithTestCaseHandler};

use super::ProblemMetaWithTestCase;

pub struct VscodeAdapter {
    inner: Box<dyn ProblemMetaWithTestCaseHandler>,
}

impl VscodeAdapter {
    pub fn new(h: Box<dyn ProblemMetaWithTestCaseHandler>) -> Self {
        Self { inner: h }
    }

    // FIXME: use json5? how to keep comments?
    fn replace_running_contest(&self, original: &str, running_env: &str) -> String {
        static RUNNING_CONTEST_SETTING_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(r#""RUNNING_CONTEST":\s*"([^"]*)""#)
                .expect("running contest regex not correct")
        });

        RUNNING_CONTEST_SETTING_RE
            .replace(original, format!(r#""RUNNING_CONTEST": "{}""#, running_env))
            .into_owned()
    }
}

#[async_trait::async_trait]
impl ProblemMetaWithTestCaseHandler for VscodeAdapter {
    async fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        let cfg = get_global_cfg();
        if cfg.in_vscode_project && cfg.running_mode.map_or(false, |x| x.enable) {
            let vscode_settings_file = cfg.project_root.join(".vscode").join("settings.json");
            let content = tokio::fs::read_to_string(vscode_settings_file.as_path()).await?;
            let running_env = self.inner.detecte(data)?;
            let new_content = self.replace_running_contest(&content, &running_env);
            tokio::fs::write(vscode_settings_file.as_path(), new_content).await?;
        }
        self.inner.handle(data).await
    }

    fn detecte(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<String> {
        self.inner.detecte(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_running() {
        let s = r#"{
                "cmake.environment": {
                "GTEST_BRIEF": "1",
                "ENABLE_THIRD_PARTY": "true",
                // "ENABLE_TESTING": "true",
                "ENABLE_BINS": "true",
                "ENABLE_ARCHIVE": "true",
                "RUNNING_CONTEST": "abc374",
            },
        }"#;
        struct MockInner;
        let h = VscodeAdapter { inner: MockInner };
        let new = h.replace_running_contest(&s, "cf1234");
        assert_eq!(
            new.chars()
                .filter(|x| !x.is_whitespace())
                .collect::<String>(),
            r#"{
            "cmake.environment": {
            "GTEST_BRIEF": "1",
            "ENABLE_THIRD_PARTY": "true",
            // "ENABLE_TESTING": "true",
            "ENABLE_BINS": "true",
            "ENABLE_ARCHIVE": "true",
            "RUNNING_CONTEST": "cf1234",
        },
    }"#
            .chars()
            .filter(|x| !x.is_whitespace())
            .collect::<String>()
        );
    }
}
