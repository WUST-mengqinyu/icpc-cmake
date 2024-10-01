use anyhow::{anyhow, bail, Context, Result};
use cph_proxy::cfg::*;
use ratatui::{
    crossterm::event,
    prelude::*,
    widgets::{block, Block, Gauge, LineGauge, List, ListItem, Paragraph},
};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt::Display,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, LazyLock, OnceLock, RwLock},
    thread,
    time::{Duration, Instant},
};
pub mod thread_manager;

#[derive(Debug)]
pub struct Cmd {
    pub brute_force: Source,
    pub solver: Source,
    pub gen: Source,
    pub checker: CheckerSource,
    pub times: u32,
    pub work_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Source {
    CMakeTarget(String),
    CPP20SingleFile(PathBuf),
}

pub trait Compileable: Display + Sync + Send {
    fn compile(&self) -> Result<PathBuf>;
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::CMakeTarget(t) => {
                f.write_str("cmake_target:")?;
                f.write_str(t)
            }
            Source::CPP20SingleFile(path_buf) => {
                f.write_str("single_file:")?;
                f.write_str(path_buf.as_os_str().to_str().unwrap_or(""))
            }
        }
    }
}

impl Display for CheckerSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckerSource::Source(source) => source.fmt(f),
            CheckerSource::Prebuilt(prebuilt_source) => {
                f.write_str("prebuilt:")?;
                f.write_fmt(format_args!("{}", prebuilt_source))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum CheckerSource {
    Source(Source),
    Prebuilt(PrebuiltSource),
}

impl Display for PrebuiltSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PrebuiltSource::CheckerTotalSame => "ck_rs_total_same",
            PrebuiltSource::CheckerIgnoreBlankWord => "ck_rs_ignore_blank_word",
        })
    }
}

#[derive(Debug, Clone)]
pub enum PrebuiltSource {
    CheckerTotalSame,
    CheckerIgnoreBlankWord,
}

impl CheckerSource {
    pub fn get_from_flag(
        matches: &clap::ArgMatches,
        target: &str,
        single_file: &str,
        prebuilt: &str,
    ) -> Self {
        match (
            matches.get_one::<String>(target),
            matches.get_one::<PathBuf>(single_file),
            matches.get_one::<PrebuiltSource>(prebuilt),
        ) {
            (Some(target), None, None) => Self::Source(Source::CMakeTarget(target.to_owned())),
            (None, Some(file), None) => Self::Source(Source::CPP20SingleFile(file.to_owned())),
            (None, None, Some(prebuilt)) => Self::Prebuilt(prebuilt.clone()),
            _ => panic!("invalid checker source"),
        }
    }
}

impl Source {
    pub fn get_from_flag(matches: &clap::ArgMatches, target: &str, single_file: &str) -> Self {
        match (
            matches.get_one::<String>(target),
            matches.get_one::<PathBuf>(single_file),
        ) {
            (Some(target), None) => Self::CMakeTarget(target.to_owned()),
            (None, Some(file)) => Self::CPP20SingleFile(file.to_owned()),
            _ => panic!("invalid source"),
        }
    }
}

impl Compileable for Source {
    fn compile(&self) -> Result<PathBuf> {
        match self {
            Source::CMakeTarget(target) => {
                Ok(get_global_cfg().project_root.join("build").join(target))
            }
            Source::CPP20SingleFile(path_buf) => {
                // TODO: support cfg
                let mut output_file = get_global_cfg().project_root.join("build").join(
                    path_buf
                        .as_path()
                        .file_name()
                        .ok_or(anyhow!("file name is none"))?,
                );
                output_file.set_extension("tmp.out");
                let exit = std::process::Command::new("clang++")
                    .arg("-o")
                    .args([output_file.as_path(), path_buf.as_path()])
                    .status()
                    .with_context(|| "do build single_file failed")?;
                if !exit.success() {
                    bail!("build single_file exit code: {}", exit);
                }
                Ok(output_file)
            }
        }
    }
}

