use crate::Solution;
use std::{cell::RefCell, collections::HashMap};
use std::{convert::TryInto, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_iterator(
        iter: impl IntoIterator<Item = Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = iter
            .into_iter()
            .map(|x| Some(Rc::new(RefCell::new(TreeNode::new(x?)))))
            .collect::<Vec<_>>();

        for i in (1..nodes.len()).step_by(2) {
            let mut parent_idx = ((i + 1) / 2) - 1;
            if nodes[parent_idx].is_none() {
                parent_idx += 1;
            }
            nodes[parent_idx].as_mut().unwrap().borrow_mut().left = nodes[i].clone();
            nodes[parent_idx].as_mut().unwrap().borrow_mut().right =
                nodes.get(i + 1).cloned().flatten();
        }
        nodes.into_iter().next().and_then(|x| x)
    }
}

#[test]
fn from_iterator_test() {
    dbg!(TreeNode::from_iterator([
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7)
    ]));
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn rec(root: Option<&RefCell<TreeNode>>, result: &mut Vec<i32>) {
            match root {
                Some(x) => {
                    let node = x.borrow();
                    rec(node.left.as_deref(), result);
                    result.push(node.val);
                    rec(node.right.as_deref(), result)
                }
                None => {}
            }
        }
        let mut result = vec![];
        rec(root.as_deref(), &mut result);
        result
    }
}

#[test]
fn inorder_traversal_test() {
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_iterator([Some(1), None, Some(2), Some(3)])),
        vec![1, 3, 2]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_iterator([])),
        vec![]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_iterator([Some(1), Some(2)])),
        vec![2, 1]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_iterator([Some(1), None, Some(2)])),
        vec![1, 2]
    );
}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn zigzag(root: Option<&RefCell<TreeNode>>, result: &mut Vec<Vec<i32>>, level: usize) {
            match root {
                Some(x) => {
                    if result.len() <= level {
                        result.push(vec![])
                    }
                    let x = x.borrow();
                    zigzag(x.left.as_deref(), result, level + 1);
                    result[level].push(x.val);
                    zigzag(x.right.as_deref(), result, level + 1);
                }
                None => {}
            }
        }
        let mut resp = Vec::new();
        zigzag(root.as_deref(), &mut resp, 0);
        for (i, x) in resp.iter_mut().enumerate() {
            if i % 2 != 0 {
                x.reverse()
            }
        }
        resp
    }
}

#[test]
fn zigzag_level_order_test() {
    assert_eq!(
        Solution::zigzag_level_order(TreeNode::from_iterator([
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7)
        ])),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
    assert_eq!(
        Solution::zigzag_level_order(TreeNode::from_iterator([
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5)
        ])),
        vec![vec![1], vec![3, 2], vec![4, 5]]
    );
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder;
        let inorder_idx = inorder
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i as isize))
            .collect::<HashMap<_, _>>();
        preorder.reverse();
        fn build_tree(
            preorder: &mut Vec<i32>,
            inorder_idx: &HashMap<i32, isize>,
            left: isize,
            right: isize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match preorder.last().copied() {
                Some(_x) if left <= right => {
                    let x = preorder.pop().unwrap();
                    let mut root = TreeNode::new(x);
                    root.left = build_tree(preorder, inorder_idx, left, inorder_idx[&x] - 1);
                    root.right = build_tree(preorder, inorder_idx, inorder_idx[&x] + 1, right);
                    Some(Rc::new(RefCell::new(root)))
                }
                Some(_x) => None,
                None => None,
            }
        }
        build_tree(
            &mut preorder,
            &inorder_idx,
            0 as isize,
            inorder.len() as isize - 1,
        )
    }
}

#[test]
fn build_tree_test() {
    assert_eq!(
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        TreeNode::from_iterator([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    )
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn find(root: Option<&RefCell<TreeNode>>, k: usize, cur_idx: &mut usize) -> Option<i32> {
            match root {
                Some(root) => {
                    let root = root.borrow();
                    find(root.left.as_deref(), k, cur_idx).or_else(|| {
                        *cur_idx += 1;
                        if *cur_idx == k {
                            Some(root.val)
                        } else {
                            find(root.right.as_deref(), k, cur_idx)
                        }
                    })
                }
                None => None,
            }
        }
        let mut idx = 0;
        find(root.as_deref(), k as usize, &mut idx).unwrap()
    }
}

#[test]
fn kth_smallest_test() {
    assert_eq!(
        Solution::kth_smallest(
            TreeNode::from_iterator([Some(3), Some(1), Some(4), None, Some(2)]),
            1
        ),
        1
    );
    assert_eq!(
        Solution::kth_smallest(
            TreeNode::from_iterator([
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                None,
                Some(1)
            ]),
            3
        ),
        3
    );
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let moves = [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)];
        let mut visited = grid
            .iter()
            .map(|x| x.iter().map(|_| false).collect())
            .collect::<Vec<Vec<_>>>();
        let mut stack = Vec::new();
        let mut islands = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if point == &'1' && !visited[i][j] {
                    islands += 1;
                    stack.push((i, j));

                    while let Some((i, j)) = stack.pop() {
                        moves
                            .iter()
                            .filter_map(|(x, y)| {
                                let i: usize = ((i as isize) + x).try_into().ok()?;
                                let j: usize = ((j as isize) + y).try_into().ok()?;
                                if grid.get(i)?.get(j)? == &'1' && !visited[i][j] {
                                    visited[i][j] = true;
                                    Some((i, j))
                                } else {
                                    None
                                }
                            })
                            .for_each(|x| stack.push(x))
                    }
                }
            }
        }
        islands
    }
}

#[test]
fn num_islands_test() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(Solution::num_islands(grid), 1);

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(Solution::num_islands(grid), 3);

    let grid = vec![
        vec![
            '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1',
            '0', '1', '1',
        ],
        vec![
            '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1',
            '1', '1', '0',
        ],
        vec![
            '1', '0', '1', '1', '1', '0', '0', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '0', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '0', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '0', '1', '1', '1', '0',
            '1', '1', '1',
        ],
        vec![
            '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '0', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1',
            '0', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '0', '1', '1', '1', '1', '1', '0', '1', '1', '1', '0', '1', '1', '1', '1', '0',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1',
            '1', '1', '0',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1',
            '1', '0', '0',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
        vec![
            '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
            '1', '1', '1',
        ],
    ];
    assert_eq!(Solution::num_islands(grid), 1)
}
