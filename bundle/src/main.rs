use clap::{Parser, Subcommand};
use handler::*;

mod consts;
mod handler;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Gen {
        #[clap(short, long)]
        exec_name: Option<Vec<String>>,
        #[clap(short, long)]
        exec_path: Option<Vec<String>>,
    },
    Clear {},
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Gen { exec_name: None, exec_path: None }) {
        Commands::Gen {
            ref exec_name,
            ref exec_path,
        } => {
            match (exec_name, exec_path) {
                (None, None) => {
                    let mut ctx = BundlerContext::new();
                    ctx.clear_all()?;
                    ctx.bundle_all()?;
                },
                (None, Some(_)) => panic!(
                    "Gen command exec_name len({}) and exec_path len({}) not equal, use `bundle help gen` to get help",
                    exec_name.as_ref().map_or(0, |v|v.len()),
                    exec_path.as_ref().map_or(0, |v|v.len()),
                ),
                (Some(_), None) => panic!(
                    "Gen command exec_name len({}) and exec_path len({}) not equal, use `bundle help gen` to get help",
                    exec_name.as_ref().map_or(0, |v|v.len()),
                    exec_path.as_ref().map_or(0, |v|v.len()),
                ),
                (Some(execs), Some(sources)) => {
                    if sources.len() != execs.len() {
                        panic!(
                            "`exec_name` and `exec_path` has not equal len, use `bundle help gen` to get help"
                        );
                    }
                    let deal_len = sources.len();
                    let mut ctx = BundlerContext::new();
                    for i in 0..deal_len {
                        ctx.clear_target(&execs[i])?;
                        ctx.bundle_target( &execs[i], &sources[i])?;
                    }
                },
            }
        }
        Commands::Clear {} => BundlerContext::new().clear_all()?,
    }
    Ok(())
}
