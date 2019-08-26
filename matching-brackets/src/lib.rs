fn is_open_bracket(c: char) -> bool {
    c == '(' || c == '[' || c == '{'
}

fn is_close_bracket(c: char) -> bool {
    c == ')' || c == ']' || c == '}'
}

fn match_bracket(c: char, top: Option<&char>) -> bool {
    match top {
        None => false,
        Some(b) => match b {
            '(' => c == ')',
            '[' => c == ']',
            '{' => c == '}',
            _ => false,
        },
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        if is_open_bracket(c) {
            stack.push(c)
        } else if is_close_bracket(c) {
            if !match_bracket(c, stack.last()) {
                return false;
            } else {
                stack.pop();
            }
        }
    }
    stack.is_empty()
}
