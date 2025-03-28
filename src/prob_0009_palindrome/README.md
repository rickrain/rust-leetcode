# Palindrome Number

[Back to root-level README](../../README.md)

Given an integer `x`, return true if `x` is palindrome integer.

An integer is a **palindrome** when it reads the same backward as forward. For example, `121` is a palindrome while `123` is not.

## Examples

### Example 1

```console
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
```

### Example 2

```console
Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
```

### Example 3

```console
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
```

## Constraints

- -2<sup>31</sup> <= x <= 2<sup>31</sup> - 1