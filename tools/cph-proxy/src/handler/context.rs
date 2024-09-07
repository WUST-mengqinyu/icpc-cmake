use std::fmt::Debug;


#[derive(Debug)]
pub struct Context<ContestID: Debug, ProblemID: Debug> {
    pub home_dir: std::path::PathBuf,
    pub contest_id: ContestID,
    pub problem_id: ProblemID,
}
