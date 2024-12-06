# Contributing to Loggix

First off, thank you for considering contributing to Loggix! It's people like you that make Loggix such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to cploutarchou@gmail.com.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include details about your configuration and environment

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* A clear and descriptive title
* A detailed description of the proposed enhancement
* Examples of how the enhancement would be used
* Why this enhancement would be useful to most Loggix users

### Pull Requests

* Fill in the required template
* Follow the Rust coding style
* Include appropriate test cases
* Update documentation as needed
* End all files with a newline

## Development Process

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Run the test suite: `cargo test`
5. Run the formatting checks: `cargo fmt -- --check`
6. Run the linting checks: `cargo clippy`
7. Commit your changes: `git commit -m 'Add some feature'`
8. Push to the branch: `git push origin feature/your-feature-name`
9. Submit a pull request

### Rust Style Guide

* Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.1/style/README.html)
* Use `cargo fmt` to format your code
* Use `cargo clippy` to catch common mistakes
* Write documentation for public APIs
* Include unit tests for new functionality

### Testing

* Write test cases for any new functionality
* Ensure all tests pass locally before submitting a PR
* Include both unit tests and integration tests where appropriate
* Use meaningful test names that describe the behavior being tested

### Documentation

* Update the README.md if needed
* Add inline documentation for new code
* Update the CHANGELOG.md for notable changes
* Include examples for new features

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create a new GitHub release
4. Publish to crates.io

## Getting Help

If you need help, you can:

* Open an issue with your question
* Contact the maintainers directly
* Join our community discussions

## License

By contributing to Loggix, you agree that your contributions will be licensed under its MIT license.
