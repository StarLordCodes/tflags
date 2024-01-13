pub struct TFlags {
    flags: Vec<String>,
    args: Vec<String>,
}

impl TFlags {
    pub fn parse() -> Self {
        let input_args: Vec<String> = std::env::args().collect();
        let (flags, args) = get_flags_args(&input_args);
        return TFlags { flags, args };
    }
}

fn get_flags_args(input_args: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut flags_output: Vec<String> = vec![];
    let mut args_output: Vec<String> = vec![];
    for arg in input_args {
        if arg.starts_with("-") || arg.starts_with("--") {
            flags_output.push(arg.to_owned());
        } else {
            args_output.push(arg.to_owned());
        }
    }
    (flags_output, args_output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_flags_args() {
        let input_args = vec![
            String::from("-a"),
            String::from("hello"),
            String::from("hello.txt"),
            String::from("-b"),
            String::from("--flag"),
            String::from("@1234"),
        ];
        let ex_flags_output = vec![
            String::from("-a"),
            String::from("-b"),
            String::from("--flag"),
        ];
        let ex_args_output = vec![
            String::from("hello"),
            String::from("hello.txt"),
            String::from("@1234"),
        ];
        let (got_flags_output, got_args_output) = get_flags_args(&input_args);
        assert_eq!(got_flags_output, ex_flags_output);
        assert_eq!(got_args_output, ex_args_output);
    }
}
