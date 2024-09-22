extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Time: {:?}", start.elapsed());
}

/*
这段代码的功能是将一个文件压缩为gzip格式。

代码步骤如下：
1. 检查命令行参数的数量，如果不是3个，则打印错误信息并返回。
2. 使用 `BufReader` 打开第一个命令行参数指定的文件，并将其赋值给 `input` 变量。如果文件打开失败，会产生一个错误。
3. 创建一个新的文件，用于存储压缩后的数据，并将其赋值给 `output` 变量。如果文件创建失败，会产生一个错误。
4. 创建一个 `GzEncoder` 对象，将 `output` 作为输出流，并使用默认的压缩级别。将该对象赋值给 `encoder` 变量。
5. 使用 `Instant` 记录当前时间，并将其赋值给 `start` 变量。
6. 将 `input` 的数据复制到 `encoder` 中，实现文件的压缩。如果复制过程中发生错误，会产生一个错误。
7. 调用 `encoder` 的 `finish` 方法，完成压缩过程，并将结果赋值给 `output` 变量。
8. 打印原始文件的长度，使用 `input.get_ref().metadata().unwrap().len()` 获取原始文件的元数据，并打印其长度。
9. 打印压缩后文件的长度，使用 `output.metadata().unwrap().len()` 获取压缩后文件的元数据，并打印其长度。
10. 打印程序执行的时间，使用 `start.elapsed()` 获取程序执行的时间，并打印出来。 */
