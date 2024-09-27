mod noise_codec;
mod pb;

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use pb::{request::*, *};
use std::sync::Arc;
use tokio::net::TcpListener;
// use tokio_util::codec::LengthDelimitedCodec;
use noise_codec::{NoiseCodec, NoiseStream, NOISE_PARAMS};
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

// 通过#[tokio::main]注解(annotation)，使得async main自身成为一个async runtime。
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;

    info!("Listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New client: {:?} accepted", addr);

        let shared = state.clone();

        tokio::spawn(async move {
            // let mut stream = LengthDelimitedCodec::builder()
            let mut stream = NoiseCodec::builder(NOISE_PARAMS, false)
                // .length_field_length(2)
                .new_framed(stream)?;

            // <- e
            let data = stream.next().await.unwrap()?;
            info!("server <- e");

            // -> e, ee, s, es
            stream.send(data.freeze()).await?;
            info!("server -> e, ee, s, es");

            // <- s, se
            let _data = stream.next().await.unwrap()?;
            info!("<- s, se");

            // stream.codec_mut().into_transport_mode()?;
            stream.into_transport_mode()?;

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    None => unimplemented!(),
                };
                stream.send(response.into()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}

// kv store (基于 tokio)
// 使用 protobuf(prost) 设计 kv store 的传输协议
// 用 tokio TcpListener / TcpStream 实现客户端和服务器的交互
// 使用 dashmap 在内存中存储 kv
// 作业：用sled替换 dashmap 让 kv store 可持久化
// delete

// 服务端应用的基本组成部分
// 数据序列化：serde/protobuf/flatbuffer/capnp/etc
// 传输协议：tcp/http/websocket/quic/etc
// 安全协议：TLS/noise protocol/secio/etc
// 应用协议：your own application logic
// 数据在各个部分之间的流传：共享内存，channel 等

// 使用 noise protocol 增强 kv store 的安全性
// 实现 NoiseCodec?
// 实现 Stream/Sink?
