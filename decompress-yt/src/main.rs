use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let filename = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }
        if (*file.name()).ends_with('/') {
            println!("File {i} extracted to \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {i} extracted to \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}

/*
这段代码的作用是解压缩一个zip文件。

代码逐步解释如下：

1. 导入需要的库： `std::fs` 和 `std::io` 。
2. 定义 `main` 函数。
3. 在 `main` 函数中调用 `real_main` 函数并使用 `std::process::exit` 退出程序。
4. 定义 `real_main` 函数，返回一个 `i32` 类型的值。
5. 创建一个 `args` 变量，它是一个 `Vec` 类型，用于存储命令行参数。
6. 检查命令行参数的数量是否小于2，如果是，则打印使用说明并返回1。
7. 获取命令行参数中的第一个参数作为文件名。
8. 使用 `fs::File::open` 打开文件，并将其赋值给 `file` 变量。
9. 使用 `zip::ZipArchive::new` 将 `file` 变量封装为一个zip归档，并将其赋值给 `archive` 变量。
10. 使用 `for` 循环遍历归档中的每个文件。
11. 对于每个文件，使用 `archive.by_index` 获取文件，并将其赋值给 `file` 变量。
12. 使用 `file.enclosed_name` 获取文件的封闭名称，并将其赋值给 `outpath` 变量。
13. 如果 `outpath` 存在，打印文件的注释。
14. 如果 `file.name()` 以斜杠结尾，则打印文件被提取到的路径，并创建该路径。
15. 如果 `file.name()` 不以斜杠结尾，则打印文件被提取到的路径和文件大小，并创建父路径。
16. 创建一个 `outfile` 变量，使用 `fs::File::create` 创建一个文件，并将其赋值给 `outfile` 变量。
17. 使用 `io::copy` 将 `file` 中的内容复制到 `outfile` 中。
18. 如果是Unix系统，使用 `file.unix_mode()` 获取文件的Unix模式，并使用 `fs::set_permissions` 设置文件的权限。
19. 返回0表示成功执行。

这段代码的主要功能是解压缩zip文件并将文件提取到指定的路径中。 */
