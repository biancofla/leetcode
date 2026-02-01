pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;

        // Example:
        // l1 = [1] -> [3] -> None
        // l2 = [2] -> [4] -> None
        // result = [-1] -> None
        // tail ------^
        let mut result = Box::new(ListNode::new(-1));
        let mut tail = &mut result;

        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap().val;
            let v2 = l2.as_ref().unwrap().val;

            if v1 <= v2 {
                // CASE v1 <= v2.
                //
                // l1 = [1] -> [3] -> None
                //
                // After tail.next = l1.take():
                //      l1        = None
                //      tail.next =  [1] -> [3] -> None
                //
                // The line l1 = tail.next.as_mut().unwrap().next.take(), does the
                // following:
                //      tail           = [-1] -> [1] -> [3] -> None
                //      tail.next      =  [1] -> [3] -> None
                //      tail.next.next =  [3] -> None
                // tail.next.next is then extracted using the take function, which
                // leaves:
                //      tail.next.next = None
                //      tail.next      =  [1] -> None
                //      tail           = [-1] -> [1] -> None
                //      l1             =  [3] -> None
                //
                // REPEAT FOR ALL ITERATIONS.
                tail.next = l1.take();

                l1 = tail.next.as_mut().unwrap().next.take();
            } else {
                // CASE v1 > v2.
                //
                // Same as above with l2.
                //
                // REPEAT FOR ALL ITERATIONS.
                tail.next = l2.take();

                l2 = tail.next.as_mut().unwrap().next.take();
            }

            // At this point, we shift tail at the next node:
            // result = [-1] -> [1] -> None
            // tail -------------^
            //
            // REPEAT FOR ALL ITERATIONS.
            tail = tail.next.as_mut().unwrap();
        }

        // At the end of the loop:
        // l1 = None
        // l2 = [4] -> None
        // result = [-1] -> [1] -> [2] -> [3] -> None
        // tail ---------------------------^
        //
        // We attach the remaining node to result:
        // result = [-1] -> [1] -> [2] -> [3] -> [4] -> None
        // tail ----------------------------------^
        tail.next = l1.or(l2);

        // We return the result without the initial [-1] node.
        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        // Example:
        // values = [1, 2, 3]
        //
        // 1st Iteration:
        //      node      = ListNode::new(val: 3)
        //      node.next = None
        //      head      = [3] -> None
        //
        // 2nd Iteration:
        //      node      = ListNode::new(val: 2)
        //      node.next = [3] -> None
        //      head      = [2] -> [3] -> None
        //
        // 3rd Iteration:
        //      node      = ListNode::new(val: 1)
        //      node.next = [2] -> [3] -> None
        //      head      = [1] -> [2] -> [3] -> None
        for &val in values.iter().rev() {
            let mut node: ListNode = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }

        head
    }

    #[test]
    fn test_example_1() {
        let list1 = to_list(&[1, 2, 4]);
        let list2 = to_list(&[1, 3, 4]);
        let result = to_list(&[1, 1, 2, 3, 4, 4]);

        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }

    #[test]
    fn test_example_2() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[]);
        let result = to_list(&[]);

        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }

    #[test]
    fn test_example_3() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[0]);
        let result = to_list(&[0]);

        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }
}
