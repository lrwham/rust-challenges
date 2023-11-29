pub fn print_solve(){
    println!("atoi");
    println!("123: {}", atoi("123".to_string()));
    println!("   123: {}", atoi("   123 as 123".to_string()));
}

pub fn atoi(x: String) -> i32 {
    let x = x.trim();
    let mut sign = 1;
    let mut result: i32 = 0;

    let mut allow_sign = true;

    for c in x.chars() {
        if c == ' ' && allow_sign{
            continue;
        }
        if c == '-' && allow_sign{
            sign = -1;
            allow_sign = false;
        } else if c == '+' && allow_sign{
            sign = 1;
            allow_sign = false;
        } else if c.is_digit(10) {
            allow_sign = false;
            result = result.saturating_mul(10).saturating_add(c.to_digit(10).unwrap() as i32 * sign);
        } else {
            return result
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi() {
        assert_eq!(atoi("123".to_string()), 123);
        assert_eq!(atoi("   123 as 123".to_string()), 123);
        assert_eq!(atoi("   -123 as 123".to_string()), -123);
        assert_eq!(atoi("   +123 as 123".to_string()), 123);
    }
}