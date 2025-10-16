# Rust Workspace Github Template

This repository is intended to be a template for Rust projects hosted on GitHub.

## After Cloning

1. Update workspace `Cargo.toml`
   - `authors`
   - `edition`
   - `rust-version`
   - `categories`
   - `keywords`

2. Update `bin` and `lib` crates
   - Crate names and directories (if desired)
   - Update workspace `Cargo.toml` appropriately
   - Update "`bin`" crate dependency to "`lib`" crate

3. Update `LICENSE`

4. Update/replace this `README.md`

## Project Getting Started

1. Initialize your development environment

   ```sh
   make deps
   ```
