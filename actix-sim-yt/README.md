# Rust-actix-sim-yt

## 使用Actix-Web和Rhai库构建的简单的Web服务器

服务器有两个路由，一个是 `/multiply/{num1}/{num2}` ，另一个是 `/add/{num1}/{num2}` 。这两个路由分别用于执行一个名为 `multiply` 和一个名为 `add` 的函数。

 `multiply` 和 `add` 函数都接受两个路径参数 `num1` 和 `num2` ，并将其解析为 `(i64, i64)` 元组。
 然后，它们创建一个Rhai引擎，并将 `num1` 和 `num2` 注册为Rhai函数。
 接下来，它们通过调用Rhai引擎的 `eval_file` 方法来执行一个Rhai脚本文件，该文件的路径是 `src/multiply.rhai` 和 `src/add.rhai` 。
 执行结果被存储在 `result` 变量中，并最终以字符串的形式返回。

在 `main` 函数中，创建了一个HttpServer实例，并将其绑定到 `127.0.0.1:8000` 地址上。服务器使用 `App` 构建，并注册了 `multiply` 和 `add` 函数作为服务。然后，服务器开始运行并等待请求。

整体而言，这段代码实现了一个简单的Web服务器，可以通过两个路由执行乘法和加法操作，并使用Rhai脚本作为计算逻辑。
