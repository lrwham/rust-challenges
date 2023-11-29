pub fn print_solve(){
    println!("Solving Palindrome Integer");
    println!("131: {}", solve_reverse_method(131));
    println!("1231: {}", solve_reverse_method(1231));
    println!("12321: {}", solve_reverse_method(12321));
    println!("1: {}", solve_reverse_method(1));
    println!("0: {}", solve_reverse_method(0));

}

pub fn solve_reverse_method(x: i32) -> bool {
    let reversed = reverse_digits(x);
    
    x == reversed || x == reversed / 10
}

fn reverse_digits(x: i32) -> i32 {
    let mut y = x;
    let mut reversed = 0;
    while y > 0 {
        reversed = reversed * 10 + y % 10;
        y /= 10;
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrom_integer() {
        assert!(solve_reverse_method(131));
        assert!(!solve_reverse_method(1231));
        assert!(solve_reverse_method(12321));
        assert!(solve_reverse_method(1));
        assert!(solve_reverse_method(0));
        assert!(!solve_reverse_method(10));
    }

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
