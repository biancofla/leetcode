use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // The simplest solution would be to use two for loops to iterate through
        // all the possible combinations of elements until one that satisfies the
        // target condition is found.
        let mut result: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let sum: i32 = nums[i] + nums[j];

                if sum == target {
                    result.push(i as i32);
                    result.push(j as i32);
                }
            }
        }

        return result;
    }

    pub fn two_sum_enhanced(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // A better solution would be to use a hash map to store each number and
        // its index as the iterations progress. For each number, we would check
        // if its complement exists in the map. If found, we have our solution.
        // Otherwise, we add the current number to the map.
        let mut result: Vec<i32> = Vec::new();

        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement: i32 = target - num;

            // map.get(...) returns an Option<&i32> value. There are two approaches
            // to handle it:
            // 1. the standard match { Some(...) => ..., None => ... } approach.
            // 2. the if let Some(...) == map.get(...) approach.
            // We don't need to handle the default case, so we will stick with the
            // latter.
            if let Some(&j) = map.get(&complement) {
                result.push(j);
                result.push(i as i32);

                break;
            }

            map.insert(num, i as i32);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(
            Solution::two_sum_enhanced(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
        assert_eq!(Solution::two_sum_enhanced(vec![3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
        assert_eq!(Solution::two_sum_enhanced(vec![3, 3], 6), [0, 1]);
    }
}
