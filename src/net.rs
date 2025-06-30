use iroh::{Endpoint, NodeAddr, protocol::Router};

use crate::net::protocol::{ALPN, EchoProtocol};

mod protocol;

pub async fn start_accept_side() -> anyhow::Result<Router> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    let router = Router::builder(endpoint)
        .accept(protocol::ALPN, EchoProtocol)
        .spawn();

    Ok(router)
}

pub async fn connect_side(addr: NodeAddr) -> anyhow::Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    // Connects to an existing address
    let conn = endpoint.connect(addr, ALPN).await?;

    // Opens a bi-directional stream
    let (mut send, mut recv) = conn.open_bi().await?;

    // Writes the data to be echoed
    send.write_all(b"Hello World!").await?;

    // Signal the end of data for this stream
    send.finish()?;

    // Reads the response, but limits it to 1000 bytes
    let response = recv.read_to_end(1000).await?;
    assert_eq!(&response, b"Hello World!");

    // Closes the connection
    conn.close(0u32.into(), b"Bye!");

    // The above call only queues a close connection (see how it's not async)
    // Call this to ensure connection is closed
    endpoint.close().await;

    Ok(())
}
