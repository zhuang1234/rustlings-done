// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

//这个函数返回冰箱里剩下多少冰淇淋。
//如果在晚上 10 点之前，则还剩 5 件。晚上10点有人吃
//全部，所以不会再剩下 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    //我们这里使用24小时制，所以10PM的值为22，12AM的值为0
    //选项输出应该优雅地处理 time_of_day > 23 的情况。
    //TODO: 完成函数体——记得返回一个选项！
    match time_of_day {
        x if x < 22 => Some(5),
        x @ 22..=23 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
