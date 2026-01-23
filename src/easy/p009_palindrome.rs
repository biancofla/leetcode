pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // 0. Check if x is a negative number. Negative numbers can't
        // be palindrome because of the leading '-' sign.
        if x < 0 {
            return false;
        }

        // 1. Trasform number to string.
        let s = x.to_string();

        // 2. Check if string is equal to reversed string.
        s.chars().eq(s.chars().rev())
    }

    pub fn is_palindrome_no_string(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut copy: i32 = x;
        let mut reversed: i32 = 0;

        while copy > 0 {
            // NOTE: to understand the following lines, we firstly need to
            // understand what the modulo operation does.
            //
            // The '% 10' operation extracts the last digit from any number.
            // Here are some example:
            // 1234 % 10 = 4
            // 123  % 10 = 3
            // 12   % 10 = 2
            // 1    % 10 = 1
            //
            // The '/ 10' operation removes the last digit:
            // 1234 / 10 = 123
            // 123  / 10 = 12
            // 12   / 10 = 1
            // 1    / 10 = 0
            //
            // By combining these operations, we extract digits one by one
            // from right to left and build the reversed number.
            //
            // Example:
            //  - n_copy     = 1234
            //  - n_reversed = 0
            //
            // 1st iteration.
            // 1) n_reversed = 0 * 10 + (1234 % 10) = 4
            // 2) n_copy = 123
            //
            // 2nd iteration.
            // 1) n_reversed = 4 * 10 + (123 % 10) = 43
            // 2) n_copy = 12
            //
            // 3rd iteration.
            // 1) n_reversed = 43 * 10 + (12 % 10) = 432
            // 2) n_copy = 1
            //
            // 4th iteration.
            // 1) n_reversed = 432 * 10 + (1 % 10) = 4321
            // 2) n_copy = 0
            //
            // The while loop ends with n_reversed equal to 4321, which is
            // the reverse of n.
            reversed = reversed * 10 + (copy % 10);
            copy /= 10;
        }

        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome_no_string(121), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome_no_string(-121), false);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome_no_string(10), false);
    }
}
