mod atc;
mod cf;
mod config;
pub mod context;
pub mod default;

use super::model::*;
pub use config::*;
pub use default::common_utils::*;
use log::*;

pub async fn store_data(metadata: ProblemMetaWithTestCase) -> anyhow::Result<()> {
    let platform = CompetitvePlatform::parse_from_url(&metadata.url);
    let real_handler = platform.dispatch();
    info!("handle platform: {:?}", platform);
    real_handler.handle(&metadata).await
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

    fn dispatch(&self) -> &dyn ProblemMetaWithTestCaseHandler {
        match self {
            CompetitvePlatform::Codeforces => &cf::CodeforcesHandler {},
            CompetitvePlatform::Atcoder => &atc::AtcoderHandler {},
            CompetitvePlatform::Unknown(_) => &default::DefaultHandler {},
        }
    }
}

#[async_trait::async_trait]
trait ProblemMetaWithTestCaseHandler: Send + Sync {
    async fn handle(&self, data: &ProblemMetaWithTestCase) -> anyhow::Result<()>;
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
