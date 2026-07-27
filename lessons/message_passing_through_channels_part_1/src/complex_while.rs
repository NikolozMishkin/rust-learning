//! Сложные варианты `while` на примере калькулятора выражений.
//!
//! Здесь `while` используется в четырёх разных ролях, и ни в одной из них
//! `for` не подошёл бы: число итераций зависит от состояния, а не от длины
//! коллекции.
//!
//! 1. `'outer: while let ...` + `break 'outer` — выход через два уровня вложенности.
//! 2. `while let Some(&c) = chars.peek()` — цикл по `Peekable` без потребления символа.
//! 3. `while matches!(stack.last(), Some(..) if guard)` — условие с паттерном и guard-ом.
//! 4. `while let Some(top) = stack.pop()` — «дренаж» коллекции, которую мутируем.

use std::collections::HashMap;

#[derive(Debug)]
enum Token {
    Num(i64),
    Op(char),
    Open,
    Close,
}

/// Токенизатор: `while let` по итератору + вложенный `while` для многозначных чисел.
fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let mut out = Vec::new();
    let mut chars = src.chars().peekable();

    'outer: while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
                continue 'outer;
            }
            '0'..='9' => {
                let mut n: i64 = 0;
                // условие цикла = «следующий символ вообще цифра?»
                while let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
                    n = match n.checked_mul(10).and_then(|n| n.checked_add(d as i64)) {
                        Some(v) => v,
                        // без метки break вышел бы только из внутреннего while,
                        // и парсер молча продолжил бы с испорченным числом
                        None => break 'outer,
                    };
                    chars.next();
                }
                out.push(Token::Num(n));
            }
            '+' | '-' | '*' | '/' => {
                out.push(Token::Op(c));
                chars.next();
            }
            '(' => {
                out.push(Token::Open);
                chars.next();
            }
            ')' => {
                out.push(Token::Close);
                chars.next();
            }
            other => return Err(format!("неизвестный символ: {other:?}")),
        }
    }

    // если сюда попали не «естественно», а через break 'outer — вход не дочитан
    if chars.peek().is_some() {
        return Err("число слишком большое, разбор прерван".to_string());
    }
    Ok(out)
}

/// Shunting-yard: `while` с условием на верхушку стека.
fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let prec: HashMap<char, u8> = [('+', 1), ('-', 1), ('*', 2), ('/', 2)].into_iter().collect();
    let mut out: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for tok in tokens {
        match tok {
            Token::Num(_) => out.push(tok),
            Token::Op(op) => {
                // matches! с guard-ом прямо в условии while: одновременно
                // читаем верхушку стека и pop-аем её в теле — for тут невозможен
                while matches!(stack.last(), Some(Token::Op(top)) if prec[top] >= prec[&op]) {
                    out.push(stack.pop().unwrap());
                }
                stack.push(Token::Op(op));
            }
            Token::Open => stack.push(Token::Open),
            Token::Close => {
                let mut matched = false;
                while let Some(top) = stack.pop() {
                    if matches!(top, Token::Open) {
                        matched = true;
                        break;
                    }
                    out.push(top);
                }
                if !matched {
                    return Err("лишняя закрывающая скобка".into());
                }
            }
        }
    }

    // «слив» остатка стека: с `for x in &stack` borrow checker такое не пропустит
    while let Some(top) = stack.pop() {
        if matches!(top, Token::Open) {
            return Err("незакрытая скобка".into());
        }
        out.push(top);
    }
    Ok(out)
}

/// Обычный `while` со счётчиком — выход из середины через `?`.
fn eval_rpn(rpn: &[Token]) -> Result<i64, String> {
    let mut st: Vec<i64> = Vec::new();
    let mut i = 0;
    while i < rpn.len() {
        match rpn[i] {
            Token::Num(n) => st.push(n),
            Token::Op(op) => {
                let b = st.pop().ok_or("не хватает операнда")?;
                let a = st.pop().ok_or("не хватает операнда")?;
                st.push(match op {
                    '+' => a.checked_add(b).ok_or("переполнение")?,
                    '-' => a.checked_sub(b).ok_or("переполнение")?,
                    '*' => a.checked_mul(b).ok_or("переполнение")?,
                    '/' if b == 0 => return Err("деление на ноль".into()),
                    '/' => a / b,
                    _ => unreachable!(),
                });
            }
            _ => return Err("скобка в RPN".into()),
        }
        i += 1;
    }

    let top = st.pop().ok_or("пустое выражение")?;
    if !st.is_empty() {
        return Err("некорректное выражение".into());
    }
    Ok(top)
}

pub fn calc(src: &str) -> Result<i64, String> {
    eval_rpn(&to_rpn(tokenize(src)?)?)
}

pub fn demo() {
    let cases = [
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "100 / (2 + 3) - 7 * (1 + 1)",
        "2 + (3 * 4",
        "10 / 0",
        "1 + 99999999999999999999",
        "2 $ 3",
    ];

    println!("--- complex_while::demo ---");
    for src in cases {
        match calc(src) {
            Ok(v) => println!("{src:>32} = {v}"),
            Err(e) => println!("{src:>32} ! {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn precedence_and_parens() {
        assert_eq!(calc("2 + 3 * 4"), Ok(14));
        assert_eq!(calc("(2 + 3) * 4"), Ok(20));
        assert_eq!(calc("100 / (2 + 3) - 7 * (1 + 1)"), Ok(6));
    }

    #[test]
    fn errors() {
        assert!(calc("2 + (3 * 4").is_err());
        assert!(calc("2 + 3) * 4").is_err());
        assert!(calc("10 / 0").is_err());
        assert!(calc("1 + 99999999999999999999").is_err());
        assert!(calc("2 $ 3").is_err());
    }
}
