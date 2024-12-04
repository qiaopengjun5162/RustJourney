/*
 * @Author: QiaoPengjun5162 qiaopengjun0@gmail.com
 * @Date: 2023-04-13 21:39:51
 * @LastEditors: QiaoPengjun5162 qiaopengjun0@gmail.com
 * @LastEditTime: 2023-04-13 21:46:50
 * @FilePath: /smart/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE 
 */
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointers created.")
}
