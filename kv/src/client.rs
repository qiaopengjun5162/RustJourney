mod noise_codec;
mod pb;

use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use pb::{Request, Response};
use tokio::net::TcpStream;
// use tokio_util::codec::LengthDelimitedCodec;
use noise_codec::{NoiseCodec, NoiseStream, NOISE_PARAMS};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = "localhost:8888";
    let stream = TcpStream::connect(addr).await?;
    // let mut stream = LengthDelimitedCodec::builder()
    let mut stream = NoiseCodec::builder(NOISE_PARAMS, true)
        // .length_field_length(2)
        .new_framed(stream)?;

    // -> e
    stream.send(Bytes::from_static(&[])).await?;
    info!("client -> e");

    // <- e, ee, s, es
    let data = stream.next().await.unwrap()?;
    info!("client <- e, ee, s, es");

    // -> s, se
    stream.send(data.freeze()).await?;
    info!("client -> s, se");

    // stream.codec_mut().into_transport_mode()?;
    stream.into_transport_mode()?;

    let msg = Request::new_put("hello", b"world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        println!("Got mag: {:?}", msg)
    }
    Ok(())
}
