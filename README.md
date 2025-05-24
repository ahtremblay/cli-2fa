# cli-2fa

A simple, secure command-line tool written in Rust for managing Time-based One-Time Password (TOTP) two-factor authentication (2FA) codes. Access your 2FA codes directly from your terminal without needing to reach for your phone!

## Features

*   **Secure Storage:** Stores your 2FA secrets in your operating system's native keychain (macOS Keychain, GNOME Keyring, KWallet, Windows Credential Manager) via the `keyring` crate. Secrets are not stored in plain text files.
*   **Multi-Account Support:** Manage 2FA codes for different services (e.g., Google, GitHub, AWS) using unique account names.
*   **OTP Generation:** Generates TOTP codes locally using the `totp-rs` crate.
*   **Simple Interface:** Easy-to-use `push` (add/update) and `get` (generate) commands.
*   **Cross-Platform:** Built with Rust, aiming for compatibility across macOS, Linux, and Windows (wherever a `keyring` backend is available).

## Why `cli-2fa`?

*   **Convenience:** Quickly grab 2FA codes from the terminal, especially useful for developers and CLI power users.
*   **Scriptability:** Can be integrated into scripts for automated logins (use with caution and understand security implications).
*   **Security-conscious:** Leverages the OS keychain for secret storage, which is generally more secure than custom encryption or plain text.

## Prerequisites

*   **Rust Toolchain:** Install Rust via [rustup](https://rustup.rs/).
*   **OS Keychain Service:** Your operating system must have a functioning keychain or secret service.
    *   **macOS:** Keychain Access (built-in).
    *   **Linux:** GNOME Keyring (libsecret), KWallet, or another service supported by the `secret-service-rs` crate. You might need to install packages like `libsecret-1-dev` (Debian/Ubuntu) or `gnome-keyring` and ensure the daemon is running.
    *   **Windows:** Credential Manager (built-in).

## Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/ahtremblay/cli-2fa.git
    cd cli-2fa
    ```
    You will run the tool from within this cloned directory using `cargo run `.

## Usage

The tool uses a fixed service name `rust.twofa-cli` to store all secrets in the OS keychain. Each 2FA secret is then uniquely identified by an **account name** you provide. All commands are run from the root of the cloned project directory.

### Adding or Updating a 2FA Secret

To store or update the 2FA secret (the Base32 string provided by the service like Google, GitHub, etc.):

```bash
cargo run push <ACCOUNT_NAME> <BASE32_SECRET_STRING>
```

*   `<ACCOUNT_NAME>`: A unique identifier you choose for this 2FA account. This allows you to store multiple 2FA secrets. For example: `google_personal`, `github_work`, `aws_console`.
*   `<BASE32_SECRET_STRING>`: The secret key provided by the 2FA service provider.

**Example:**

```bash
cargo run push my_google_account JBSWY3DPEHPK3PXP # (Use your actual secret)
cargo run push company_aws_account ONYXK234ONYXKABC
```

### Generating an OTP

To generate the current OTP for a previously stored secret:

```bash
cargo run get <ACCOUNT_NAME>
```

*   `<ACCOUNT_NAME>`: The same unique identifier you used with the `push` command.

**Example Output:**

```bash
cargo run get my_google_account
```
```
123456
```

The OTP is printed to standard output.

### Other Commands

```bash
cargo run delete <ACCOUNT_NAME>
```

```bash
cargo run list
```

## Security Considerations

*   This tool stores your sensitive 2FA secrets in your operating system's keychain. The security of these secrets relies on the security of your OS keychain and your user account.
*   The application itself does not transmit your secrets over the network; all OTP generation is done locally.
*   Ensure your computer is secured with a strong password, disk encryption, and up-to-date software.
*   Be cautious if integrating this tool into automated scripts, as this could potentially expose OTPs if the script's environment or output is compromised.

## License

This project is licensed under the MIT License.
