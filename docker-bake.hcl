# Variables to be specified externally.
variable "registry" {
  default = "ghcr.io/nhudson"
  description = "The image registry."
}

variable "version" {
  default = ""
  description = "The release version."
}

variable "revision" {
  default = ""
  description = "The current Git commit SHA."
}

# Values to use in the targets.
now = timestamp()
authors = "Nick Hudson <nick.hudson@gmail.com>"
url = "https://github.com/nhudson/clerkcli"

target "default" {
  platforms = ["linux/amd64", "linux/arm64"]
  context = "."
  dockerfile-inline = <<EOT
  FROM rust:1.86-slim AS builder
  WORKDIR /app
  COPY . .
  RUN cargo build --release
  FROM gcr.io/distroless/cc
  COPY --from=builder /app/target/release/clerkcli /usr/local/bin/clerkcli
  ENTRYPOINT ["/usr/local/bin/clerkcli"] 
  EOT
  tags = [
    "${registry}/clerkcli:latest",
    "${registry}/clerkcli:${version}",
  ]
  annotations = [
    "index,manifest:org.opencontainers.image.created=${now}",
    "index,manifest:org.opencontainers.image.url=${url}",
    "index,manifest:org.opencontainers.image.source=${url}",
    "index,manifest:org.opencontainers.image.version=${version}",
    "index,manifest:org.opencontainers.image.revision=${revision}",
    "index,manifest:org.opencontainers.image.vendor=${authors}",
    "index,manifest:org.opencontainers.image.title=Clerk CLI",
    "index,manifest:org.opencontainers.image.description=Clerk CLI",
    "index,manifest:org.opencontainers.image.documentation=${url}",
    "index,manifest:org.opencontainers.image.authors=${authors}",
    "index,manifest:org.opencontainers.image.licenses=MIT",
    "index,manifest:org.opencontainers.image.base.name=scratch",
  ]
  labels = {
    "org.opencontainers.image.created" = "${now}",
    "org.opencontainers.image.url" = "${url}",
    "org.opencontainers.image.source" = "${url}",
    "org.opencontainers.image.version" = "${version}",
    "org.opencontainers.image.revision" = "${revision}",
    "org.opencontainers.image.vendor" = "${authors}",
    "org.opencontainers.image.title" = "Clerk CLI",
    "org.opencontainers.image.description" = "Clerk CLI",
    "org.opencontainers.image.documentation" = "${url}",
    "org.opencontainers.image.authors" = "${authors}",
    "org.opencontainers.image.licenses" = "MIT",
    "org.opencontainers.image.base.name" = "scratch",
  }
}