pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 完成函数签名
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 初始化输出
        let mut output: Vec<String> = vec![];
        
        for (string, command) in input.iter() {
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => {
                    let mut new_string = string.clone();
                    for _ in 0..*times {
                        new_string.push_str("bar");
                    }
                    new_string
                }
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入模块
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
