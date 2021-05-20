fn main() {
    // TODO >> 变量的可变性
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);


    // TODO >> 变量和常量的区别
    // TODO >> 声明一个常量
    const MAX_POINTS: u32 = 10000;
    println!("MAX_POINTS is: {}", MAX_POINTS);

    // TODO >> 隐藏（shadowing）
    // 定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
    let y = 1;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

}
