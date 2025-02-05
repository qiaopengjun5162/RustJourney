# shape_area_rust

实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

## Explain this code

这段代码实现了一个简单的图形面积计算程序。它定义了一个 `Shape` 特征（trait），其中包含一个 `area` 方法，然后为圆形、三角形和正方形分别实现了这个特征。最后定义了一个泛型函数 `print_area` ，该函数接受实现了 `Shape` 特征的类型作为参数，计算并打印出图形的面积。

逐步解释代码：

1. 定义了 `Shape` 特征，包含一个 `area` 方法，用于计算图形的面积。
2. 为圆形结构体 `Circle` 实现了 `Shape` 特征，根据半径计算圆的面积。
3. 为三角形结构体 `Triangle` 实现了 `Shape` 特征，根据底边和高计算三角形的面积。
4. 为正方形结构体 `Square` 实现了 `Shape` 特征，根据边长计算正方形的面积。
5. 定义了泛型函数 `print_area` ，接受实现了 `Shape` 特征的类型的引用作为参数，计算并打印出图形的面积。
6. 在 `main` 函数中，创建了圆形、三角形和正方形的实例。
7. 调用 `print_area` 函数并传入不同的图形实例，分别计算并打印出它们的面积。

## Explain this code in Chinese

这段代码是一个使用 Rust 编程语言实现的图形面积计算程序。首先，定义了一个名为 `Shape` 的特征（trait），其中包含一个名为 `area` 的方法，用于计算图形的面积。然后，为圆形、三角形和正方形分别实现了这个特征。最后，定义了一个泛型函数 `print_area` ，该函数接受实现了 `Shape` 特征的类型作为参数，计算并打印出图形的面积。

逐步解释代码：

1. 定义了 `Shape` 特征，包含一个 `area` 方法，用于计算图形的面积。
2. 为圆形结构体 `Circle` 实现了 `Shape` 特征，根据半径计算圆的面积。
3. 为三角形结构体 `Triangle` 实现了 `Shape` 特征，根据底边和高计算三角形的面积。
4. 为正方形结构体 `Square` 实现了 `Shape` 特征，根据边长计算正方形的面积。
5. 定义了泛型函数 `print_area` ，接受实现了 `Shape` 特征的类型的引用作为参数，计算并打印出图形的面积。
6. 在 `main` 函数中，创建了圆形、三角形和正方形的实例。
7. 调用 `print_area` 函数并传入不同的图形实例，分别计算并打印出它们的面积。

## 代码运行

在命令行中运行以下命令：

```bash
shape_area_rust on  master [?] is 📦 0.1.0 via 🦀 1.77.0 via 🅒 base 
➜ cargo run
   Compiling shape_area_rust v0.1.0 (/Users/qiaopengjun/Code/rust/shape_area_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/shape_area_rust`
The area is: 78.53981633974483
The area is: 6
The area is: 36
```

## 参考资料

- [Rust 编程语言](https://www.rust-lang.org/zh-CN/learn/get-started)
- [Rust 中文文档](<https://rustwiki.org/docs/>)
- [Rust 备忘清单](<https://www.itbaoku.cn/reference/docs/rust.html/>)
- [Rust入门秘籍](<https://rust-book.junmajinlong.com/about.html>)
- [Rust 语言圣经](<https://course.rs/into-rust.html>)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [Rust 版本指南 中文版](<https://rustwiki.org/zh-CN/edition-guide/>)
