# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)



特征
特征是方法的集合。

数据类型可以实现特征。为此，组成特征的方法为数据类型定义。例如，String 数据类型实现 From<&str> 特征。这允许用户编写 String::from("hello")。

这样，特征有点类似于 Java 接口和 C++ 抽象类。

一些其他常见的 Rust 特征包括：

Clone（克隆方法）
Display（允许通过 {} 进行格式化显示）
Debug（允许通过 {:?} 进行格式化显示）
由于特征表示数据类型之间的共享行为，因此在编写泛型时很有用。

更多信息
特征