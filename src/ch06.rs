#![allow(unused_variables)] // 不对 未使用的变量 warning


pub fn ch06_01_defining_an_enum() {
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            // 枚举的成员
            V6,  // 枚举的成员
        }

        // 创建实例
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;


        // 关联结构体
        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        println!("{:#?}",home);
        println!("{:#?}",loopback);
    }


    // 简洁的关联方法
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
    // 枚举中放入结构体 也可以是另一个枚举
    {
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum SomeEnum {
            A,
            B,
            C,
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
            SomeEnum(SomeEnum),
        }
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
        struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体

        // 可以在枚举上定义方法
        impl Message {
            fn call(&self) {
                // 在这里定义方法体
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // Option 枚举 和 其对于空值的优势
    // rust 没有空值,不过它确实拥有一个可以编码存在编码存在或不存在概念的枚举
    /*
        enum Option<T> {
            Some(T),
            None,
        }
    */
    {
        // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，
        // 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;

        // 这句话会提示 编译错误 类型不同就不能相加
        // 提前声明 某些值可能为空 那么使用这些值的时候 就必须处理 为空的情况 才能正确编译
        // 所以没有提前准备 Option 申明的 运行时 还是可能会出现 空值错误的
        // some_number + 1;
    }
}

pub fn ch06_02_match(){
    // match 控制流运算符

    // 当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。
    // 如果模式并不匹配这个值，将继续执行下一个分支
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    // 绑定值的模式
    {

        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        // 嵌套枚举
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                },
            }
        }

        println!("{}",value_in_cents(Coin::Quarter(UsState::Alaska)))
    }

    // 匹配Option<T>
    {
        fn plus_one(x:Option<i32>) -> Option<i32>{
            match x{
                None => None,
                Some(i) => Some(i + 1)

            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    // 匹配是穷尽的
    {
        // 必须每个枚举都必须要有 执行的分支 或者有一个 _ 通配符 默认
        let some_u8_value = 0u8;
        // match 匹配一个 值而不是枚举的时候就一定要有一个 _ 通配符
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }

    }






}

pub fn ch06_03_if_let() {
    // if let 简单控制流
    // if let 语法让我们以一种不那么冗长的方式结合 if 和 let，
    // 来处理只匹配一个模式的值而忽略其他模式的情况。
    let some_u8_value = Some(4);
    // 我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
    // 为了满足 match 表达式（穷尽性）的要求，
    // 必须在处理完这唯一的成员后加上 _ => ()，
    // 这样也要增加很多样板代码。
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // 没有佩佩到 Some(3) 就走了这个分支什么都不做
    }

    // 使用 if let
    // 一次 只匹配一种类型 并且忽略其他所有的值
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // 可以在 if let 中包含一个 else
    // lse 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else
    {

        let mut count = 0;
//        match coin {
//            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//            _ => count += 1,
//        }
    }
    {
        let mut count = 0;
//        if let Coin::Quarter(state) = coin {
//            println!("State quarter from {:?}!", state);
//        } else {
//            count += 1;
//        }
    }
}