impl Compileable for CheckerSource {
    fn compile(&self) -> Result<PathBuf> {
        match self {
            CheckerSource::Source(source) => source.compile(),
            CheckerSource::Prebuilt(prebuilt_source) => Ok(get_global_cfg()
                .project_root
                .join("target")
                .join("release")
                .join(prebuilt_source.to_string())),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn run(
    terminal: &mut Terminal<impl Backend>,
    workers: Vec<Worker>,
    mut compares: CompareRuns,
    rx: mpsc::Receiver<Event>,
) -> Result<()> {
    let mut redraw = true;
    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, &compares))?;
        }
        redraw = true;

        match rx.recv()? {
            Event::Input(_event) => {
                // TODO keyboard interact
                // if event.code == event::KeyCode::Char('q') {
                //     break;
                // }
            }
            Event::CompareAbort(worker_id, compare_id, result) => {
                terminal.insert_before(1, |buf| {
                    Paragraph::new(Line::from(vec![
                        Span::from("Abort "),
                        Span::styled(
                            format!("compare {compare_id}"),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::from(format!(
                            " in {}ms",
                            compares
                                .in_progress
                                .remove(&worker_id)
                                .unwrap()
                                .started_at
                                .elapsed()
                                .as_millis()
                        )),
                        Span::from(format!(" result {}", result)),
                    ]))
                    .render(buf.area, buf);
                })?;
                break;
            }
            Event::Resize => {
                terminal.autoresize()?;
            }
            Event::Tick => {}
            Event::CompareUpdate(worker_id, _compare_id, stage) => {
                let compare = compares.in_progress.get_mut(&worker_id).unwrap();
                compare.stage = stage;
                redraw = false;
            }
            Event::CompareDone(worker_id, compare_id, result) => {
                let compare = compares.in_progress.remove(&worker_id).unwrap();
                terminal.insert_before(1, |buf| {
                    Paragraph::new(Line::from(vec![
                        Span::from("Finished "),
                        Span::styled(
                            format!("compare {compare_id}"),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::from(format!(
                            " in {}ms",
                            compare.started_at.elapsed().as_millis()
                        )),
                        Span::from(format!(" result {}", result)),
                    ]))
                    .render(buf.area, buf);
                })?;
                match compares.next(worker_id) {
                    Some(d) => workers[worker_id].tx.send(d).unwrap(),
                    None => {
                        if compares.in_progress.is_empty() {
                            terminal.insert_before(1, |buf| {
                                Paragraph::new("Done !").render(buf.area, buf);
                            })?;
                            break;
                        }
                    }
                };
            }
        };
    }
    Ok(())
}

pub struct CompareRuns {
    all_cases: usize,
    pending: VecDeque<CompareRun>,
    in_progress: BTreeMap<WorkerId, RunInProgress>,
    pub compile_cache: Arc<RwLock<HashMap<String, std::result::Result<PathBuf, String>>>>, // TODO del it?
}

pub enum Event {
    Input(event::KeyEvent),
    Tick,
    Resize,
    CompareUpdate(WorkerId, CompareRunId, Stage),
    CompareAbort(WorkerId, CompareRunId, CompareResult),
    CompareDone(WorkerId, CompareRunId, CompareResult),
}

pub type RunInProgressId = usize;
pub type CompareRunId = usize;
pub type WorkerId = usize;

#[derive(Debug, Clone)]
pub enum CompareResult {
    AC,
    CE(String),
    WA(String, (String, String)),
    RE(String),
    TLE(u64, u64),
    CompareResultUnexpected(CompareResultUnexpected, String),
}

#[derive(Debug, Clone)]
pub enum CompareResultUnexpected {
    BruteForceBuildFailed,
    CheckerBuildFailed,
    GenBuildFailed,
    GenRuntimeError,
    GenOutputError,
    CompareRuntimeError,
}

// TODO: optimize result hint output
impl Display for CompareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let CompareResult::CompareResultUnexpected(res, hint) = self {
            return f.write_fmt(format_args!("Unexpected result: {:?}, hint: {}", res, hint));
        }
        match self {
            CompareResult::AC => write!(f, "[accept]"),
            CompareResult::CE(hint) => write!(f, "[compile error] {hint}"),
            CompareResult::WA(hint, (brute_force, solver)) => write!(
                f,
                "[wrong answer]: {hint}\nbrute_force:\n {brute_force}\nsolver:\n {solver}"
            ),
            CompareResult::RE(hint) => write!(f, "[runtime error]: {hint}"),
            CompareResult::TLE(time_limit, time_used) => write!(
                f,
                "[time limit excceed]: limit: {time_limit}ms, used: {time_used}ms"
            ),
            CompareResult::CompareResultUnexpected(_, _) => unreachable!(),
        }
    }
}

