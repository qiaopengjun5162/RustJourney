# Rust-warp-demo

## Explain main.rs code

这段代码是一个简单的Web服务器，使用了Rust的warp库来构建路由和处理HTTP请求。代码的主要功能如下：

1. 引入必要的库和模块。
2. 定义了一个常量WEB_FOLDER，表示静态文件的目录。
3. 定义了一个DbPool结构体，用于表示数据库连接池。
4. 定义了一个名为with_db_pool的函数，用于创建一个过滤器，将数据库连接池传递给处理函数。
5. 在main函数中，创建了一个数据库连接池的Arc实例db_pool。
6. 定义了一个处理"/hi"路径的路由，返回字符串"Hello, world from hi!"。
7. 定义了一个处理"/"路径的路由，返回静态文件目录中的index.html文件。
8. 将上述两个路由组合成一个路由apis。
9. 定义了一个处理静态文件的路由content。
10. 将apis和content组合成一个最终的路由routes。
11. 启动Web服务器，监听本地地址127.0.0.1和端口8080，将请求交给routes处理。

代码的执行流程如下：

1. 引入必要的库和模块。
2. 定义了一个DbPool结构体。
3. 定义了一个名为with_db_pool的函数，该函数接受一个`Arc<DbPool>`参数，并返回一个过滤器。
4. 在main函数中，创建一个Arc实例db_pool作为数据库连接池。
5. 定义了一个处理"/hi"路径的路由hi，返回字符串"Hello, world from hi!"。
6. 定义了一个处理静态文件的路由content。
7. 将hi和todos_filter(db_pool.clone())组合成一个路由apis。
8. 定义了一个处理根路径"/"的路由root，返回静态文件目录中的index.html文件。
9. 将content和root组合成一个路由static_site。
10. 将apis和static_site组合成一个最终的路由routes。
11. 打印"start web-server"。
12. 启动Web服务器，监听本地地址127.0.0.1和端口8080，将请求交给routes处理。

代码的功能是创建一个简单的Web服务器，处理不同路径的请求，并提供静态文件服务。

## Code Run

```shell
cargo watch -q -c -w src -x run
