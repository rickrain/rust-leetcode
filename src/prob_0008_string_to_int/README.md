# String to Integer (atoi)

[Back to root-level README](../../README.md)

Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer (similar to C/C++'s `atoi` function).

The algorithm for `myAtoi(string s)` is as follows:

1. Read in and ignore any leading whitespace.
2. Check if the next character (if not already at the end of the string) is `'-'` or `'+'`. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
3. Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
4. Convert these digits into an integer (i.e. `"123"-> 123`, `"0032" -> 32`). If no digits were read, then the integer is `0`. Change the sign as necessary (from step 2).
5. If the integer is out of the 32-bit signed integer range [-2<sup>31</sup>, 2<sup>31</sup> - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -2<sup>31</sup> should be clamped to -2<sup>31</sup>, and integers greater than 2<sup>31</sup> - 1 should be clamped to 2<sup>31</sup> - 1.
6. Return the integer as the final result.

> NOTE
> - Only the space character `' '` is considered a whitespace character.
> - Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

## Examples

### Example 1

```console
Input: s = "42"
Output: 42
```

### Example 2

```console
Input: s = "   -42"
Output: -42
```

### Example 3

```console
Input: s = "4193 with words"
Output: 4193
```

## Constraints

- `0 <= s.length <= 200`
- `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`.