fn main() {
    // TODO >> 标量类型
    // TODO >> Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

    // TODO >> 整数 是一个没有小数部分的数字。我们在第二章使用过 u32 整数类型。
    // TODO >> 该类型声明表明，它关联的值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 i 开头而不是 u）。
    // TODO >> 在有符号列和无符号列中的每一个变体（例如，i16）都可以用来声明整数值的类型
    let a:i8 = 8;
    let b:i16 = 16;
    let c:i32 = 32;
    let d:i64 = 64;
    let e:i128 = 128;

    let a1:u8 = 8;
    let b1:u16 = 16;
    let c1:u32 = 32;
    let d1:u64 = 64;
    let e1:u128 = 128;

    println!("a:{}, b:{}, c:{}, d:{}, e{}",a, b, c, d, e);
    println!("a1:{}, b1:{}, c1:{}, d1:{}, e1{}",a1, b1, c1, d1, e1);

    // TODO >> Rust 也有两个原生的 浮点数（floating-point numbers）类型，它们是带小数点的数字。Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。
    // TODO >> 默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。
    let x:f64 = 64.00;
    let y:f32 = 32.1;

    println!("x:{}, y:{}",x, y);

    // TODO >> 数值运算 Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。
    // 加法
    let sum = 6 + 7;

    // 减法
    let diff = 10 - 1;

    // 乘法
    let product = 100 * 2;

    // 除法
    let quotient = 36 / 6;

    // 取余
    let remainder = 41 % 5;
    println!("sum:{}, diff:{}, product:{}, quotient:{}, remainder:{}",sum, diff, product, quotient, remainder);

    // TODO >> 布尔型 正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示
    let t = true;
    let f:bool = false;
    println!("t:{}, f:{}", t, f);
    // TODO >> 字符类型 目前为止只使用到了数字，不过 Rust 也支持字母。Rust 的 char 类型是语言中最原生的字母类型，如下代码展示了如何使用它。（注意 char 由单引号指定，不同于字符串使用双引号。）

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("c:{}, z:{}, heart_eyed_cat:{}", c, z, heart_eyed_cat);

    // TODO >> Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。
    // TODO >> 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
    // TODO >> Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
    // TODO >> 不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符” 可能与 Rust 中的 char 并不符合。


    // TODO >> 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // TODO >> 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    // TODO >> 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // TODO >> tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值，
    // let (x1, y1, z1) = tup;

    // TODO >> 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。
    let first_tup = tup.0;

    println!("The value of first_tup is: {}", first_tup);
    // TODO >> 数组（array）。与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let arr = [1, 2, 3, 4, 5, 6];

    // TODO >> 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时，数组非常有用。但是数组并不如 vector 类型灵活。
    // TODO >> vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，你可能应该使用 vector。
    let _arr1:[i128; 6] = [1, 2, 3, 4, 5, 6];

    // TODO >> 如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号，然后在方括号中指定数组的长度
    let _arr2 = [10, 3];

    // TODO >> 数组是一整块分配在栈上的内存。可以使用索引来访问数组的元素
    let first_arr = arr[0];
    let second_arr = arr[1];
    println!("The value of first_arr is: {}", first_arr);
    println!("The value of second_arr is: {}", second_arr);


    // TODO >> isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的

    // TODO >> 字符串
    let str = "abc";
    println!("{}-{}", str, str.len());
}

