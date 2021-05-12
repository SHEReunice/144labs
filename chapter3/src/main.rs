/*
//结构体
struct SeaCreature {
    // String 是个结构体
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,

}
*/
/*3.2
//函数(function)和方法(method)不同
//静态方法：属于某个类型，调用时使用::运算符
//实例方法：属于某个类型的实例，调用时使用.运算符
fn main(){
    //使用静态方法来创建一个String实例
    let s = String::from("hello world!");
    //使用实例来调用方法
    println!("{} is {} characters long.", s, s.len());
}
*/
/*3.3
Rust程序有三个存放数据的内存区域
- 数据内存：对于固定大小和静态的数据
- 栈内存 ：对于在函数钟声明为变量的数据
- 堆内存： 对于在程序运行时创建的数据； 当数据添加到该区域时，我们称其为分配，从本区域中删除数据后，我们将其称为释放
*/
/*3.4
//在内存中创建数据
struct SeaCreature{
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
fn main(){
    //SeaCreature的数据在栈上
    let ferris = SeaCreature{
        //String结构体也在栈上
        //但也存放了一个数据在堆上的引用
        animal_type: String::from("螃蟹"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("大钳子"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("章鱼"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("无"),
    };

    println!(
        "{} 是只 {}。它有 {} 只胳膊 {} 条腿，还有一个 {}。",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} 是只 {}。它有 {} 只胳膊 {} 条腿，还有一个 {}。",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon
    );
    

}
*/
/*3.5类元组结构体
//可以创建像元组一样被使用的结构体
struct Location(i32,i32);
fn main() {
    //这仍然是一个在栈上的结构体
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}
*/
/*3.6 类单元结构体
一个unit是空元组()的别称，此类结构体被称为类单元
struct Maker;

fn main(){
    let _m = Maker;
}
*/
/*3.7枚举
枚举允许使用enum关键字创建一个新类型，该类型的值可以包含几个带标记的元素
match有助于确保对所有可能的枚举值进行彻底的处理，使其成为确保高质量代码的强大工具
#! [allow(dead_code)] //this line prevents compiler warning
enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature
    {
        species: Species::Octopus,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
    }
}
*/

