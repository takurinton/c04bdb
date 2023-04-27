use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

#[derive(Clone, PartialEq, Debug)]
enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Hat,
    LeftParen,
    RightParen,

    Int(i64),
}

fn char_to_token(c: char) -> Option<Token> {
    match c {
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '*' => Some(Token::Star),
        '/' => Some(Token::Slash),
        '%' => Some(Token::Percent),
        '^' => Some(Token::Hat),
        '(' => Some(Token::LeftParen),
        ')' => Some(Token::RightParen),
        _ => None,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn tokenizer(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        // 空白は無視
        if c.is_whitespace() {
            continue;
        }

        // 数字の場合数字以外がくるまで繰り返す
        if is_digit(c) {
            let mut num = String::new();
            num.push(c);

            while let Some(&c) = chars.peek() {
                if is_digit(c) {
                    num.push(c);
                    chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Int(num.parse().unwrap()));
            continue;
        }

        // それ以外の文字はトークンに変換
        if let Some(token) = char_to_token(c) {
            tokens.push(token);
            continue;
        }

        // 数字以外で token に変換できない文字が来たらエラー
        return Err(format!("Unexpected character: {}", c));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    #[test]
    fn test_char_to_token() {
        assert_eq!(char_to_token('+'), Some(Token::Plus));
        assert_eq!(char_to_token('-'), Some(Token::Minus));
        assert_eq!(char_to_token('*'), Some(Token::Star));
        assert_eq!(char_to_token('/'), Some(Token::Slash));
        assert_eq!(char_to_token('%'), Some(Token::Percent));
        assert_eq!(char_to_token('^'), Some(Token::Hat));
        assert_eq!(char_to_token('('), Some(Token::LeftParen));
        assert_eq!(char_to_token(')'), Some(Token::RightParen));
        assert_eq!(char_to_token('a'), None);
    }

    #[test]
    fn test_is_digit() {
        assert_eq!(is_digit('0'), true);
        assert_eq!(is_digit('1'), true);
        assert_eq!(is_digit('2'), true);
        assert_eq!(is_digit('3'), true);
        assert_eq!(is_digit('4'), true);
        assert_eq!(is_digit('5'), true);
        assert_eq!(is_digit('6'), true);
        assert_eq!(is_digit('7'), true);
        assert_eq!(is_digit('8'), true);
        assert_eq!(is_digit('9'), true);
        assert_eq!(is_digit('a'), false);
    }

    #[test]
    fn test_tokenizer() {
        let tokens1 = tokenizer(String::from("2 + 2"));
        assert_eq!(
            tokens1,
            Ok(vec![Token::Int(2), Token::Plus, Token::Int(2),])
        );

        let tokens2 = tokenizer(String::from("2 + 2 * 2"));
        assert_eq!(
            tokens2,
            Ok(vec![
                Token::Int(2),
                Token::Plus,
                Token::Int(2),
                Token::Star,
                Token::Int(2),
            ])
        );

        let tokens3 = tokenizer(String::from("(1 + 1) * 10 / 5"));
        assert_eq!(
            tokens3,
            Ok(vec![
                Token::LeftParen,
                Token::Int(1),
                Token::Plus,
                Token::Int(1),
                Token::RightParen,
                Token::Star,
                Token::Int(10),
                Token::Slash,
                Token::Int(5),
            ])
        );

        let tokens4 = tokenizer(String::from("1000"));
        assert_eq!(tokens4, Ok(vec![Token::Int(1000)]));
    }
}

// enum Operator {
//     // Root,
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Mod,
//     Pow,
// }

// impl Operator {
//     fn from_token(token: &Token) -> Option<Operator> {
//         match token {
//             Token::Plus => Some(Operator::Add),
//             Token::Minus => Some(Operator::Sub),
//             Token::Star => Some(Operator::Mul),
//             Token::Slash => Some(Operator::Div),
//             Token::Percent => Some(Operator::Mod),
//             Token::Hat => Some(Operator::Pow),
//             _ => None,
//         }
//     }
// }

// token list から計算順序を整理する
// MEMO: 計算順序のところもう少し上手く探索できるはず
fn parse(tokens: Vec<Token>) -> Result<String, String> {
    let mut tokens = tokens;
    let mut result = String::new();

    // まずは括弧を展開する
    while let Some(i) = tokens.iter().position(|t| t == &Token::LeftParen) {
        let mut depth = 1;
        let mut j = i + 1;
        while depth > 0 {
            if tokens[j] == Token::LeftParen {
                depth += 1;
            } else if tokens[j] == Token::RightParen {
                depth -= 1;
            }
            j += 1;
        }

        let sub_tokens = tokens[i + 1..j - 1].to_vec();
        let sub_result = parse(sub_tokens)?;
        tokens[i] = Token::Int(sub_result.parse().unwrap());
        tokens.drain(i + 1..j);
    }

    // Hat を計算する
    while let Some(i) = tokens.iter().position(|t| t == &Token::Hat) {
        let left = match tokens[i - 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i - 1])),
        };
        let right = match tokens[i + 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i + 1])),
        };

        let _result = left.pow(right as u32);
        tokens[i - 1] = Token::Int(_result);
        tokens.drain(i..i + 2);
    }

    // Mul, Div, Mod を計算する
    while let Some(i) = tokens
        .iter()
        .position(|t| t == &Token::Star || t == &Token::Slash || t == &Token::Percent)
    {
        let left = match tokens[i - 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i - 1])),
        };
        let right = match tokens[i + 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i + 1])),
        };

        let _result = match tokens[i] {
            Token::Star => left * right,
            Token::Slash => left / right,
            Token::Percent => left % right,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i])),
        };
        tokens[i - 1] = Token::Int(_result);
        tokens.drain(i..i + 2);
    }

    // Add, Sub を計算する
    while let Some(i) = tokens
        .iter()
        .position(|t| t == &Token::Plus || t == &Token::Minus)
    {
        let left = match tokens[i - 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i - 1])),
        };
        let right = match tokens[i + 1] {
            Token::Int(n) => n,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i + 1])),
        };

        let _result = match tokens[i] {
            Token::Plus => left + right,
            Token::Minus => left - right,
            _ => return Err(format!("Unexpected token: {:?}", tokens[i])),
        };
        tokens[i - 1] = Token::Int(_result);
        tokens.drain(i..i + 2);
    }

    let _result = match tokens[0] {
        Token::Int(n) => n,
        _ => return Err(format!("Unexpected token: {:?}", tokens[0])),
    };

    result.push_str(&_result.to_string());
    Ok(result)
}

