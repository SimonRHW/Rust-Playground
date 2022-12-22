use std::fmt;

fn main() {
    println!("Hello, world!");
    // 通常情况下，`{}` 会被任意变量内容所替换。
    println!("{} days", 31);
    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // 可以在 `:` 后面指定特殊的格式 将第二个参数的值打印为二进制数
    println!("{} of {:b} people know binary, the other half don't", 1, 10);
    //  将第二个参数的值打印为八制数
    println!("{} of {:o} people know octal, the other half don't", 1, 10);
    //  将第二个参数的值打印为十六制数
    println!("{} of {:x} people know hexadecimal, the other half don't", 1, 10);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    let pi = 3.141592;
    println!("The value of pi is {:.*}", 2, pi);

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[derive(Debug)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
            write!(f, "{}", self)
        }
    }
    println!("This struct print: `{:?}`", Structure(3));
    #[derive(Debug)]
    struct Position {
        longitude: f32,
        latitude: f32,
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.longitude, self.latitude)
        }
    }
    println!("This position print:`{:?}`", Position { longitude: 1.987, latitude: 2.983 });
}
