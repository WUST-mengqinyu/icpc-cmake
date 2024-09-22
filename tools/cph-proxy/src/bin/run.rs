use clap::{Parser, Subcommand};
use cph_proxy::init;
use log::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        comp_path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let _h = init::init();

    let rt = tokio::runtime::Runtime::new().expect("init tokio runtime failed");
    rt.block_on(async {
        let args = Args::parse();
        match args.command {
            Commands::Init { comp_path } => {
                let uuid = uuid::Uuid::new_v4().to_string();
                let src_path = comp_path.canonicalize()?;

                if !src_path.as_path().exists() {
                    anyhow::bail!("[{}] path no exist", src_path.display());
                }

                let mut entries = tokio::fs::read_dir(src_path.as_path()).await?;
                while let Ok(Some(sub_dir)) = entries.next_entry().await {
                    trace!("scanned dir: {}", sub_dir.path().display());

                    if sub_dir.file_type().await?.is_dir() {
                        if sub_dir.path().join(".cmake-ignore").exists() {
                            continue;
                        }

                        let name = sub_dir.file_name().into_string().map_err(|_| {
                            anyhow::anyhow!("cannot get sub_dir name: {}", sub_dir.path().display())
                        })? + ".cc";

                        let src = sub_dir.path().join("main.h");

                        trace!("{} link to {}", src.as_path().display(), name);

                        cph_proxy::service::default::common_utils::recreated_ref_in_running(
                            &uuid, &src, name,
                        )
                        .await?;
                    }
                }
            }
        }
        Ok(())
    })?;
    drop(_h);
    Ok(())
}
