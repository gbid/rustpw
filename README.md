# RustPW

RustPW is a minimalist, command-line password manager designed for personal usage. Its primary distinctions from [pass](https://www.passwordstore.org/) are:

1. **Minimalist Design**: With approximately 250 lines of Rust, RustPW is intentionally minimal. This allows users to review the entire code managing their passwords in an hour or two. Apart from the standard library, RustPW depends on only a few commonly used crates:
   - [clap](https://crates.io/crates/clap) for command-line argument parsing,
   - [clipboard](https://crates.io/crates/clipboard) for directly copying passwords to the clipboard, and
   - [rand](https://crates.io/crates/rand) for generating secure random passwords.

2. **No Encryption**: RustPW does not encrypt the stored passwords. It is designed for users who understand the implications of this choice. In particular, RustPW does *not* protect you from the following attack vectors:
    - If your hard drive is encrypted, an attacker with (physical or remote) access to any user who can read your password file can obtain your passwords.
    - If your hard drive is not encrypted, additionally, an attacker with physical access to your machine can obtain your passwords.


3. **Simple Storage**: RustPW stores your passwords in a single plain text file using a straightforward `key: value` syntax. This approach gives users the option to manually retrieve their passwords if needed.
