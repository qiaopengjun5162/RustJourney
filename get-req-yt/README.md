# Rust-get-req-yt

## 代码解释 Explain this code

这段代码使用了error_chain库来处理错误。
首先，导入了error_chain和std::io::Read库。
然后，使用error_chain宏定义了错误链，其中包含了两个foreign_links：Io和HttpRequest。

在main函数中，首先通过reqwest::blocking::get函数发送一个GET请求到"<http://httpbin.org/get">，并将返回的结果赋值给变量res。然后，创建一个空的字符串变量body。

接下来，通过res.read_to_string函数将res的内容读取为字符串，并将结果存储在body中。

最后，打印出res的状态码、头部信息和body的内容，并返回一个Ok结果。

## 代码的步骤如下

1. 导入所需的库：error_chain和std::io::Read。
2. 定义错误链，包含两个foreign_links：Io和HttpRequest。
3. 在main函数中，发送一个GET请求到"<http://httpbin.org/get">，并将结果赋值给变量res。
4. 创建一个空的字符串变量body。
5. 将res的内容读取为字符串，并将结果存储在body中。
6. 打印出res的状态码、头部信息和body的内容。
7. 返回一个Ok结果。
