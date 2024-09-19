use axum::Router;
use cph_proxy::{cfg::*, handler, init};
use log::*;

#[volo::main]
async fn main() {
    let h = init::init();
    let listen_host = format!(
        "{}:{}",
        get_global_cfg().listen_host.0,
        get_global_cfg().listen_host.1
    );
    info!("starting server in: {}", listen_host);

    let app = handler::axum_router(Router::new());

    let listener = tokio::net::TcpListener::bind(&listen_host).await.unwrap();

    let (axum_s, tcp_s) = tokio::join!(axum::serve(listener, app), clipboard_proxy());
    axum_s.unwrap();
    tcp_s.unwrap();
    drop(h);
}

// FIXME: use semaphore to control accept
#[cfg(feature = "clipboard_proxy")]
async fn clipboard_proxy() -> anyhow::Result<()> {
    use tokio::net::TcpListener;

    match get_global_cfg().clipboard_proxy {
        Some(x) => {
            // let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(10));
            if x.enable {
                let listener = TcpListener::bind(x.forward_host).await?;

                loop {
                    // let permit = semaphore.clone().acquire().await?;
                    let (socket, _) = listener.accept().await?;
                    tokio::spawn(
                        async {
                            if let Err(e) = handler::clipboard_proxy::clipboard_handler(socket).await {
                                error!("clipboard set err: {e}");
                            }
                             // drop(permit);
                        }
                    );
                }
            }
        }
        None => todo!(),
    }
    Ok(())
}
