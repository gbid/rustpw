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
