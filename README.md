# ojrs

Online Judge Tools in Rust - A command-line tool for competitive programming.

## Features

- **Download test cases** from online judge platforms (AtCoder, Codeforces)
- **Test your code locally** against downloaded test cases
- **Submit solutions** to online judge platforms (planned)
- **Login to platforms** for authenticated actions (planned)

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/ojrs`.

## Usage

### Download Test Cases

Download sample test cases from a problem URL:

```bash
ojrs download <URL>
```

Example:
```bash
ojrs download https://atcoder.jp/contests/abc123/tasks/abc123_a
ojrs download https://codeforces.com/contest/1234/problem/A
```

By default, test cases are saved in the `test/` directory. You can specify a different directory:

```bash
ojrs download <URL> --directory my-tests
```

### Test Your Solution

Test your code against downloaded test cases:

```bash
ojrs test --command "<your-command>"
```

Examples:
```bash
ojrs test --command "python3 solution.py"
ojrs test --command "node solution.js"
ojrs test --command "./a.out"
ojrs test --command "java Solution"
```

You can specify a different test directory:

```bash
ojrs test --command "python3 solution.py" --directory my-tests
```

### Submit Solution (Coming Soon)

Submit your solution to an online judge:

```bash
ojrs submit <URL> <file>
```

### Login (Coming Soon)

Login to an online judge platform:

```bash
ojrs login <URL>
```

## Supported Platforms

- ✅ AtCoder (download test cases)
- ✅ Codeforces (download test cases)
- 🚧 Submit and login features (in development)

## Example Workflow

1. Download test cases:
   ```bash
   ojrs download https://atcoder.jp/contests/abc123/tasks/abc123_a
   ```

2. Write your solution (e.g., `solution.py`)

3. Test your solution:
   ```bash
   ojrs test --command "python3 solution.py"
   ```

4. Submit when all tests pass:
   ```bash
   ojrs submit https://atcoder.jp/contests/abc123/tasks/abc123_a solution.py
   ```

## License

MIT License - see [LICENSE](LICENSE) file for details