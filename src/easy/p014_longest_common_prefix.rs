pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // 1. Find the smallest string.
        let mut smallest_str: &String = &String::new();
        for str in &strs {
            if smallest_str.is_empty() || str.len() < smallest_str.len() {
                smallest_str = str;
            }
        }

        // 2. Build the common prefix.
        let mut longest_common_prefix: String = String::new();
        for (i, c) in smallest_str.chars().enumerate() {
            for str in &strs {
                if Some(c) != str.chars().nth(i) {
                    return longest_common_prefix;
                }
            }
            longest_common_prefix.push(c);
        }

        longest_common_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(Solution::longest_common_prefix(input), "fl");
    }

    #[test]
    fn test_example_2() {
        let input: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        assert_eq!(Solution::longest_common_prefix(input), "");
    }
}
