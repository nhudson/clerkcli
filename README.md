# clerkcli

clerkcli is a fast, flexible command-line tool for querying and managing users in Clerk organizations.

## Features

- List users for one or more Clerk organizations
- Output as a table or just email addresses
- Filter and sort results
- Built in Rust for speed and reliability

## Usage

```sh
clerkcli users --org-id org_abc,org_xyz --order-by -created_at
clerkcli users --org-id org_abc --emails-only
```

## Container Usage

```sh
docker run --rm ghcr.io/nhudson/clerkcli:latest users --org-id org_abc --emails-only
``` 