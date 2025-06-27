mod search;


#[cfg(test)]
mod tests {
    use super::*;
    use search::search;
    
    #[test]
    fn one_result() {
        let query = "bog";
        let contents = "poem.txt";

let expected = vec!["to an admiring bog!".to_string()];
let result = search(query, contents);
assert_eq!(result.unwrap(), expected);
    }
}