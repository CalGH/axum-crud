use crate::get_api_router;
use crate::get_key_cert;
use axum_server::{bind_openssl, tls_openssl::OpenSSLConfig, Handle as AxumHandle, HttpConfig};
use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::{Handle as SignalHandle, Signals};
use std::error::Error;
use std::net::SocketAddr;
use tokio::{runtime::Runtime, sync::oneshot::error::RecvError, task, time::Duration};

pub fn start_axum_api() -> Result<(), Box<dyn Error>> {
    const REQUIRED: &[&str] = &["CERT_FOLDER", "CARGO_MANIFEST_DIR", "CERT_NAME", "KEY_NAME"];

    let rt = Runtime::new().unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));

    let (cert, key) = get_key_cert(REQUIRED);

    let config = OpenSSLConfig::from_pem_file(cert, key).unwrap();

    let httpconfig = HttpConfig::new()
        .http1_only(true)
        .http2_only(false)
        .max_buf_size(8192)
        .build();

    let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<()>();

    let mut signals = Signals::new(&[SIGINT]).unwrap();

    let signalhandle: SignalHandle = signals.handle();

    let axumhandle = AxumHandle::new();
    let axumhandle_clone = axumhandle.clone();

    let mut tasks: task::JoinSet<()> = task::JoinSet::new();

    let app = rt.block_on(get_api_router());

    tasks.spawn(async move {
        bind_openssl(addr, config)
            .handle(axumhandle_clone)
            .http_config(httpconfig)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    tasks.spawn(async move {
        while let Some(signal) = signals.next().await {
            match signal {
                SIGINT => {
                    dbg!("Received signal", signal);
                    signalhandle.close();
                }
                _ => (),
            }
        }
        println!("Signal listener exiting");
        let _ = shutdown_sender.send(());
    });

    let _res = rt.block_on(async move {
        let _ = shutdown_receiver.await?;

        println!("Giving axum 10 seconds to shutdown");
        dbg!(axumhandle.graceful_shutdown(Some(Duration::from_secs(10))));

        while let Some(_) = dbg!(tasks.join_next().await) {
            println!("Task Joined");
        }
        Result::Ok::<(), RecvError>(())
    });
    Ok(())
}
