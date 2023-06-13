# rustpw
A simple command line password manager.
Sure, here's a template you can use as a starting point for your README.md file:

---

# RustPW

RustPW is a simple, command-line password manager designed for personal usage. It aims to offer a minimal and easily reviewable codebase, ensuring complete transparency about how your passwords are managed.

## :warning: Security Notice

RustPW does not encrypt the stored passwords. Therefore, only use RustPW if you understand the implications and know what you're doing. Your passwords will be stored in a single plain text file, which allows for easy manual retrieval if necessary.

## Key Features

- **Simplicity**: With just about 250 lines of Rust, you can review the complete codebase in an hour or two. 

- **Minimal Dependencies**: Apart from the standard library, RustPW only uses the highly popular [`clap`](https://crates.io/crates/clap) crate for command-line argument parsing.

- **Ease of Access**: Passwords are stored in a single plain text file using a simple `key: value` syntax. This allows for easy, manual retrieval of your passwords if needed.

## Installation

Here you can describe how to install the program. It could be via `cargo install`, manually cloning and building, or any other distribution method.

## Usage

Here you can provide some examples of how to use the application, demonstrating various commands and their outputs.

## Contributing

If you're interested in contributing, thank you! First, please read our [Contributing Guide](LINK_TO_CONTRIBUTING_GUIDE).

## License

RustPW is open-source software, freely available under the [MIT License](LICENSE).

---

Please replace "LINK_TO_CONTRIBUTING_GUIDE" with the actual link to your contributing guide (if any), and ensure that the MIT license is correctly linked to. If you don't have a separate `LICENSE` file, I'd recommend creating one. If you're not using the MIT license, replace "MIT License" with the appropriate license name and link.


# RustPW

RustPW is a minimalist, command-line password manager designed for personal usage. Its primary distinctions from [pass](https://www.passwordstore.org/) are:

1. **No Encryption**: RustPW does not encrypt the stored passwords. It is designed for users who understand the implications of this choice. Be sure you know what you're doing before deciding to use RustPW for password storage.

2. **Minimalist Design**: With approximately 250 lines of Rust, RustPW is intentionally minimal. This allows users to review the entire code managing their passwords in an hour or two. Apart from the standard library, RustPW uses only a few commonly used crates:
   - [clap](https://crates.io/crates/clap) for command-line argument parsing,
   - [clipboard](https://crates.io/crates/clipboard) for directly copying passwords to the clipboard, and
   - [rand](https://crates.io/crates/rand) for generating secure random passwords.

3. **Simple Storage**: RustPW stores your passwords in a single plain text file using a straightforward `key: value` syntax. This approach gives users the option to manually retrieve their passwords if needed.

## Warning
This tool is provided as-is, with no guarantees. Given the unencrypted nature of the stored passwords, make sure you understand the potential security implications before using RustPW.
