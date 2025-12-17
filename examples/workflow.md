# Example Workflow

This document demonstrates a typical workflow using ojrs for competitive programming.

## Scenario: Solving an AtCoder Problem

Let's say you want to solve problem A from AtCoder Beginner Contest 123.

### Step 1: Download Test Cases

```bash
$ ojrs download https://atcoder.jp/contests/abc123/tasks/abc123_a

ℹ Downloading test cases from: https://atcoder.jp/contests/abc123/tasks/abc123_a
ℹ Detected platform: AtCoder
ℹ Found 3 test case(s)
✓ Saved test case 1 to test/sample-1.in and test/sample-1.out
✓ Saved test case 2 to test/sample-2.in and test/sample-2.out
✓ Saved test case 3 to test/sample-3.in and test/sample-3.out
✓ Downloaded 3 test case(s) successfully!
```

This creates a `test/` directory with the following files:
```
test/
├── sample-1.in
├── sample-1.out
├── sample-2.in
├── sample-2.out
├── sample-3.in
└── sample-3.out
```

### Step 2: Write Your Solution

Create a file `solution.py`:

```python
# solution.py
a, b = map(int, input().split())
print(a + b)
```

### Step 3: Test Your Solution

```bash
$ ojrs test --command "python3 solution.py"

ℹ Testing with command: python3 solution.py
ℹ Found 3 test case(s)

Test 1 ... AC
  Time: 15.2ms

Test 2 ... AC
  Time: 14.8ms

Test 3 ... AC
  Time: 15.1ms

Results: 3 passed, 0 failed
✓ All tests passed!
```

### Step 4: Submit Your Solution

```bash
$ ojrs submit https://atcoder.jp/contests/abc123/tasks/abc123_a solution.py

ℹ Submit functionality for https://atcoder.jp/contests/abc123/tasks/abc123_a with file solution.py
⚠ Submit command is not yet implemented
ℹ This feature requires authentication and platform-specific API integration
```

(Note: Submit functionality is under development)

## Example with Different Languages

### C++

```bash
# Write solution.cpp
$ g++ -o solution solution.cpp
$ ojrs test --command "./solution"
```

### Java

```bash
# Write Solution.java
$ javac Solution.java
$ ojrs test --command "java Solution"
```

### Rust

```bash
# Write solution.rs
$ rustc solution.rs -o solution
$ ojrs test --command "./solution"
```

### Node.js

```bash
# Write solution.js
$ ojrs test --command "node solution.js"
```

## Custom Test Directory

You can use a custom directory for test cases:

```bash
$ ojrs download https://codeforces.com/contest/1234/problem/A --directory my-tests
$ ojrs test --command "python3 solution.py" --directory my-tests
```

## Handling Wrong Answers

If your solution produces wrong answers:

```bash
$ ojrs test --command "python3 wrong_solution.py"

ℹ Testing with command: python3 wrong_solution.py
ℹ Found 2 test case(s)

Test 1 ... WA
  Expected:
    3
  Got:
    5

Test 2 ... AC
  Time: 14.5ms

Results: 1 passed, 1 failed
✗ 1 test(s) failed
```

The tool shows you the expected output vs. your actual output, making it easy to debug.

## Tips

1. **Use version control**: Commit your solution and test cases
2. **Test incrementally**: Test your solution as you develop it
3. **Check edge cases**: Add custom test cases if needed
4. **Time your solution**: The test command shows execution time
5. **Use the right command**: Make sure your compile/run command is correct
