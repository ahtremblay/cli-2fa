# cli-2fa

A simple, secure command-line tool written in Rust for managing Time-based One-Time Password (TOTP) two-factor authentication (2FA) codes. Access your 2FA codes directly from your terminal without needing to reach for your phone!

## Features

*   **Secure Storage:** Stores your 2FA secrets in your operating system's native keychain (macOS Keychain, GNOME Keyring, KWallet, Windows Credential Manager) via the `keyring` crate. Secrets are not stored in plain text files.
*   **OTP Generation:** Generates TOTP codes locally using the `totp-rs` crate.
*   **Simple Interface:** Easy-to-use `push` (add/update) and `get` (generate) commands.
*   **Cross-Platform:** Built with Rust, aiming for compatibility across macOS, Linux, and Windows (wherever a `keyring` backend is available).

## Why `cli-2fa`?

*   **Convenience:** Quickly grab 2FA codes from the terminal, especially useful for developers and CLI power users.
*   **Scriptability:** Can be integrated into scripts for automated logins (use with caution and understand security implications).
*   **Security-conscious:** Leverages the OS keychain for secret storage, which is generally more secure than custom encryption or plain text.
*   **Learn Rust:** This project also serves as a practical example of building a CLI tool in Rust.

## Prerequisites

*   **Rust Toolchain:**  Install Rust via [rustup](https://rustup.rs/).
*   **OS Keychain Service:** Your operating system must have a functioning keychain or secret service.
    *   **macOS:** Keychain Access (built-in).
    *   **Linux:** GNOME Keyring (libsecret), KWallet, or another service supported by the `secret-service-rs` crate. You might need to install packages like `libsecret-1-dev` or `gnome-keyring`.
    *   **Windows:** Credential Manager (built-in).

## Installation

### From Source (Recommended for now)

1.  Clone the repository:
    ```bash
    git clone https://github.com/ahtremblay/cli-2fa.git
    cd cli-2fa
    ```
2.  Build the release binary:
    ```bash
    cargo build --release
    ```


## Usage

`cli-2fa` currently manages **one** 2FA secret at a time. This secret is associated with a default service name (`com.example.twofa-cli`) and account ID (`salesforce_default_user`) in your OS keychain. Future versions aim to support multiple named accounts.

### Pushing (Adding/Updating) a Secret

To store or update the 2FA secret (the Base32 string provided by the service, e.g., Google, Salesforce):

```bash
cli-2fa push YOUR_BASE32_SECRET_STRING
```

This will store `YOUR_BASE32_SECRET_STRING` securely in your OS keychain.

### Getting (Generating) an OTP

To generate the current OTP for the stored secret:

```bash
cli-2fa get
```

Example output:
```
Generated OTP: 123456
```

The OTP is printed to standard output.

## Security Considerations

*   This tool stores your sensitive 2FA secrets in your operating system's keychain. The security of these secrets relies on the security of your OS keychain and your user account.
*   The application itself does not transmit your secrets over the network.
*   Ensure your computer is secured with a strong password, disk encryption, and up-to-date software.
*   Be cautious if integrating this tool into automated scripts, as this could potentially expose OTPs if the script's environment or output is compromised.

## Current Limitations & Future Enhancements

`cli-2fa` is currently in its early stages. Here are some planned improvements:

*   **[HIGH PRIORITY] Named Accounts:**
    *   Allow managing multiple 2FA secrets for different services (e.g., `cli-2fa push google <secret>`, `cli-2fa get github`).
    *   Commands like `add <account_name> <secret>`, `get <account_name>`, `list`, `delete <account_name>`.

## License

This project is licensed under the Unlicense License