struct RunInProgress {
    id: RunInProgressId,
    started_at: Instant,
    stage: Stage,
    percent: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub enum Stage {
    Build,
    GenInput,
    RunBruteforce,
    RunSolver,
    RunChecker,
}

impl Stage {
    fn percent(&self) -> f64 {
        const FULL: f64 = 100f64;
        static WEIGTH_MAP: LazyLock<BTreeMap<Stage, u32>> = LazyLock::new(|| {
            BTreeMap::from([
                (Stage::Build, 10),
                (Stage::GenInput, 2),
                (Stage::RunBruteforce, 15),
                (Stage::RunSolver, 20),
                (Stage::RunChecker, 5),
            ])
        });
        static WEIGTH_FULL: LazyLock<u32> = LazyLock::new(|| WEIGTH_MAP.values().sum());
        FULL / (*WEIGTH_FULL as f64) * (WEIGTH_MAP[self] as f64)
    }
}

pub struct CompareRun {
    id: CompareRunId,
    test_case_index: u32,
    brute_force: Arc<Source>,
    solver: Arc<Source>,
    gen: Arc<Source>,
    checker: Arc<CheckerSource>,
    work_path: Arc<PathBuf>,
    compile_cache: Arc<RwLock<HashMap<String, std::result::Result<PathBuf, String>>>>,
    single_flight: Arc<singleflight::Group<std::result::Result<PathBuf, String>>>,
    out_cache: OnceLock<std::result::Result<std::fs::File, CompareResult>>, // reuse fd, clone when used
}

pub struct Worker {
    pub id: WorkerId,
    pub tx: mpsc::Sender<CompareRun>,
}

pub enum FutureOrNow<T> {
    Now(T),
    Future(std::thread::JoinHandle<T>),
}

impl CompareRun {
    fn build_file_with_cache<I: Compileable + 'static>(
        &self,
        item: Arc<I>,
    ) -> FutureOrNow<std::result::Result<PathBuf, String>> {
        let cache_read = self.compile_cache.read().unwrap();
        let entry = cache_read.get(item.to_string().as_str());
        match entry {
            Some(e) => FutureOrNow::Now(e.clone()),
            None => {
                drop(cache_read);
                let sf = self.single_flight.clone();
                let cache = self.compile_cache.clone();
                FutureOrNow::Future(std::thread::spawn(move || {
                    let key = item.to_string();
                    let r = sf.work(key.as_ref(), || item.compile().map_err(|e| e.to_string()));
                    let mut cache_write = cache.write().unwrap();
                    cache_write.insert(key, r.clone());
                    r
                }))
            }
        }
    }

    fn build(&self) -> Result<(PathBuf, PathBuf, PathBuf, PathBuf), CompareResult> {
        let brute_force = self.build_file_with_cache(self.brute_force.clone());
        let checker = self.build_file_with_cache(self.checker.clone());
        let solver = self.build_file_with_cache(self.solver.clone());
        let gen = self.build_file_with_cache(self.gen.clone());

        let [brute_force, checker, solver, gen] = [brute_force, checker, solver, gen]
            // .into_iter()
            .map(|r| match r {
                FutureOrNow::Now(r) => r,
                FutureOrNow::Future(f) => f.join().unwrap(),
            });
        Ok((
            brute_force.map_err(|e| {
                CompareResult::CompareResultUnexpected(
                    CompareResultUnexpected::BruteForceBuildFailed,
                    e,
                )
            })?,
            checker.map_err(|e| {
                CompareResult::CompareResultUnexpected(
                    CompareResultUnexpected::CheckerBuildFailed,
                    e,
                )
            })?,
            solver.map_err(CompareResult::CE)?,
            gen.map_err(|e| {
                CompareResult::CompareResultUnexpected(CompareResultUnexpected::GenBuildFailed, e)
            })?,
        ))
    }

