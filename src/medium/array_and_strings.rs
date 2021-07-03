use std::collections::{BTreeSet, HashMap};

use crate::Solution;

impl Solution {
    // pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     let mut nums = nums;
    //     nums.sort();
    // }
}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let first_zero_idx = matrix.iter().enumerate().find_map(|(i, x)| {
            x.iter()
                .enumerate()
                .find_map(|(k, y)| if *y == 0 { Some((i, k)) } else { None })
        });
        if let Some((i, k)) = first_zero_idx {
            for j in 0..matrix.len() {
                if matrix[j][k] == 0 {
                    matrix[j][k] = -1;
                } else {
                    matrix[j][k] = 0;
                }
            }

            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][j] = -1;
                } else {
                    matrix[i][j] = 0;
                }
            }

            for j in 0..matrix.len() {
                for l in 0..matrix[j].len() {
                    if j != i && l != k && matrix[j][l] == 0 {
                        matrix[j][k] = -1;
                        matrix[i][l] = -1;
                    }
                }
            }
            for j in 0..matrix.len() {
                if matrix[j][k] == -1 {
                    for l in 0..matrix[j].len() {
                        matrix[j][l] = 0;
                    }
                }
            }
            for j in 0..matrix[0].len() {
                if matrix[i][j] == -1 {
                    for l in 0..matrix.len() {
                        matrix[l][j] = 0;
                    }
                }
            }
        }
    }
}

#[test]
fn set_zeroes_test() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, expected);

    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let expected = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, expected);
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn get_ch_idx(c: char) -> usize {
            ('z' as usize) - (c as usize)
        }

        let strings = strs.iter().map(|s| {
            let mut chars_array = [0; 26];
            for c in s.chars() {
                chars_array[get_ch_idx(c)] += 1;
            }
            (chars_array, s)
        });
        let mut group = HashMap::new();
        for (chars_array, s) in strings {
            group
                .entry(chars_array)
                .or_insert_with(|| vec![])
                .push(s.to_owned())
        }
        group.values().cloned().collect()
    }
}

#[test]
fn group_anagrams_test() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(From::from)
        .collect();
    let expected: BTreeSet<BTreeSet<_>> = vec![
        vec!["bat".to_owned()].into_iter().collect(),
        vec!["nat".to_owned(), "tan".to_owned()]
            .into_iter()
            .collect(),
        vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()]
            .into_iter()
            .collect(),
    ]
    .into_iter()
    .collect();
    let result: BTreeSet<BTreeSet<_>> = Solution::group_anagrams(strs)
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();

    assert_eq!(result, expected);
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h: HashMap<_, usize> = HashMap::new();
        let mut substr_start = 0;
        let mut longest = 0;
        for (i, c) in s.chars().enumerate() {
            match h.entry(c) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    let dub_index = *o.get();
                    if dub_index >= substr_start {
                        longest = std::cmp::max(longest, i - substr_start);
                        substr_start = dub_index + 1;
                    }
                    o.insert(i);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(i);
                }
            }
        }
        std::cmp::max(longest, s.len() - substr_start) as i32
    }
}

#[test]
fn length_of_longest_substring_test() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_owned()),
        3
    );
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_owned()),
        3
    );
    assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
    assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
}

// impl Solution {
//     pub fn longest_palindrome(s: String) -> String {
//         let mid = s.len() / 2;
//         let mut palindrome = String::new();
//     }
// }

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        false
    }
}

#[test]
fn increasing_triplet_test() {
    // assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    // assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    // assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    // assert!(!Solution::increasing_triplet(vec![0, 4, 2, 1, 0, -1, -3]));
    assert!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]));
}
