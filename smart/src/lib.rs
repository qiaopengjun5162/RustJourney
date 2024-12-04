/*
 * @Author: QiaoPengjun5162 qiaopengjun0@gmail.com
 * @Date: 2023-04-13 22:32:41
 * @LastEditors: QiaoPengjun5162 qiaopengjun0@gmail.com
 * @LastEditTime: 2023-04-13 22:53:33
 * @FilePath: /smart/src/lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes of scope = {}", Rc::strong_count(&a));
}
