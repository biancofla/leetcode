pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        // We use a stack to keep track of the parentheses order.
        // 1. If c is equal to '(', '[', or '{', we push its value into the stack.
        // 2. If c is equal to ')', ']', or '}', we check whether the last element
        //    in the stack is the corresponding opening parenthesis. If this isn't
        //    the case, we return false.
        // 3. Finally, the stack must be empty. If not, there are unclosed opening
        //    parentheses (e.g., "(((" or "({[" would fail otherwise.)
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
