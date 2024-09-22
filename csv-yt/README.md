# csv-yt

## 从一个 CSV 文件中读取数据并打印出来

### 代码的主要部分是

- `read_from_file`  函数，它接受一个文件路径作为参数，并返回一个  `Result`  类型的值。
- `Result`  类型是一个枚举类型，可以表示成功或失败的结果。
- 在这个函数中，成功的结果是一个空的  `Ok(())`  值，而失败的结果是一个  `Box<dyn Error>`  类型的错误。
- 在函数中，首先创建了一个  `csv::Reader`  对象，它可以从文件中读取 CSV 数据。
- 然后，通过  `from_path`  方法将文件路径传递给  `Reader`  对象，返回一个结果。
- 如果打开文件成功，就会得到一个  `Reader`  对象，否则会得到一个错误。
- 接下来，使用  `for`  循环遍历  `reader.records()` ，这个方法返回一个迭代器，可以逐行读取 CSV 文件的记录。
- 在循环中，使用  `?`  运算符检查每一行记录是否存在错误。
- 如果有错误，就会返回错误结果，否则会得到一条记录。
- 然后，使用  `println!`  宏打印出记录的内容。
- 最后， `main`  函数调用了  `read_from_file`  函数，并使用  `if let`  语句处理可能的错误。
- 如果  `read_from_file`  函数返回一个错误，就会打印出错误信息，否则什么都不做。

### 实操

```shell
cargo new csv-yt
cd csv-yt/
c # code .
cargo add csv
cargo run
```
