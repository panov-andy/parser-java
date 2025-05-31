fn main() {
    // Тестирование функции digit0 на различных входных данных
    let test_cases = vec!["123abc", "abc123", "abc123def", "123", "abc", ""];

    println!("Тестирование функции digit0:");
    for test in test_cases {
        match digit0(test) {
            Ok((rest, digits)) => {
                println!("Вход: '{}', Цифры: '{}', Остаток: '{}'", test, digits, rest)
            }
            Err(e) => println!("Ошибка при обработке '{}': {:?}", test, e),
        }
    }
}

pub type IResult<I, O, E = Err<String>> = Result<(I, O), Err<E>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Err<TError> {
    Error(TError),
}

/// Recognizes zero or more ASCII numerical characters: 0-9
///
/// ```
/// # use crate::{digit0, IResult, Err};
/// 
/// fn parser(input: &str) -> IResult<&str, &str> {
///     digit0(input)
/// }
///
/// assert_eq!(parser("21c"), Ok(("c", "21")));
/// assert_eq!(parser("21"), Ok(("", "21")));
/// assert_eq!(parser("a21c"), Ok(("a21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
pub fn digit0(input: &str) -> IResult<&str, &str> {
    let mut i = 0;
    let input_bytes = input.as_bytes();

    // Находим позицию первого символа, который не является цифрой
    while i < input_bytes.len() {
        let c = input_bytes[i];
        if !(c >= b'0' && c <= b'9') {
            break;
        }
        i += 1;
    }

    // Разделяем строку на две части: цифры и остаток
    let (digits, rest) = input.split_at(i);
    Ok((rest, digits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit0_examples() {
        // Тесты из документации
        assert_eq!(digit0("21c"), Ok(("c", "21")));
        assert_eq!(digit0("21"), Ok(("", "21")));
        assert_eq!(digit0("a21c"), Ok(("a21c", "")));
        assert_eq!(digit0(""), Ok(("", "")));
    }
}