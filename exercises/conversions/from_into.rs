// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

//你的任务是完成这个实现
//为了编译 `let p = Person::from("Mark,20")` 行
//请注意，您需要将年龄组件解析为“usize”
//类似于`"4".parse::<usize>()`。这个结果需要
//适当处理。
//
//脚步：
//1.如果提供的字符串长度为0，则返回默认的Person
//2. 用逗号分割给定的字符串
//3. 从 split 操作中提取第一个元素并将其用作名称
//4.如果name为空，则返回默认的Person
//5. 从 split 操作中提取另一个元素并将其解析为 `usize` 作为年龄
//如果解析年龄时出现问题，则返回 Person 的默认值
//否则，则返回带有结果的实例化 Person 对象
//我还没做完

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let vec_s: Vec<&str> = s.split(',').collect();
        if vec_s.len() == 2 && vec_s[0].len() > 0 {
            match vec_s[1].parse::<usize>() {
                Ok(age) => {
                    return Self {
                        name: vec_s[0].to_string(),
                        age: age,
                    }
                }
                Err(_) => return Person::default(),
            }
        }
        Person::default()
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
