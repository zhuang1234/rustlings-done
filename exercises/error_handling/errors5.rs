// errors5.rs

//这个程序使用了 errors4 中代码的修改版本。

//这个练习使用了一些概念，我们要到课程的后面才会讲到，比如 `Box` 和
//`From` 特征。现在详细了解它们并不重要，但如果您愿意，可以提前阅读。
//现在，将 `Box<dyn ???>` 类型视为“我想要任何可以做的事情？？？”类型，其中，给定
//Rust 通常的运行时安全标准，应该让您觉得有点宽松！
//简而言之，这个盒子的特殊用例是当你想要拥有一个值并且你只关心它是一个
//实现特定特征的类型。为此，将 Box 声明为 Box<dyn Trait> 类型，其中 Trait 是特征
//编译器查找在该上下文中使用的任何值。对于本练习，该上下文是潜在的错误
//可以在结果中返回。
//我们可以用什么来描述这两个错误？换句话说，是否存在两个错误都实现的特征？

//执行 `rustlings hint errors5` 或使用 `hint` watch 子命令获取提示。

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
