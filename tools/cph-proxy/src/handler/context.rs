use std::{fmt::Debug, sync::Arc};

pub type CodeforcesContext = Arc<Context<u32, String>>;

#[derive(Debug, Default)]
pub struct Context<ContestID: Debug, ProblemID: Debug> {
    pub home_dir: std::path::PathBuf,
    pub contest_id: ContestID,
    pub problem_id: ProblemID,
}
