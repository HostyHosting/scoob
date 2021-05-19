# scoob

Scoob is a secrets management tool, designed to make managing your development and production secrets easier, and cloud-agnostic.

Secrets are encrypted using [Sodium sealed boxes](https://libsodium.gitbook.io/doc/public-key_cryptography/sealed_boxes).

## Installing

**[Homebrew](https://brew.sh/):**

```bash
brew install hostyhosting/tap/scoob
```

**Shell (Mac, Linux):**

```bash
curl -fsSL https://scoob-rs.netlify.app/install.sh | sh
```

**PowerShell (Windows):**

```bash
iwr https://scoob-rs.netlify.app/install.ps1 -useb | iex
```

**Build and install from source using [Cargo](https://crates.io/crates/scoob):**

```bash
cargo install scoob --locked
```

## Getting Started

First, you'll want to create a secrets file:

```bash
scoob manage ./secrets/dev.yml
```

This will open your editor with an example Scoob configuration file. A pair of public and secret keys will auto-generated and provided in the file. Make sure you don't commit these into your repository, and instead replace them with values provided dynamically via environment variables. When you close your editor, all of the values under `configuration:` will be encrypted, and the file will be written to disk.

At a later point, you can add additional secrets by running the same command:

```bash
scoob manage ./secrets/dev.yml
```

We recommend creating a separate secrets file for development and production. This way, you can keep your production keys separate.

To use these secrets, you can start a process with Scoob:

```bash
scoob start ./secrets/dev.yml <command...>
```

This will decrypt the secrets in the file, and will run the command with the secrets added to the environment variables.
