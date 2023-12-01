pub fn print_solve(){
    println!("Roman Numeral to Integer");
    println!("IV: {}", convert("IV"));
    println!("CD: {}", convert("CD"));
    println!("CM: {}", convert("CM"));
    println!("MMCMXCIV: {}", convert("MMCMXCIV"));
}

// Search for subtraction patterns first and remove them
//
// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.
//
// Find and extract IV, IX, XL, XC, CD, and CM
pub fn convert(s: &str) -> i32{
    let mut s: String = s.to_owned();
    let mut value = 0;

    let patterns = vec![("IV", 4), ("IX", 9), ("XL", 40), ("XC", 90), ("CD", 400), ("CM", 900),("I",1),("V",5),("X",10),("L",50),("C",100),("D",500),("M",1000)];

    for pattern in patterns.iter(){
        while s.contains(pattern.0){
            value += pattern.1;
            s = s.replacen(pattern.0, "", 1);
        }
    }

    value
}


#[cfg(test)]
mod tests {
    use crate::challenges::roman_numeral_to_integer;

    #[test]
    fn test_roman_to_i(){
        assert_eq!(roman_numeral_to_integer::convert("IV"),4);
        assert_eq!(roman_numeral_to_integer::convert("IX"),9);
        assert_eq!(roman_numeral_to_integer::convert("XL"),40);
        assert_eq!(roman_numeral_to_integer::convert("XC"),90);
        assert_eq!(roman_numeral_to_integer::convert("CD"),400);
        assert_eq!(roman_numeral_to_integer::convert("CM"),900);
        assert_eq!(roman_numeral_to_integer::convert("MCMXCIV"),1994);
        assert_eq!(roman_numeral_to_integer::convert("MMMCMXCIX"),3999);
    }
}