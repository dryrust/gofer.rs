# Gofer.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html)
[![Package](https://img.shields.io/crates/v/gofer)](https://crates.io/crates/gofer)
[![Documentation](https://docs.rs/gofer/badge.svg)](https://docs.rs/gofer/)

**Gofer.rs** makes it easy to fetch data from any URL in Rust.
Just call `gofer::open(url)` to get back a `Read`!

## ✨ Features

- Currently supports `https:`, `http:`, `ftp:`, `file:`, and `data:` URLs.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.81+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add gofer
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
gofer = "0.1"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
gofer = { version = "0.1", default-features = false, features = ["file"] }
```

## 👉 Examples

### Importing the library

```rust
use gofer::open;
```

### Reading HTTP(S) URLs

```rust
let result = gofer::open("https://www.google.com/robots.txt");
```

### Reading FTP URLs

```rust
let result = gofer::open("ftp://ftp.isc.org/welcome.msg");
```

### Reading `file:` URLs

```rust
let result = gofer::open("file:///path/to/file.txt");
```

### Reading `stdin:` URLs

```rust
let result = gofer::open("stdin:");
```

### Reading `data:` URLs

```rust
let result = gofer::open("data:,Hello%2C%20world%21%0A");
```

## 📚 Reference

https://docs.rs/gofer/

### Protocols

Scheme   | Feature  | Summary
:------- | :------- | :---------------------------------------------------------
`data:`  | `data`   | Inline data in Base64 or URL-encoded format
`file:`  | `file`   | Local file path
`ftp:`   | `ftp`    | FTP
`http:`  | `http`   | HTTP
`https:` | `https`  | HTTPS
`stdin:` | `stdin`  | Standard input stream

### Integrations

Crate (Feature) | Version | Usage | Summary
:--- | :--- | :--- | :---
[clap] &nbsp;<sub>(`"clap"`)</sub> | 4.5 | [![clap](https://docs.rs/clap/badge.svg)](https://docs.rs/clap/) | Implements `clap::builder::TypedValueParser` (TBD)
[miette] &nbsp;<sub>(`"miette"`)</sub> | 7.5 | [![miette](https://docs.rs/miette/badge.svg)](https://docs.rs/miette/) | Derives `miette::Diagnostic` on `gofer::Error`
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/gofer.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/gofer.rs&text=Gofer.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/gofer.rs&title=Gofer.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/gofer.rs&t=Gofer.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/gofer.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/gofer.rs)

[feature flags]: https://github.com/dryrust/gofer.rs/blob/master/lib/gofer/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[clap]: https://crates.io/crates/clap
[miette]: https://crates.io/crates/miette
