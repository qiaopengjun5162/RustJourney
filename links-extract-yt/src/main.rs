use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    Ok(())
}

/*
这段代码使用了 `error_chain` 、 `select` 和 `reqwest` 等库。代码的功能是从指定网页获取HTML内容，并提取其中的所有链接，并将链接打印出来。

代码的步骤如下：
1. 引入所需的库，包括 `error_chain` 、 `select` 和 `reqwest` 。
2. 使用 `error_chain` 宏定义了一个错误链。
3. 定义了一个异步的 `main` 函数，返回一个 `Result` 类型。
4. 使用 `reqwest` 库的 `get` 方法发送一个GET请求，获取指定网页的响应。
5. 使用 `await` 等待请求的结果，并将响应内容以文本形式获取。
6. 使用获取到的HTML内容创建一个 `Document` 对象。
7. 使用 `find` 方法查找所有 `<a>` 标签。
8. 使用 `filter_map` 方法过滤并映射出每个 `<a>` 标签的 `href` 属性的值。
9. 使用 `for_each` 方法对每个链接执行打印操作。
10. 返回一个 `Ok(())` 表示程序执行成功结束。 */
