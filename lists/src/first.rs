// This code implements a singly linked list data structure with push and pop operations.
// The list is implemented using a recursive enum called Link, which can either be empty or contain a boxed Node.
// The Node struct contains an element of type i32 and a Link to the next Node.
// The push operation adds a new Node to the front of the list, while the pop operation removes and returns the element at the front of the list.
// The Drop trait is implemented to properly deallocate memory when the list goes out of scope.
// The code also includes a test module with basic tests for the list operations.

// 这段代码实现了一个链表数据结构，包含了以下几个部分：

// 1. `List`  结构体：包含一个  `head`  成员，表示链表的头节点。
// 2. `Link`  枚举类型：表示链表节点的链接方式，可以是空（Empty）或者指向另一个节点的指针（More）。
// 3. `Node`  结构体：表示链表中的节点，包含一个  `elem`  成员表示节点的值，和一个  `next`  成员表示指向下一个节点的链接方式。
// 4. `impl List`  块：包含了链表的方法实现，包括创建新链表、添加节点、删除节点等操作。
// 5. `impl Drop for List`  块：实现了链表的析构函数，在链表被释放时，逐个释放链表中的节点。

// 具体代码步骤如下：

// 1. 引入  `std::mem`  模块。
// 2. 定义  `List`  结构体，包含一个  `head`  成员，类型为  `Link`  枚举类型。
// 3. 定义  `Link`  枚举类型，表示链表节点的链接方式，可以是空（Empty）或者指向另一个节点的指针（More）。
// 4. 定义  `Node`  结构体，表示链表中的节点，包含一个  `elem`  成员表示节点的值，和一个  `next`  成员表示指向下一个节点的链接方式。
// 5. 实现  `List`  结构体的方法块：
//    - `new`  方法：创建一个新的链表，头节点为空。
//    - `push`  方法：将一个新的节点添加到链表的头部。
//    - `pop`  方法：从链表的头部弹出一个节点，并返回其值。
// 6. 实现  `Drop`  trait 的  `drop`  方法，用于在链表被释放时释放链表中的节点。
// 7. 定义测试模块  `test` ，包含一个测试函数  `basics` 。
// 8. 在测试函数中，创建一个新的链表，并进行一系列的操作和断言，验证链表的功能是否正常。

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
