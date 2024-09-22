use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

/*
这段代码使用了error_chain库来处理错误。
首先，通过error_chain宏定义了两个外部链接错误：Io和HttpRequest。
然后，定义了一个async的main函数，返回一个Result类型。
在main函数中，使用reqwest库发送了一个GET请求，并将返回的response存储在res变量中。
接着，打印了response的状态码和头部信息。
最后，将response的内容以字符串形式打印出来。

代码步骤解释：
1. 引入error_chain库和相应的宏。
2. 定义了两个外部链接错误：Io和HttpRequest。
3. 定义了一个async的main函数，返回一个Result类型。
4. 使用reqwest库发送了一个GET请求，并将返回的response存储在res变量中。
5. 打印了response的状态码。
6. 打印了response的头部信息。
7. 将response的内容以字符串形式存储在body变量中。
8. 打印了body的内容。
9. 返回Ok，表示程序执行成功。 */
