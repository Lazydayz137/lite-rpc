use crate::{
    bridge::LiteBridge, bridge_pubsub::LitePubSubBridge, rpc::LiteRpcServer,
    rpc_pubsub::LiteRpcPubSubServer,
};
use jsonrpsee::server::ServerBuilder;
use solana_lite_rpc_core::AnyhowJoinHandle;
pub async fn start_servers(
    rpc: LiteBridge,
    pubsub: LitePubSubBridge,
    ws_addr: String,
    http_addr: String,
) -> anyhow::Result<()> {
    let rpc = rpc.into_rpc();
    let pubsub = pubsub.into_rpc();

    let ws_server_handle = ServerBuilder::default()
        .ws_only()
        .build(ws_addr.clone())
        .await?
        .start(pubsub)?;

    let http_server_handle = ServerBuilder::default()
        .http_only()
        .build(http_addr.clone())
        .await?
        .start(rpc)?;

    let ws_server: AnyhowJoinHandle = tokio::spawn(async move {
        log::info!("Websocket Server started at {ws_addr:?}");
        ws_server_handle.stopped().await;
        anyhow::bail!("Websocket server stopped");
    });

    let http_server: AnyhowJoinHandle = tokio::spawn(async move {
        log::info!("HTTP Server started at {http_addr:?}");
        http_server_handle.stopped().await;
        anyhow::bail!("HTTP server stopped");
    });

    tokio::select! {
        res = ws_server => {
            anyhow::bail!("WebSocket server {res:?}");
        },
        res = http_server => {
            anyhow::bail!("HTTP server {res:?}");
        },
    }
}