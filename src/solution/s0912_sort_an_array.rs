use std::fmt::Debug;
use std::mem::swap;

/**
 * [912] Sort an Array
 *
 * Given an array of integers nums, sort the array in ascending order.
 *  
 * Example 1:
 * Input: nums = [5,2,3,1]
 * Output: [1,2,3,5]
 * Example 2:
 * Input: nums = [5,1,1,2,0,0]
 * Output: [0,0,1,1,2,5]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 50000
 * 	-50000 <= nums[i] <= 50000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-an-array/
// discuss: https://leetcode.com/problems/sort-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.to_vec();
        SelectionSort.sort(result.as_mut_slice());
        result
    }
}

// submission codes end
pub trait Sorter {
    fn sort<T>(&self, _: &mut [T])
    where
        T: Ord + Debug;
}

// bubble sort
pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Debug,
    {
        let mut done = false;
        while !done {
            done = true;
            // if we check every element in the slice and find nothing to swap, done is marked as true
            // otherwise, done is marked as false, and go to another round of loop.
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    done = false
                }
            }
        }
    }
}

pub struct InsertionSort {
    pub use_binary_search: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Debug,
    {
        // the slice is organized as [ sorted | unsorted ].
        // take each elem in unsorted area and place it in the right position in sort area
        // until all elem is sorted.
        for unsorted in 1..slice.len() {
            if !self.use_binary_search {
                // native way without using binary search.
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find the right position and rotate right.
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Debug,
    {
        for unsorted in 0..slice.len() {
            let smallest_in_unsorted = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("not empty");
            if smallest_in_unsorted != unsorted {
                slice.swap(smallest_in_unsorted, unsorted);
            }
        }
    }
}

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Debug,
    {
        // [ unsorted | pivot | unsorted ]
        q_sort(slice);
    }
}

// quick sort in place sort.
fn q_sort<T: Ord + Debug>(slice: &mut [T]) {
    // check termination.
    match slice.len() {
        0 | 1 => return,
        2 => {
            // really simply case, swap and return is fine.
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    // find pivot.
    // let (pivot, remain) = slice.split_first_mut().expect("should not empty");
    let mut left = 1; // track idx.
    let mut right = slice.len() - 1; // track idx.

    // find the pivot using double pointer.
    while left < right {
        // check and update left.
        if slice[left] <= slice[0] {
            left += 1;
        } else {
            slice.swap(left, right);
            right -= 1;
        }
        if left == right {
            break;
        }
        // check and update right.
        if slice[right] >= slice[0] {
            right -= 1;
        } else {
            slice.swap(left, right);
            left += 1;
        }
    }
    // at this point, left and right equals.
    // the element they point to is not cpmpared with pivot yet.
    // we need to check it here
    let mut pivot_idx = left;
    if slice[pivot_idx] < slice[0] {
        slice.swap(pivot_idx, 0);
    } else {
        pivot_idx -= 1;
        slice.swap(pivot_idx, 0);
    }
    // sort on sub slice.
    // eprintln!("slice {:?} p_idx {:?}", slice, pivot_idx);
    let (l_slice, r_slice) = slice.split_at_mut(pivot_idx);
    q_sort(l_slice);
    q_sort(&mut r_slice[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_912() {
        assert_eq!(
            Solution::sort_array(vec![5, 2, 3, 4, 1]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            Solution::sort_array(vec![5, 1, 3, 2, 4, 0]),
            vec![0, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_912_bubble_sort() {
        let mut nums = vec![4, 2, 5, 3, 1];
        BubbleSort.sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_912_insertion_sort() {
        {
            let mut nums = vec![5, 2, 3, 4, 1];
            InsertionSort {
                use_binary_search: false,
            }
            .sort(&mut nums);
            assert_eq!(nums, vec![1, 2, 3, 4, 5]);
        }
        {
            let mut nums = vec![5, 2, 3, 4, 1];
            InsertionSort {
                use_binary_search: true,
            }
            .sort(&mut nums);
            assert_eq!(nums, vec![1, 2, 3, 4, 5]);
        }
    }
    #[test]
    fn test_912_selection_sort() {
        let mut nums = vec![5, 2, 3, 4, 1];
        SelectionSort.sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_912_quick_sort() {
        let mut nums = vec![5, 1, 1, 2, 0, 0];
        QuickSort.sort(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 5]);
    }
}
