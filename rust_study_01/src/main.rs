// 所有权和生命周期是 Rust 和其它编程语言的主要区别，也是 Rust 其它知识点的基础。
// 从一个变量使用堆栈的行为开始，探究 Rust 设计所有权和生命周期的用意

// 动态数组因为大小在编译期无法确定，所以放在堆上，并且在栈上有一个包含了长度和容量的胖指针指向堆上的内存。

fn main() {
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

// main() 函数中定义了一个动态数组 data 和一个值 v，然后将其传递给函数 find_pos，
// 在 data 中查找 v 是否存在，存在则返回 v 在 data 中的下标，不存在返回 None
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}
