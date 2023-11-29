pub fn print_solve(){
    println!("123456789: {}", reverse_digits(123456789));   
}

fn reverse_digits(x: i32) -> i32 {

    let sign = if x < 0 { -1 } else { 1 };
    let mut reversed = 0;
    let mut x = x * sign;

    while x > 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    reversed * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_digits() {
        assert_eq!(reverse_digits(123), 321);
        assert_eq!(reverse_digits(1), 1);
        assert_eq!(reverse_digits(0), 0);
        assert_eq!(reverse_digits(10), 1);
        assert_eq!(reverse_digits(12321), 12321);
        assert_eq!(reverse_digits(123456789), 987654321);
    }
}