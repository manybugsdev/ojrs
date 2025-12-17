# Contributing to ojrs

Thank you for your interest in contributing to ojrs!

## Development Setup

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/manybugsdev/ojrs.git
   cd ojrs
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Project Structure

```
ojrs/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── commands/            # Command implementations
│   │   ├── download.rs      # Download test cases
│   │   ├── test.rs          # Test solutions
│   │   ├── submit.rs        # Submit solutions
│   │   └── login.rs         # Login to platforms
│   ├── platforms/           # Platform-specific implementations
│   │   ├── atcoder.rs       # AtCoder support
│   │   ├── codeforces.rs    # Codeforces support
│   │   └── detector.rs      # Platform detection
│   └── utils/               # Utility modules
│       ├── output.rs        # Colored output
│       ├── file_system.rs   # File operations
│       └── executor.rs      # Command execution
└── tests/                   # Integration tests
```

## Adding a New Platform

To add support for a new online judge platform:

1. Create a new file in `src/platforms/` (e.g., `leetcode.rs`)
2. Implement the `Platform` and `Downloadable` traits:
   ```rust
   pub struct LeetCode {
       client: reqwest::Client,
   }

   impl Platform for LeetCode {
       fn detect(url: &str) -> bool where Self: Sized {
           url.contains("leetcode.com")
       }
       
       fn name(&self) -> &str {
           "LeetCode"
       }
   }

   impl Downloadable for LeetCode {
       async fn download_test_cases(&self, url: &str) -> Result<Vec<TestCase>> {
           // Implementation here
       }
   }
   ```

3. Add the platform to `src/platforms/mod.rs`
4. Update the detector in `src/platforms/detector.rs`
5. Add tests for the new platform

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run integration tests only
cargo test --test integration_test
```

## Code Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to lint code
- Follow Rust naming conventions
- Add documentation comments for public APIs

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and ensure they pass
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Issues

If you find a bug or have a feature request, please open an issue on GitHub.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
