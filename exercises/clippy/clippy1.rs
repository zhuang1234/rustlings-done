// clippy1.rs
//Clippy 工具是用于分析代码的 lints 集合
//这样你就可以发现常见的错误并改进你的 Rust 代码。
//
//对于这些练习，当出现 clippy 警告时，代码将无法编译
//从输出中检查 clippy 的建议以解决练习。
//执行 `rustlings hint clippy1` 或使用 `hint` watch 子命令获取提示。
//Clippy 检查浮点文本，这些浮点文本分别近似于 std::f32::consts 或 std::f64::consts 中定义的常量，建议使用预定义的常量

use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
