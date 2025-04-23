# Gofer.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html)
[![Package](https://img.shields.io/crates/v/gofer)](https://crates.io/crates/gofer)
[![Documentation](https://docs.rs/gofer/badge.svg)](https://docs.rs/gofer/)

🚧 _We are building in public. This is presently under heavy construction._

## ✨ Features

- Supports opting out of any feature using comprehensive feature flags.
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
gofer = { version = "0.1", default-features = false, features = [] }
```

## 👉 Examples

### Importing the library

```rust
use gofer::*;
```

## 📚 Reference

https://docs.rs/gofer/

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

[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
