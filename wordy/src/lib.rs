pub struct WordProblem;

#[derive(PartialEq, Debug)]
enum Token {
    What,
    Number(i32),
    Plus,
    Minus,
    Mult,
    Div,
    Raised,
    Error,
}

fn scan(command: &str) -> Vec<Token> {
    command
        .to_lowercase()
        .split_whitespace()
        .flat_map(|word| match word {
            "what" => Some(Token::What),
            "is" => None,
            "plus" => Some(Token::Plus),
            "minus" => Some(Token::Minus),
            "divided" => Some(Token::Div),
            "multiplied" => Some(Token::Mult),
            "by" => None,
            "to" => None,
            "the" => None,
            "power" => None,
            "raised" => Some(Token::Raised),
            s if s.ends_with("th") || s.ends_with("nd") || s.ends_with("st") => {
                match s[..s.len() - 2].parse::<i32>() {
                    Err(_) => Some(Token::Error),
                    Ok(num) => Some(Token::Number(num)),
                }
            }
            s => match s.parse::<i32>() {
                Err(_) => Some(Token::Error),
                Ok(num) => Some(Token::Number(num)),
            },
        })
        .collect()
}

fn parse_number(token: &Token) -> Option<i32> {
    match token {
        Token::Number(n) => Some(*n),
        _ => None,
    }
}

fn parse_expression(tokens: &[Token]) -> Option<i32> {
    let mut num = parse_number(tokens.first()?)?;
    let mut iter = tokens[1..].iter();
    loop {
        match iter.next() {
            Some(Token::Plus) => num += parse_number(iter.next()?)?,
            Some(Token::Minus) => num -= parse_number(iter.next()?)?,
            Some(Token::Mult) => num *= parse_number(iter.next()?)?,
            Some(Token::Div) => num /= parse_number(iter.next()?)?,
            Some(Token::Raised) => num = num.pow(parse_number(iter.next()?)? as u32),
            Some(_) => return None,
            None => break,
        }
    }
    Some(num)
}

fn parse_question(tokens: &[Token]) -> Option<i32> {
    match tokens.first()? {
        Token::What => parse_expression(&tokens[1..]),
        _ => None,
    }
}

pub fn answer(command: &str) -> Option<i32> {
    if command.ends_with('?') {
        parse_question(&scan(&command[..command.len() - 1]))
    } else {
        None
    }
}
