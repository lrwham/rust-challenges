pub fn print_solve(){
    println!("Longest Common Prefix");
    println!("[\"apple\",\"apply\",\"ape\"] : {}", find(vec![format!("apple"),format!("apply"),format!("ape")]));
}
// find the shortest word in the vector
// check if shortest word is a prefix of all other words
// pop last character if not, check again, repeat
pub fn find(strings: Vec<String>) -> String{
    let mut shortest: String = match strings.iter().min_by_key(|s| s.len()){
        Some(s) => s.to_string(),
        None => return "".to_owned()
    };

    'outer: loop{
        for s in &strings{
            if s.starts_with(&shortest) {
                continue
            } else {
                shortest = shortest[0..shortest.len()-1].to_owned();
                continue 'outer;
            }
        }

        return shortest.to_owned();
    }
    
}

#[cfg(test)]
mod tests {
    use crate::challenges::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix(){
        assert_eq!(longest_common_prefix::find(vec![format!("apple"),format!("apply"),format!("ape")]),"ap");
        assert_eq!(longest_common_prefix::find(vec![format!("flower"),format!("flow"),format!("flight")]),"fl");
    }
}