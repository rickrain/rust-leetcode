# Introduction

This repo contains a collection of leetcode challenges I've completed using Rust.

Each challenge in this repo is organized into a module with tests specific to the module to test for correct results. Each challenge also has a README that explains the challenge (similar to how it was described on leetcode), including samples and constraints for the challenge. The list below contains the challenges in this repo.

- [two sum](./src/prob_0001_two_sum/README.md)
- [add two numbers](./src/prob_0002_add_two_nums/README.md)
- [longest substring](./src/prob_0003_longest_substring/README.md)
- [longest palindromic substr](./src/prob_0005_longest_palindromic_substr/README.md)
- [reverse integer](./src/prob_0007_reverse_integer/README.md)
- [string to int](./src/prob_0008_string_to_int/README.md)
- [palindrome](./src/prob_0009_palindrome/README.md)
- [permutations](./src/prob_0046_permutations/README.md)

## Running the challenges

The leetcode challenges are designed such that your code produces a specific output, such as a vector of integers ( `Vec<i32>` ), a `boolean` or some other _value_ that can be deterministically tested for. Therefore, user experiences, such as typing something at the keyboard and getting a response, are not associated with the challenges. So, to actually run the challenges, you simply need to run the tests that were created for each challenge.

To run all tests, run `cargo test`.

To run a specific test, such as the _palindrome_ challenge tests, run `cargo test prob_0009_palindrome`.