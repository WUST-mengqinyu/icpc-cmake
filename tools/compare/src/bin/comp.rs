use clap::{arg, Command};
use compare::*;
use cph_proxy::cfg::*;
use ratatui::{TerminalOptions, Viewport};
use std::sync::mpsc;

const WORKER_NUM: usize = 4;
const DEFAULT_TEST_TIMES: u32 = 20;

trait FromMatches {
    fn get_from_flag(matches: &clap::ArgMatches) -> Self;
}

impl FromMatches for Cmd {
    fn get_from_flag(matches: &clap::ArgMatches) -> Self {
        let brute_force = Source::get_from_flag(
            matches,
            "brute_force_cmake_target",
            "brute_force_cpp20_single_file",
        );

        let solver =
            Source::get_from_flag(matches, "solver_cmake_target", "solver_cpp20_single_file");
        let gen = Source::get_from_flag(matches, "gen_cmake_target", "gen_cpp20_single_file");
        let checker = CheckerSource::get_from_flag(
            matches,
            "checker_cmake_target",
            "checker_single_file",
            "checker_prebuilt",
        );

        let times = matches
            .get_one::<u32>("times")
            .unwrap_or(&DEFAULT_TEST_TIMES)
            .to_owned();

        let work_path = matches
            .get_one("work_path")
            .unwrap_or(&get_global_cfg().project_root)
            .to_owned();

        Self {
            brute_force,
            solver,
            gen,
            checker,
            times,
            work_path,
        }
    }
}

fn main() {
    color_eyre::install().expect("install color_eyre failed");
    let _h = init_refresh_global_cfg();
    let matches = Command::new("compare")
        .about("compare two solver output")
        .args([
            // brute force
            arg!(--brute_force_cmake_target "brute force cmake target")
                .conflicts_with("brute_force_cpp20_single_file"),
            arg!(--brute_force_cpp20_single_file "brute force cpp20 file")
                .conflicts_with("brute_force_cmake_target"),
            // solver
            arg!(--solver_cmake_target "solver cmake target")
                .conflicts_with("solver_cpp20_single_file"),
            arg!(--solver_cpp20_single_file "solver cpp20 file")
                .conflicts_with("solver_cmake_target"),
            // gen
            arg!(--gen_cmake_target "gen cmake target").conflicts_with("gen_cpp20_single_file"),
            arg!(--gen_cpp20_single_file "gen cpp20 file").conflicts_with("gen_cmake_target"),
            // checker
            arg!(--checker_prebuilt "checker prebuilt")
                .conflicts_with_all(["checker_cmake_target", "checker_single_file"]),
            arg!(--checker_cmake_target "checker cmake target")
                .conflicts_with_all(["checker_prebuilt", "checker_single_file"]),
            arg!(--checker_single_file "checker single file")
                .conflicts_with_all(["checker_prebuilt", "checker_cmake_target"]),
        ])
        .get_matches();

    let cmd = Cmd::get_from_flag(&matches);

    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(8),
    });

    let (tx, rx) = mpsc::channel();
    input_handling(tx.clone());
    let workers = workers(WORKER_NUM, tx);
    let mut compares = compares(cmd);

    for w in &workers {
        let d = compares.next(w.id).unwrap();
        w.tx.send(d).unwrap();
    }

    run(&mut terminal, workers, compares, rx).expect("runtime error");

    ratatui::restore();

    drop(_h);
}
