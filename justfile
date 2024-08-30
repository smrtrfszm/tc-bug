#!/usr/bin/env -S just --justfile

image_base := "ghcr.io/verseghy"

_default:
  @just --list --unsorted

install-tools:
  cargo install cargo-deny --locked
  cargo install cargo-nextest --locked
  cargo install cargo-llvm-cov --locked

test:
  @cargo nextest run

test-coverage:
  @cargo llvm-cov --ignore-filename-regex "(migration|entity|cmds)/.*" nextest

clean:
  @cargo clean

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all --all-targets --all-features

[private]
build-image NAME FILE:
  podman build -t {{image_base}}/{{NAME}} -f {{FILE}} .

[group("build")]
build-setup-image: (build-image "matverseny-setup" "containerfiles/setup.Containerfile")
[group("build")]
build-migration-image: (build-image "matverseny-migration" "containerfiles/migration.Containerfile")
[group("build")]
build-backend-image: (build-image "matverseny-backend" "Containerfile")

[group("build")]
build-images: build-setup-image build-migration-image build-backend-image

[private]
push-image NAME:
  podman push {{image_base}}/{{NAME}}

[group("push")]
push-setup-image: (push-image "matverseny-setup")
[group("push")]
push-migration-image: (push-image "matverseny-migration")
[group("push")]
push-backend-image: (push-image "matverseny-backend")

[group("push")]
push-images: build-setup-image push-migration-image push-backend-image
