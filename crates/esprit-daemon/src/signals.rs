use tokio::signal;

pub async fn wait_for_shutdown() {
    tokio::select! {

        _ = signal::ctrl_c() => {}

    }
}
