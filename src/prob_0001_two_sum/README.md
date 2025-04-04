# Two Sum

[Back to root-level README](../../README.md)

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have _**exactly one solution**_, and you may not use the same element twice.

You can return the answer in any order.

## Examples

### Example 1

```console
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

### Example 2

```console
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

### Example 3

```console
Input: nums = [3,3], target = 6
Output: [0,1]
``` 

## Constraints

- 2 <= nums.length <= 10<sup>4</sup>
- -10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>
- -10<sup>9</sup> <= target <= 10<sup>9</sup>
- Only one valid answer exists.
 
**Follow-up:** Can you come up with an algorithm that is less than O(n<sup>2</sup>) time complexity?