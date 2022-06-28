pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // // Calculate the factorial - this is how long the returned vector will need to be.
        let nums_len = nums.len() as i32;
        let result_len = self::Solution::compute_factorial(nums_len);

        let mut permutations: Vec<Vec<i32>> =
            vec![Vec::with_capacity(nums_len as usize); result_len as usize];
        println!("permutations: {:?}", permutations);

        let group_size = self::Solution::compute_factorial(nums_len - 1) as usize;
        println!("group_size: {:?}", group_size);
        let mut num_index = 0;
        for (index, vec) in permutations.iter_mut().enumerate() {
            println!("index: {:?}", index);
            println!("num_index: {:?}", num_index);

            vec.push(nums[num_index]); // append(&mut nums[num_index]); // insert(num_index, nums[num_index]);

            if (index+1) % group_size == 0 {
                num_index += 1;
            }
        }

        println!("{:?}", permutations);

        // let mut col = 0;
        // let mut group = 0;
        // for num in nums.iter() {
        //     for row in 0..result_len {
        //         let cell = permutations.get_mut(row as usize).unwrap();
        //         cell[col as usize] = *num;
        //     }
        //     col += 1;
        // }

        // println!("{:?}", permutations);
        permutations
    }

    fn permutate(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            1 => return vec![vec![nums[0]]],
            2 => return vec![vec![nums[0], nums[1]], vec![nums[1], nums[0]]],
            _ => {
                let mut result = vec![nums[0]];

                return vec![result];
            }
        }
    }

    fn compute_factorial(num: i32) -> i32 {
        let mut factorial = 1;
        for x in 1..num + 1 {
            factorial *= x;
        }
        factorial
    }
}

// input: 1 2 3 4
// 1 2 3 4
// 1 2 4 3
// 1 3 2 4
// 1 3 4 2
// 1 4 2 3
// 1 4 3 2
// for num_permutations
//   for col
//
//
//
// col_1_occurrence = factorial(nums-1)
// col_2_occurrence = factorial(nums-2)
// col_3_occurrence = factorial(nums-3)

#[cfg(test)]
mod tests {
    use crate::permutations::Solution;

    #[test]
    fn compute_factorial_works() {
        assert_eq!(Solution::compute_factorial(1), 1);
        assert_eq!(Solution::compute_factorial(2), 2);
        assert_eq!(Solution::compute_factorial(3), 6);
        assert_eq!(Solution::compute_factorial(4), 24);
        assert_eq!(Solution::compute_factorial(5), 120);
        assert_eq!(Solution::compute_factorial(6), 720);
    }

    #[test]
    fn solution_works() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
        assert_eq!(Solution::permute(vec![1,2]), vec![vec![1,2], vec![2,1]]);
        //assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
        //assert_eq!(Solution::permute(vec![1,2]), vec![vec![1,2], vec![2,1]]);
    }
}
