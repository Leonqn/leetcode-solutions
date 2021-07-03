use crate::Solution;

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

    fn from_iter(i: impl IntoIterator<Item = i32>) -> Option<Box<Self>> {
        let mut iter = i.into_iter();
        let mut head = Box::new(ListNode::new(iter.next()?));
        let mut current = head.as_mut();
        for x in iter {
            current.next = Some(Box::new(ListNode::new(x)));
            current = current.next.as_mut().unwrap()
        }
        Some(head)
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn sum(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            mem: i32,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (None, None) => {
                    if mem == 1 {
                        Some(Box::new(ListNode::new(1)))
                    } else {
                        None
                    }
                }
                (None, Some(l)) => {
                    let result = l.val + mem;
                    let mut node = ListNode::new(result % 10);
                    node.next = sum(None, l.next, result / 10);
                    Some(Box::new(node))
                }
                (Some(l), None) => {
                    let result = l.val + mem;
                    let mut node = ListNode::new(result % 10);
                    node.next = sum(None, l.next, result / 10);
                    Some(Box::new(node))
                }
                (Some(l1), Some(l2)) => {
                    let result = l1.val + l2.val + mem;
                    let mut node = ListNode::new(result % 10);
                    node.next = sum(l1.next, l2.next, result / 10);
                    Some(Box::new(node))
                }
            }
        }
        sum(l1, l2, 0)
    }
}

#[test]
fn add_two_numbers_test() {
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::from_iter([2, 4, 3]),
            ListNode::from_iter([5, 6, 4])
        ),
        ListNode::from_iter([7, 0, 8])
    );
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::from_iter([9, 9, 9, 9, 9, 9, 9]),
            ListNode::from_iter([9, 9, 9, 9])
        ),
        ListNode::from_iter([8, 9, 9, 9, 0, 0, 0, 1])
    );
}

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut even_head = head.clone().and_then(|x| x.next);
        let mut odd = head.as_mut();
        let mut even = even_head.as_mut();

        loop {
            match (odd, even) {
                (None, None) => return head,
                (None, Some(_x)) => {
                    panic!("I think this is impossible")
                }
                (Some(x), None) => {
                    x.next = even_head;
                    return head;
                }
                (Some(x), Some(y)) => {
                    x.next = x.next.as_mut().and_then(|x| x.next.take());
                    if x.next.is_some() {
                        y.next = y.next.as_mut().and_then(|y| y.next.take());
                    } else {
                        x.next = even_head;
                        return head;
                    }

                    odd = x.next.as_mut();
                    even = y.next.as_mut();
                }
            }
        }
    }
}

#[test]
fn odd_even_list_test() {
    assert_eq!(
        Solution::odd_even_list(ListNode::from_iter([1, 2, 3, 4, 5])),
        ListNode::from_iter([1, 3, 5, 2, 4])
    );
    assert_eq!(
        Solution::odd_even_list(ListNode::from_iter([2, 1, 3, 5, 6, 4, 7])),
        ListNode::from_iter([2, 3, 6, 7, 1, 5, 4])
    );
    assert_eq!(
        Solution::odd_even_list(ListNode::from_iter([1, 2, 3, 4, 5, 6, 7, 8])),
        ListNode::from_iter([1, 3, 5, 7, 2, 4, 6, 8])
    )
}
