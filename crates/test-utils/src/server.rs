use axum::Router;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    sync::oneshot::Sender,
    time::{Duration, sleep},
};

pub async fn start_test_server(router: Router) -> (String, Sender<()>) {
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("create test server");
    let addr = listener.local_addr().expect("get local address");
    let base_url = format!("http://{}", addr);

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await
            .ok();
    });

    // --- Wait for server to accept TCP connections (race-proof)
    let mut ok = false;
    for _ in 0..50 {
        if TcpStream::connect(addr).await.is_ok() {
            ok = true;
            break;
        }
        sleep(Duration::from_millis(50)).await;
    }
    assert!(ok, "server never became ready at {}", base_url);
    (base_url, shutdown_tx)
}
