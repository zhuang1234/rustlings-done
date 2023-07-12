//AsRef 和 A​​sMut 允许廉价的引用到引用的转换。
//有关它们的更多信息，请访问 https://doc.rust-lang.org/std/convert/trait.AsRef.html
//和 https://doc.rust-lang.org/std/convert/trait.AsMut.html 分别。
//执行 `rusdlingshint as_ref_mut` 或使用 `hint` watch 子命令来获取提示。

//获取给定参数中的字节数（不是字符数）。
//TODO：适当添加 AsRef 特征作为特征边界。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    *arg.as_mut() *= *arg.as_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
