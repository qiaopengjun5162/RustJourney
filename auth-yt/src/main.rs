use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send()?;
    println!("{:?}", response);
    println!("response: {:?}", response.text());

    Ok(())
}

/*
这段代码使用了reqwest库来发送HTTP请求并获取响应。代码的主要步骤如下：

1. 导入reqwest库的blocking模块和Error类型。
2. 定义了一个main函数，返回一个Result类型，表示可能发生错误。
3. 创建一个reqwest的Client对象，用于发送HTTP请求。
4. 定义了一个字符串变量user，表示用户名为"testuser"。
5. 定义了一个Option类型的字符串变量passwd，表示密码为空。
6. 使用Client对象的get方法指定请求的URL为"http://httpbin.org/get"。
7. 使用basic_auth方法设置基本认证的用户名和密码。
8. 使用send方法发送请求，并将响应保存到response变量中。
9. 使用println!宏打印response的调试信息。
10. 使用println!宏打印response的文本内容。
11. 返回一个Ok值，表示程序执行成功。

这段代码的主要功能是发送一个GET请求到"http://httpbin.org/get"，并使用基本认证方式发送用户名和密码。然后将响应的调试信息和文本内容打印到控制台上。 */
