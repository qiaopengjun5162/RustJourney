// 基础要求：固定类型的数组排序
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 提高部分：使用范型和PartialOrd实现对任意类型的排序
fn generic_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 测试基础要求
    let mut arr1 = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr1);
    println!("基础要求排序后的数组: {:?}", arr1);

    // 测试提高部分
    let mut arr2 = [64, 34, 25, 12, 22, 11, 90];
    generic_bubble_sort(&mut arr2);
    println!("提高部分排序后的数组: {:?}", arr2);
}
