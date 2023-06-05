// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

#[rustfmt::skip]
macro_rules! my_macro {
    // 多个匹配项使用分号分割
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
