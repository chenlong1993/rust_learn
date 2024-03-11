fn main() {
    //定一个数组
    let genders = ["zhangsan", "lisi", "wangwu"];
    //循环打印
    for i in genders {
        println!("{}", i)
    }
    println!("Hello, world!");


    let x: i32 = 10;
    // x=20;`
    let x = x * 2;
    println!("遮蔽后的变量值为{}", x);

    //加上mut表示这个变量是可以被覆盖的
    let mut age: i128 = 10;
    age = 18;
    println!("覆盖后的变量值为{}", age);

    //常量值必须声明类型
    const MAX_NUM: i32 = 32;
    println!("常量值为{}", MAX_NUM);

    let str = "test";
    println!("字符串值为{}", str);

    println!("1到5的范围是(1..5)");
    for i in 1..5 {
        print!("{}, ", i)
    }
    println!("");
    for i in (1..=5).rev() {
        print!("{}, ", i)
    }
    println!("");
    let sum: i32 = (1..=5).sum();
    println!("1到5的和为{}", sum);
    //元组访问
    let tup1: (i32, f32, bool) = (1, 2.0, true);
    let tup2: (i32, (i32, f32)) = (100, (2, 6.2));
    println!("元组1值为{}", tup1.0);
    println!("元组1值为{}", tup1.2);
    println!("元组2值为{},{}", tup1.0, tup2.1.1);

    let product_tup:(i32,(f64,bool,(i32,f64))) = (100, (2.0, true, (10, 20.0)));

    println!("product_tup值为{}", product_tup.1.2.0)
}



