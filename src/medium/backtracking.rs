use std::{collections::HashSet, convert::TryInto, str::Chars};

use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let numbers = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        fn letter_combinations(digits: &mut Chars, numbers: &[Vec<char>]) -> Vec<String> {
            match digits.next() {
                Some(x) => {
                    let digit = x.to_digit(10).unwrap() as usize - 2;
                    letter_combinations(digits, numbers)
                        .into_iter()
                        .flat_map(|x| numbers[digit].iter().map(move |y| format!("{}{}", y, x)))
                        .collect()
                }
                None => vec!["".to_string()],
            }
        }
        letter_combinations(&mut digits.chars(), &numbers)
    }
}

#[test]
fn letter_combinations_test() {
    assert_eq!(
        Solution::letter_combinations("23".to_string())
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string()
        ]
        .into_iter()
        .collect::<HashSet<_>>()
    );
    assert_eq!(
        Solution::letter_combinations("".to_string())
            .into_iter()
            .collect::<HashSet<_>>(),
        Vec::<String>::new().into_iter().collect::<HashSet<_>>(),
    );
    assert_eq!(
        Solution::letter_combinations("2".to_string())
            .into_iter()
            .collect::<HashSet<_>>(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
            .into_iter()
            .collect::<HashSet<_>>(),
    )
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn generate_parenthesis(n: i32) -> HashSet<String> {
            if n == 1 {
                return vec!["()".to_string()].into_iter().collect();
            }
            generate_parenthesis(n - 1)
                .into_iter()
                .flat_map(|x| {
                    (0..x.len()).map(move |i| {
                        let mut s = x.clone();
                        s.insert_str(i, "()");
                        s
                    })
                })
                .collect()
        }

        generate_parenthesis(n).into_iter().collect()
    }
}

#[test]
fn generate_parenthesis_test() {
    assert_eq!(
        Solution::generate_parenthesis(3)
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![
            "((()))".to_string(),
            "(()())".into(),
            "(())()".into(),
            "()(())".into(),
            "()()()".into()
        ]
        .into_iter()
        .collect::<HashSet<_>>()
    );
    assert_eq!(
        Solution::generate_parenthesis(4)
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect::<HashSet<_>>()
    );

    assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_string()]);
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn permute(nums: &[i32]) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                return vec![vec![]];
            }
            permute(&nums[1..])
                .into_iter()
                .flat_map(|n| {
                    (0..nums.len()).map(move |i| {
                        let mut n = n.clone();
                        n.insert(i, nums[0]);
                        n
                    })
                })
                .collect()
        }
        permute(&nums)
    }
}

#[test]
fn permute_test() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3])
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(
        Solution::permute(vec![0, 1])
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![vec![0, 1], vec![1, 0],].into_iter().collect()
    );
    assert_eq!(
        Solution::permute(vec![1])
            .into_iter()
            .collect::<HashSet<_>>(),
        vec![vec![1],].into_iter().collect()
    )
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                return vec![vec![]];
            }

            subsets(&nums[1..])
                .into_iter()
                .flat_map(|mut s| {
                    let s1 = s.clone();
                    s.push(nums[0]);
                    vec![s1, s]
                })
                .collect()
        }
        subsets(&nums)
    }
}

#[test]
fn subsets_test() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![2, 1],
            vec![3],
            vec![3, 1],
            vec![3, 2],
            vec![3, 2, 1]
        ]
    )
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(board: &mut [Vec<char>], s: &[char], chard_idx: usize, x: usize, y: usize) -> bool {
            board[x][y] = '_';
            let next_idx = chard_idx + 1;
            match s.get(next_idx) {
                None => true,
                Some(next_char) => {
                    for (i, j) in [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                        let get_legal_move = || {
                            let i: usize = ((x as isize) + i).try_into().ok()?;
                            let j: usize = ((y as isize) + j).try_into().ok()?;
                            if board.get(i)?.get(j)? == next_char {
                                Some((i, j))
                            } else {
                                None
                            }
                        };
                        if let Some((x, y)) = get_legal_move() {
                            if dfs(board, s, next_idx, x, y) {
                                return true;
                            }
                        }
                    }
                    board[x][y] = s[chard_idx];
                    false
                }
            }
        }
        match word.chars().collect::<Vec<_>>().as_slice() {
            [] => false,
            s => {
                let mut board = board;
                for x in 0..board.len() {
                    for y in 0..board[x].len() {
                        if s[0] == board[x][y] && dfs(&mut board, s, 0, x, y) {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }
}

#[test]
fn exist_test() {
    assert!(Solution::exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "ABCCED".into()
    ));
    assert!(Solution::exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "SEE".into()
    ));
    assert!(!Solution::exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "ABCB".into()
    ))
}
