use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // 1. Map Roman characters to integer values.
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let s_splitted: Vec<char> = s.chars().collect();

        let mut n: Vec<i32> = Vec::new();
        for c in s_splitted.iter() {
            if let Some(&value) = map.get(&c) {
                n.push(value);
            }
        }

        // 2. If n[i] < n[i + 1], it means that the value must
        // be subtracted. If so, n[i] becomes -n[i].
        for i in 0..n.len() - 1 {
            if n[i] < n[i + 1] {
                n[i] = -n[i];
            }
        }

        // 3. Sum values.
        n.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
