# bubble_sorting_rust

## Explain Code

这段代码首先定义了两个函数，一个是针对固定类型数组的冒泡排序`bubble_sort`，另一个是针对任意类型的冒泡排序`generic_bubble_sort`。这两个函数的实现原理都是通过两层循环，比较相邻的元素，如果前一个元素大于后一个元素，就交换它们的位置。这个过程会一直进行，直到数组完全排序。

在`main`函数中，我们首先测试了`bubble_sort`函数，对一个整数数组进行排序，并打印排序后的结果。然后，我们测试了`generic_bubble_sort`函数，对一个整数数组进行排序，并打印排序后的结果。

需要注意的是，冒泡排序的时间复杂度为O(n^2)，空间复杂度为O(1)，是一种简单但效率较低的排序算法。在实际应用中，可以考虑使用更高效的排序算法，如快速排序、归并排序等。

## 实操

```shell
cargo new bubble_sorting_rust
cd bubble_sorting_rust
cargo run
```