#[test]
fn parse_test() {
    let tokens1 = vec![Token::Int(2), Token::Plus, Token::Int(2)];
    let result1 = parse(tokens1);
    assert_eq!(result1, Ok("4".to_string()));

    let tokens2 = vec![
        Token::Int(2),
        Token::Plus,
        Token::Int(2),
        Token::Star,
        Token::Int(2),
    ];
    let result2 = parse(tokens2);
    assert_eq!(result2, Ok("6".to_string()));

    let tokens3 = vec![
        Token::LeftParen,
        Token::Int(1),
        Token::Plus,
        Token::Int(1),
        Token::RightParen,
        Token::Star,
        Token::Int(10),
        Token::Slash,
        Token::Int(5),
    ];
    let result3 = parse(tokens3);
    assert_eq!(result3, Ok("4".to_string()));

    let tokens4 = vec![
        Token::LeftParen,
        Token::Int(10000),
        Token::Plus,
        Token::Int(1),
        Token::RightParen,
        Token::Star,
        Token::LeftParen,
        Token::Int(10),
        Token::Slash,
        Token::Int(5),
        Token::RightParen,
    ];
    let result4 = parse(tokens4);
    assert_eq!(result4, Ok("20002".to_string()));
}

fn safe_eval(s: String) -> Result<String, String> {
    let tokens = tokenizer(s)?;
    let result = parse(tokens)?;
    Ok(result.to_string())
}

#[test]
fn test_safe_eval() {
    let eval1 = safe_eval(String::from("2 + 2"));
    assert_eq!(eval1, Ok("4".to_string()));

    let eval2 = safe_eval(String::from("2 + 2 * 2"));
    assert_eq!(eval2, Ok("6".to_string()));

    let eval3 = safe_eval(String::from("(1 + 1) * 10 / 5"));
    assert_eq!(eval3, Ok("4".to_string()));

    let eval4 = safe_eval(String::from(
        "0+(1+(2+(3+(4+(5+(6+(7+8)))))))-(0+(1+(2+(3+(4+(5+(6+(7+8))))))))",
    ));
    assert_eq!(eval4, Ok("0".to_string()));

    let eval5 = safe_eval(String::from("(2+1030/2)-2"));
    assert_eq!(eval5, Ok("515".to_string()));
}

pub async fn run(options: &[CommandDataOption]) -> String {
    let eval_target = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(eval_target) => eval_target,
                _ => return "計算式を入力してください".to_string(),
            },
            None => return "計算式を入力してください".to_string(),
        },
        None => return "計算式を入力してください".to_string(),
    };

    let result = match safe_eval(eval_target.to_string()) {
        Ok(result) => result,
        Err(err) => err,
    };

    format!("{} = {}", eval_target, result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("eval")
        .description("演算します")
        .create_option(|option| {
            option
                .name("eval")
                .description("計算式")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
