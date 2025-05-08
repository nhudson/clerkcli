# clerkcli

clerkcli is a fast, flexible command-line tool for querying and managing users in Clerk organizations.

## Features

- List users for one or more Clerk organizations
- Output as a table or just email addresses
- Filter and sort results
- Built in Rust for speed and reliability

## Installation

You can install the latest release from [crates.io](https://crates.io/crates/clerkcli) using Cargo:

```sh
cargo install clerkcli
```

## Usage

```sh
clerkcli users list --org-id org_abc,org_xyz --order-by -created_at
clerkcli users list --org-id org_abc --emails-only
```

## Container Usage

```sh
docker run --rm ghcr.io/nhudson/clerkcli:latest users --org-id org_abc --emails-only
``` 