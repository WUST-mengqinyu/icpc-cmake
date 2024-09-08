mod cf;
mod atc;

use crate::handler::context;
use crate::model::*;
use anyhow::Result;

fn cmake_gen<T: std::fmt::Debug, P: std::fmt::Debug>(cx: context::Context<T, P>, data: &ProblemMetaWithTestCase) -> Result<()> {
    match cx.platform {
        super::CompetitvePlatform::Codeforces => cf::cmake_gen(cx, data),
        super::CompetitvePlatform::Atcoder => atc::make_gen(cx, data),
        super::CompetitvePlatform::Unknown(_) => todo!(),
    }
}