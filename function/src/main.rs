fn main() {
    another_function(100, 100);

    // TODO >> 包含语句和表达式的函数体
    // TODO >> 函数体由一系列的语句和一个可选的结尾表达式构成。目前为止，我们只介绍了没有结尾表达式的函数，不过你已经见过作为语句一部分的表达式。
    // TODO >> 因为 Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。
    // TODO >> 其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。
    // TODO >> 实际上，我们已经使用过语句和表达式。语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。
    // TODO >> 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    let y = {
        let x = 5;
        x + 1
    };
    println!("the value of y is : {}", y);

    let number = six();
    println!("the value of number is : {}", number);

    let number1 = five();
    println!("the value of number1 is:{}", number1);
}


// TODO >> 函数遍布于 Rust 代码中。你已经见过语言中最重要的函数之一：main 函数，它是很多程序的入口点。你也见过 fn 关键字，它用来声明新函数。
// TODO >> Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
// TODO >> 函数也可以被定义为拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
// TODO >> 技术上讲，这些具体值被称为参数（arguments），但是在日常交流中，人们倾向于不区分使用 parameter 和 argument 来表示函数定义中的变量或调用函数时传入的具体值。
// TODO >> 在函数签名中，必须 声明每个参数的类型。这是 Rust 设计中一个经过慎重考虑的决定：要求在函数定义中提供类型注解，意味着编译器不需要你在代码的其他地方注明类型来指出你的意图。
fn another_function(x: i32, y: i64)
{
    println!("The value of x is: {0} y is: {1}", x, y);
}


// TODO >> 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
// TODO >> 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。


fn six() -> i32 {
    return 6;
}


fn five() -> i32 {
    5
}


