use std::{ffi::OsString, io::Write, process::Stdio, str::FromStr};

use anyhow::{Context, Result};
use cli_clipboard::ClipboardProvider;
use log::*;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::cfg;

pub async fn clipboard_handler(mut socket: TcpStream) -> Result<()> {
    let mut len_buf = [0u8; 8]; // FIXME: support 32 bit?
    let mut checksum_buf = [0u8; 4];

    socket.read_exact(&mut len_buf).await?;
    let msg_len = usize::from_be_bytes(len_buf);

    socket.read_exact(&mut checksum_buf).await?;
    let checksum = u32::from_be_bytes(checksum_buf);

    let mut msg_buf = vec![0u8; msg_len];
    socket.read_exact(&mut msg_buf).await?;

    let hs = crc32fast::hash(msg_buf.as_ref());
    if hs != checksum {
        warn!(
            "crc32 failed, checksum expected: {}, actual hash: {}",
            msg_len, checksum
        );
    }

    let recv = std::str::from_utf8(msg_buf.as_ref())?;
    trace!("recv tcp: [{}]", recv);

    auto_ctx_set_clipboard(recv)
}

fn auto_ctx_set_clipboard<S: AsRef<str>>(recv: S) -> Result<()> {
    if cfg::get_global_cfg().clipboard_proxy.is_some_and(|cfg| {
        if cfg.enable && cfg.try_use_sys_cmd {
            try_wl_copy(cfg, &recv).is_ok()
        } else {
            false
        }
    }) {
    } else {
        set_content(
            cli_clipboard::ClipboardContext::new()
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .with_context(|| "init clipboard ctx failed")?,
            &recv,
        )?;
    }
    Ok(())
}

fn try_wl_copy<S: AsRef<str>>(cfg: cfg::ClipBoardProxy, recv: S) -> Result<()> {
    let cmd = match cfg.sys_cmd_path {
        Some(path) => path.into_os_string(),
        None => OsString::from_str("wl-copy").expect("wl-copy cannot to OsString"),
    };
    let exist = std::process::Command::new("which")
        .arg(&cmd)
        .status()
        .is_ok_and(|code| code.success());
    if !exist {
        return Err(anyhow::anyhow!(""));
    }
    let mut p = std::process::Command::new(&cmd)
        .stdin(Stdio::piped())
        .spawn()?;
    let mut stdin = p
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("get stdin failed"))?;
    stdin.write_all(recv.as_ref().as_bytes())?;
    stdin.flush()?;
    Ok(())
}

// FIXME: use https://github.com/bugaevc/wl-clipboard to support cross platform
fn set_content<P: ClipboardProvider, S: AsRef<str>>(mut p: P, content: S) -> Result<()> {
    p.set_contents(content.as_ref().to_owned())
        .map_err(|e| anyhow::anyhow!(e.to_string()))
        .with_context(|| "failed to copy to clipboard")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard() {
        auto_ctx_set_clipboard("testtest").expect("set clipboard failed");
    }
}
