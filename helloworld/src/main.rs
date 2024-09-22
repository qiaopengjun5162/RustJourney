use std::env;

// 这段代码的功能是从命令行参数中获取用户输入的名称，并打印出"Hello, name!"，如果没有输入名称，则打印出"Please use ./helloworld name"。

// 1. 首先导入标准库中的env模块。
// 2. 在main函数中，使用env::args()获取命令行参数，并调用skip(1)跳过第一个参数（即程序名称），再调用next()获取下一个参数，即用户输入的名称。
// 3. 使用match语句对获取到的名称进行匹配：
//    - 如果获取到了名称（Some(name)），则使用println!宏打印出"Hello, name!"，其中name为用户输入的名称。
//    - 如果没有获取到名称（None），则打印出"Please use ./helloworld name"。
fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => {
            println!("Please use ./helloworld name ");
        }
    }
}
