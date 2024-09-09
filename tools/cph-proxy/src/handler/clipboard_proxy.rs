use anyhow::{Context, Result};
use log::*;
use tokio::{io::AsyncReadExt, net::TcpStream};
use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub async fn clipboard_handler(mut socket: TcpStream) -> Result<()> {
    let mut len_buf = [0u8; 4];
    let mut checksum_buf = [0u8; 4];

    socket.read_exact(&mut len_buf).await?;
    let msg_len = u32::from_be_bytes(len_buf) as usize;

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
    wl_clipboard_rs::copy::copy(
        Options::new(),
        Source::Bytes(msg_buf.into_boxed_slice()),
        MimeType::Text,
    )
    .with_context(|| "failed to copy to clipboard")?;
    Ok(())
}