    fn gen<P: AsRef<Path>>(&self, gen_exec_path: P) -> std::result::Result<(), CompareResult> {
        let mut child = std::process::Command::new(gen_exec_path.as_ref())
            .current_dir(self.work_path.as_path())
            .arg(self.test_case_index.to_string())
            .spawn()
            .map_err(|e| {
                CompareResult::CompareResultUnexpected(
                    CompareResultUnexpected::GenRuntimeError,
                    e.to_string(),
                )
            })?;
        child.wait().map_err(|e| {
            CompareResult::CompareResultUnexpected(
                CompareResultUnexpected::GenRuntimeError,
                e.to_string(),
            )
        })?;

        Ok(())
    }

    fn test_case_input_file_clone(&self) -> std::result::Result<impl std::io::Read, CompareResult> {
        let out_cache = self
            .out_cache
            .get_or_init(|| {
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(format!("{}.in", self.test_case_index))
                    .map_err(|e| {
                        CompareResult::CompareResultUnexpected(
                            CompareResultUnexpected::GenOutputError,
                            e.to_string(),
                        )
                    })
            })
            .to_owned();
        out_cache.as_ref().map_err(|e| e.to_owned()).and_then(|f| {
            f.try_clone().map_err(|e| {
                CompareResult::CompareResultUnexpected(
                    CompareResultUnexpected::GenOutputError,
                    e.to_string(),
                )
            })
        })
    }

    fn test_case_solver_output_file_name(&self) -> String {
        format!("{}.ans", self.test_case_index)
    }

    fn test_case_brute_force_output_file_name(&self) -> String {
        format!("{}.correct", self.test_case_index)
    }

    fn run_brute_force(&self, brute_force: PathBuf) -> std::result::Result<(), CompareResult> {
        let mut child = std::process::Command::new(brute_force)
            .current_dir(self.work_path.as_path())
            .arg(self.test_case_index.to_string())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| {
                CompareResult::CompareResultUnexpected(
                    CompareResultUnexpected::GenRuntimeError,
                    e.to_string(),
                )
            })?;

        let mut reader = self.test_case_input_file_clone()?;
        std::io::copy(&mut reader, &mut child.stdin.as_ref().unwrap())
            .map_err(|e| CompareResult::RE(format!("copy input to brute force failed: {e}")))?;

        // self.test_case_solver_output_file_name()
        child.wait().map_err(|e| {
            CompareResult::CompareResultUnexpected(
                CompareResultUnexpected::GenRuntimeError,
                e.to_string(),
            )
        })?;

        Ok(())
    }

    fn run_solver(&self, solver: PathBuf) -> std::result::Result<(), CompareResult> {
        Ok(())
    }

    fn compare(&self, checker: PathBuf) -> std::result::Result<CompareResult, CompareResult> {
        todo!()
    }

    fn do_with_stage<R, F: FnOnce() -> R>(
        &self,
        worker_id: WorkerId, // TODO: need a context?
        tx: &mpsc::Sender<Event>,
        stage: Stage,
        f: F,
    ) -> R {
        let r = f();
        let _ = tx.send(Event::CompareUpdate(worker_id, self.id, stage));
        r
    }

    fn work(
        &self,
        worker_id: WorkerId,
        tx: &mpsc::Sender<Event>,
    ) -> std::result::Result<CompareResult, CompareResult> {
        let (brute_force, checker, solver, gen) =
            self.do_with_stage(worker_id, tx, Stage::Build, || self.build())?;

        self.do_with_stage(worker_id, tx, Stage::GenInput, || self.gen(gen))?;

        self.do_with_stage(worker_id, tx, Stage::RunBruteforce, || {
            self.run_brute_force(brute_force)
        })?;

        self.do_with_stage(worker_id, tx, Stage::RunSolver, || self.run_solver(solver))?;

        self.do_with_stage(worker_id, tx, Stage::RunChecker, || self.compare(checker))
    }
}

