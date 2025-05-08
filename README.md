# clerkcli

clerkcli is a fast, flexible command-line tool for querying and managing users in Clerk organizations.

## Features

- List users for one or more Clerk organizations
- Output as a table or just email addresses
- Filter and sort results
- Built in Rust for speed and reliability

## Authentication

The CLI requires a Clerk secret key to authenticate requests. You can find your secret key under "API Keys" in the Clerk Dashboard.

You can provide the secret key in two ways:
- Set the `CLERK_SECRET_KEY` environment variable:
  ```bash
  export CLERK_SECRET_KEY=sk_test_...
  clerkcli users list --org-id org_abc
  ```
- Or pass it directly as a CLI argument:
  ```bash
  clerkcli --secret-key sk_test_... users list --org-id org_abc
  ```

## Installation

You can install the latest release from [crates.io](https://crates.io/crates/clerkcli) using Cargo:

```bash
cargo install clerkcli
```

## Usage

```bash
clerkcli users list --org-id org_abc,org_xyz --order-by -created_at
clerkcli users list --org-id org_abc --emails-only
```

## Container Usage

```bash
docker run -e CLERK_SECRET_KEY=$CLERK_SECRET_KEY--rm ghcr.io/nhudson/clerkcli:latest users list --org-id org_abc --emails-only
``` 