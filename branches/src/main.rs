fn main() {

    // TODO >> if 表达式
    let number = 5;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    // TODO >> 在let 使用 if 表达式
    let condition = true;
    let count = if condition {
        5
    } else {
        6
    };

    println!("The value of count is: {}", count);

    // TODO >> 使用loop 重复执行代码

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is: {}", result);

    // TODO >> 使用 while 循环
    let mut  number = 3;
    while  number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // TODO >> 使用 for 遍历集合
    let a = [10, 20 ,30 ,40, 50];
    for value in a.iter() {
        println!("the value is: {}", value);
    }

    // 使用for 循环来倒计时
    for num in (1..4).rev() {
        println!("{}!", num);
    }
}