impl CompareRuns {
    pub fn next(&mut self, worker_id: WorkerId) -> Option<CompareRun> {
        match self.pending.pop_front() {
            Some(d) => {
                self.in_progress.insert(
                    worker_id,
                    RunInProgress {
                        id: d.id,
                        started_at: Instant::now(),
                        stage: Stage::Build,
                        percent: 0f64,
                    },
                );
                Some(d)
            }
            None => None,
        }
    }
}

pub fn compares(cmd: Cmd) -> CompareRuns {
    let single_flight = Arc::new(singleflight::Group::new());
    let compile_cache = Arc::new(RwLock::new(HashMap::new()));
    let work_path = Arc::new(cmd.work_path);
    let brute_force = Arc::new(cmd.brute_force);
    let solver = Arc::new(cmd.solver);
    let gen = Arc::new(cmd.gen);
    let checker = Arc::new(cmd.checker);

    let pending = (0..cmd.times)
        .map(|id| CompareRun {
            id: id as usize,
            test_case_index: id,
            work_path: work_path.clone(),
            brute_force: brute_force.clone(),
            solver: solver.clone(),
            gen: gen.clone(),
            checker: checker.clone(),
            compile_cache: compile_cache.clone(),
            single_flight: single_flight.clone(),
            out_cache: OnceLock::new(),
        })
        .collect();
    CompareRuns {
        pending,
        all_cases: cmd.times as usize,
        in_progress: BTreeMap::new(),
        compile_cache,
    }
}

pub fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                    event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
                    _ => {}
                };
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}
#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
pub fn workers(worker_num: usize, tx: mpsc::Sender<Event>) -> Vec<Worker> {
    (0..worker_num)
        .map(|id| {
            let (worker_tx, worker_rx) = mpsc::channel::<CompareRun>();
            let tx = tx.clone();
            thread::spawn(move || {
                while let Ok(compare_run) = worker_rx.recv() {
                    let _ = match compare_run.work(id, &tx) {
                        Ok(res) => tx.send(Event::CompareDone(id, compare_run.id, res)),
                        Err(e) => tx.send(Event::CompareAbort(id, compare_run.id, e)),
                    }; // ignore this error, maybe abort event
                }
            });
            Worker { id, tx: worker_tx }
        })
        .collect()
}

fn draw(frame: &mut Frame, compares: &CompareRuns) {
    let area = frame.area();

    let block = Block::new().title(block::Title::from("Progress").alignment(Alignment::Center));
    frame.render_widget(block, area);

    let vertical = Layout::vertical([Constraint::Length(2), Constraint::Length(4)]).margin(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
    let [progress_area, main] = vertical.areas(area);
    let [list_area, gauge_area] = horizontal.areas(main);

    // total progress
    let done = compares.all_cases - compares.pending.len() - compares.in_progress.len();
    #[allow(clippy::cast_precision_loss)]
    let progress = LineGauge::default()
        .filled_style(Style::default().fg(Color::Blue))
        .label(format!("{done}/{}", compares.all_cases))
        .ratio(done as f64 / compares.all_cases as f64);
    frame.render_widget(progress, progress_area);

    // in progress compares
    let items: Vec<ListItem> = compares
        .in_progress
        .values()
        .map(|compare_prgress| {
            ListItem::new(Line::from(vec![
                Span::raw(symbols::DOT),
                Span::styled(
                    format!(" compare {:>2}", compare_prgress.id),
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " ({}ms)",
                    compare_prgress.started_at.elapsed().as_millis()
                )),
            ]))
        })
        .collect();
    let list = List::new(items);
    frame.render_widget(list, list_area);

    #[allow(clippy::cast_possible_truncation)]
    for (i, (_, compare)) in compares.in_progress.iter().enumerate() {
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(compare.percent / 100.0);
        if gauge_area.top().saturating_add(i as u16) > area.bottom() {
            continue;
        }
        frame.render_widget(
            gauge,
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top().saturating_add(i as u16),
                width: gauge_area.width,
                height: 1,
            },
        );
    }
}
