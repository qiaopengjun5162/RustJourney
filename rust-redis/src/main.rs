#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::{error::Error, time::Duration};
use tokio::time::sleep;

use redis::{
    from_redis_value,
    streams::{StreamClaimReply, StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // 2) Set / Get Key
    con.set("my_key", "Hello World!").await?;
    let result: String = con.get("my_key").await?;
    println!("->> my_key: {}\n", result);

    // 3) xadd to redis stream
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-01"), ("value", "value-01")],
    )
    .await?;
    let len: i32 = con.xlen("my_stream").await?;
    println!("->> my_stream len: {}\n", len);

    // 4) xrevrange the redis stream
    let result: Option<StreamRangeReply> = con.xrevrange_count("my_stream", "+", "-", 10).await?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("->> xrevrange stream entity: {} ", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!("->> {}: {}", name, from_redis_value::<String>(value)?);
            }
            println!();
        }
    }

    // 5) Blocking xread
    tokio::spawn(async {
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_tokio_connection().await.unwrap();
        loop {
            let ops = StreamReadOptions::default().count(1).block(0);
            let result: Option<StreamReadReply> = con
                .xread_options(&["my_stream"], &["$"], &ops)
                .await
                .unwrap();
            if let Some(reply) = result {
                for stream_key in reply.keys {
                    println!("->> xread block: {}", stream_key.key);
                    for stream_id in stream_key.ids {
                        println!("->> StreamId: {:?}", stream_id);
                    }
                }
                println!()
            }
        }
    });

    // 6) Add some stream entries
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-02"), ("value", "value-02")],
    )
    .await?;
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-03"), ("value", "value-03")],
    )
    .await?;

    // 7) Final wait & cleanup
    sleep(Duration::from_millis(1000)).await;
    con.del("my_key").await?;
    con.del("my_stream").await?;

    println!("->> the end");

    Ok(())
}
