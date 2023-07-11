fn main() {
    let mut s = String::from("hello"); // 创建一个字符串
    let r1 = &s; // 创建一个不可变引用
    let r2 = &s; // 创建另一个不可变引用
    println!("{} and {}", r1, r2); // 输出 hello and hello
    let r3 = &mut s; // 创建一个可变引用
    r3.push_str(", world"); // 修改字符串
    println!("{}", r3); // 输出 hello, world
    // println!("{}", r1); // 编译错误
    // println!("{}", r2); // 编译错误
}
