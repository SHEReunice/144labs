//const PI: f32 = 3.14159;

/*
fn add(x: i32, y: i32) -> i32{
    return x + y;
}
*/

/*
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}
*/


fn main() {
    
    /*基本类型
    let x = 12; //默认情况下,这是i32(无符号整型，表示正负整数)
    let a = 12u8;  //无符号整型：u8 u32 u64 u128表示正整数
    let b = 4.3;//默认情况下，这是f64(浮点数f32 f64)
    let c = 4.3f32;
    let bv = true;
    let t = (13,false);//(元组tuple(value,value,……))
    let sentence = "hello world!";
    println!("{} {} {} {} {} {} {} {}",
            x, a, b, c, bv, t.0, t.1, sentence
            );
    */

    /*基本类型转换
    let a = 12u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);  //使用as关键字进行基本类型转换
    */

    /*常量
    println!("To make an apple {} from scratch, you must first create a universe.", PI);
    */

    /*数组：数据类型为[T;N]
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
    */
    /*函数
    println!("{}", add(42, 13));
    */
      
    /*
    //返回一个元组
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    //将元组拆解为两个变量
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
    */

    /*if-else
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
    */

    /*无限循环loop——break退出当前循环
   let mut x = 0;
   loop {
       x += 1;
       if x == 42 {
           break;
       }
   } 
   println!("{}", x);
   */

   /*
   let mut x = 0;
   while x != 42 {
       x += 1;
   }
   */

   /*..运算符创建一个可以生成包含起始数字、但不包含末尾数字的数字序列的迭代器
   ..=运算符创建一个可以生成包含起始数字、且包含末尾数字的数字序列的迭代器
   for x in 0..5 {
       println!("{}", x);
   }

   for x in 0..=5 {
    println!("{}", x);
   }
   */

   /*
   //match关键字，用于匹配值的所有可能条件，并在匹配为真时执行相应代码，类似C里面的switch
   let x = 42;
   match x {
       0 => {
           println!("found zero");
       }
       //我们可以匹配多个值
       1 | 2 => {
           println!("found 1 or 2");
       }
       //可以匹配迭代器
       3..=9 => {
           println!("found a number 3 to 9 inclusive");
       }
       //可以匹配数值绑定到变量
       matched_num @ 10..=100 => {
           println!("found {} number between 10 to 100!", matched_num);
       }
       //这是默认匹配，如果没有处理所有情况，则必须存在该匹配
       _ => {
           println!("found something else!");
       }
   }
   */

   /*
   //loop可以被中断返回一个值
   let mut x = 0;
   let v = loop {
       x += 1;
       if x == 13 {
           break "found the 13";
       }
   };
  println!("from loop: {}", v);
  */

  /*
  //从块表达式返回值,如果if、match、函数或作用域块中的最后一条语句是不带;的表达式，Rust将把它作为一个值从块中返回。
   println!("from function: {}", example());
   */

   
    
}

/*
fn example() -> i32 {
    let x = 42;
    //Rust的三元表达式
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        //注意，当它只是一个返回表达式时，大括号是可选的
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        //这个作用域块让我们得到一个不影响函数作用域的结果
        let a = 1;
        let b = 2;
        a+b
    };
    println!("from block: {}", v);

    //在最后从函数返回值的惯用方法
    v + 4
}
*/
