use iroh::protocol::ProtocolHandler;
use log::info;

pub const ALPN: &[u8] = b"chat/echo/0";

#[derive(Debug, Clone)]
pub struct EchoProtocol;

impl ProtocolHandler for EchoProtocol {
    fn accept(
        &self,
        conn: iroh::endpoint::Connection,
    ) -> impl Future<Output = Result<(), iroh::protocol::AcceptError>> + Send {
        Box::pin(async move {
            // Connecting node id
            let node_id = conn.remote_node_id();
            info!("Accepting connection from node id: {:#?}", node_id);

            // Get the bi-directional stream
            let (mut send, mut recv) = conn.accept_bi().await?;

            // Echo any bytes received

            // This function sends all bytes received from "recv" and send them to "send"
            let bytes_sent = tokio::io::copy(&mut recv, &mut send).await?;
            info!("Copied over {bytes_sent} byte(s)");

            send.finish()?;

            conn.closed().await;

            Ok(())
        })
    }
}
