# Rust-redis-demo

## Explain this code

这段代码的功能是使用Redis进行一系列操作，包括创建连接、设置/获取键值、向Redis流中添加数据、读取流中的数据等。

代码步骤解释：

1) 创建连接：使用Redis客户端库创建与Redis服务器的连接。
2) 设置/获取键值：使用连接对象设置一个键值对，并通过连接对象获取该键对应的值。
3) 向Redis流中添加数据：向名为"my_stream"的Redis流中添加一个流实体，该实体包含两个字段："name"和"value"。
4) 从Redis流中读取数据：从名为"my_stream"的Redis流中逆序获取最新的10个流实体，并打印出每个实体的字段和值。
5) 阻塞式读取：创建一个异步任务，在后台持续监听名为"my_stream"的Redis流，当有新的流实体添加到流中时，立即打印出该实体的信息。
6) 添加流实体：在延迟100毫秒后，向"my_stream"中添加两个新的流实体。
7) 最终等待和清理：延迟1秒后，删除之前设置的键和流，然后打印出结束信息。

代码中使用了tokio库来实现异步操作，使用了redis-rs库来操作Redis数据库。

## 实操

```shell
cargo new rust-redis
cd rust-redis/
c
cargo add tokio --features "full"
cargo add redis --features tokio-comp
cargo build
cargo watch -q -c -x 'run -q'
```
