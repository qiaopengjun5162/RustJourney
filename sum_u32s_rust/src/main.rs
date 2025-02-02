fn sum_u32s(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in numbers {
        // 检查加法是否会溢出
        if let Some(new_sum) = sum.checked_add(num) {
            sum = new_sum;
        } else {
            // 如果溢出，返回None
            return None;
        }
    }
    // 如果没有溢出，返回求和结果
    Some(sum)
}

// 使用try_fold方法进行更简洁的实现
/// numbers.iter().cloned() 首先对 numbers 中的每个元素进行克隆，
/// 创建了一个新的u32的迭代器，然后使用 try_fold 方法来累加这些克隆后的元素，
/// 调用 u32::checked_add 来进行加法操作。这样做是为了避免修改原始数据。
fn sum_u32_1(numbers: &[u32]) -> Option<u32> {
    numbers.iter().cloned().try_fold(0, u32::checked_add)
}

/// 使用 numbers.iter() 来获取原始迭代器，
/// 然后通过 try_fold 方法来累加原始数据中的元素，
/// 使用闭包 |acc, &x| acc.checked_add(x) 来进行加法操作。
/// 这种方式直接在原始数据上进行操作，避免了克隆操作。
fn sum_u32_2(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

fn main() {
    let numbers = [1u32, 2, 3, 4, 10000]; // 示例数组
    let result = sum_u32s(&numbers);
    match result {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Sum overflowed."),
    }

    let result = sum_u32_1(&numbers);
    match result {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Sum overflowed."),
    }

    let result = sum_u32_2(&numbers);
    match result {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Sum overflowed."),
    }
}
