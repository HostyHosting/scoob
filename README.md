# scoob

Scoob is a secrets management tool, designed

Secrets are encrypted using [Sodium sealed boxes](https://libsodium.gitbook.io/doc/public-key_cryptography/sealed_boxes).

## Getting Started

Install scoob somehow.

First, you'll want to create a secrets file:

```bash
scoob modify --create ./secrets/dev.yml
```

This will open your editor with an example Scoob configuration file. A pair of public and secret keys will auto-generated and provided in the file. Make sure you don't commit these into your repository, and instead replace them with values provided dynamically via environment variables. When you close your editor, all of the values under `configuration:` will be encrypted, and the file will be written to disk.

At a later point, you can add additional secrets by running the following command:

```bash
scoob modify --edit ./secrets/dev.yml
```

We recommend creating a separate secrets file for development and production. This way, you can keep your production keys separate.

To use these secrets, you can start a process with Scoob:

```bash
scoob start ./secrets/dev.yml <command...>
```

This will decrypt the secrets in the file, and will run the command with the secrets added to the environment variables.

## Why Rust?

Scoob was originally written in Node, but has been rewritten in Rust.
Scoob is designed to ship alongside your production code, as the CLI is used to decrypt secrets into environment variables. As such, it has the following goals:

- Must work on most development / deployment platforms.
- Must have a very small runtime memory overhead.
  - `scoob-node` currently has ~55mb of memory overhead.
  - `scoob-rs` currently has ~380kb of memory overhead to start, and has **0** runtime overhead on Unix systems.
- Must start quickly.
  - `scoob-node` currently has ~500ms of start time overhead.
  - `scoob-rs` currently has ~0ms of start time overhead.
- The binary should be small enough to build into a Docker image.
  - `scoob-node` is currently ~60mb (including node runtime).
  - `scoob-rs` is currently ~1-2mb.
- Must work with projects that are not written in Node.js and do not have `npm` or `node` installed.
