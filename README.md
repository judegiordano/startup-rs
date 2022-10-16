# Rust Api

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/judegiordano/startup/blob/main/LICENSE)
![tests workflow](https://github.com/judegiordano/startup-rs/actions/workflows/tests.yml/badge.svg)
![clippy workflow](https://github.com/judegiordano/startup-rs/actions/workflows/clippy-lint.yml/badge.svg)

## Rust Resources

[Rust-Lang](https://www.rust-lang.org)

[Crates.io](https://crates.io)

[Lib.rs](https://lib.rs/about)

<details open>
<summary>Books</summary>

- [The Book](https://doc.rust-lang.org/stable/book/title-page.html)
- [Rustc Book](https://rustc-dev-guide.rust-lang.org/about-this-guide.html)
- [Rustonomicon](https://doc.rust-lang.org/nomicon/index.html)
- [Rustdoc](https://doc.rust-lang.org/rustdoc/index.html)
- [Async Book](https://rust-lang.github.io/async-book)
- [Performance Book](https://nnethercote.github.io/perf-book/title-page.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Wasm](https://rustwasm.github.io/docs/book)
- [CLI](https://rust-cli.github.io/book/index.html)
- [Embedded](https://doc.rust-lang.org/stable/embedded-book)
- [Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html)
- [References](https://doc.rust-lang.org/reference/index.html)
- [Gui](https://gtk-rs.org/gtk4-rs/stable/latest/book)
  <br>

</details>

<details open>
<summary>Practice</summary>

- [Rustlings](https://github.com/rust-lang/rustlings)
  <br>

</details>

<details open>
<summary>Examples</summary>

- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/about.html)
  <br>

</details>

<details open>
<summary>Are We -</summary>

- [Are We Web](https://www.arewewebyet.org)
- [Are We Game](https://arewegameyet.rs)
- [Are We Async](https://areweasyncyet.rs)
  <br>

</details>

---

```sh
rustup component add clippy
# this is a fairly aggressive linter
cargo clippy -- \
-W clippy::nursery \
-W clippy::pedantic \
-W clippy::unwrap_used \
-W clippy::expect_used \
-A clippy::future_not_send \
-A clippy::must_use_candidate \
-A clippy::missing_errors_doc \
-A clippy::unused_async \
```
