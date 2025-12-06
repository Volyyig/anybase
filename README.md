# 🔢 anybase

A Rust library for converting numbers between arbitrary bases.

## ✨ Features

- 🔄 Convert between any base
- ⚡ Fast and efficient algorithms
- 🎯 Simple, intuitive API
- 📦 No external dependencies

## 📥 Installation

Run the following command in your project directory:

```bash
cargo add anybase
```

## 💡 Usage

```rust
use anybase::{convert_base, Converter};

// Functional
let result = convert_base("ff", "0123456789abcdef", "01234567").unwrap();
assert_eq!(result, "377");

// Object-oriented
let converter = Converter::new("01", "0123456789");
let result = converter.convert("1010").unwrap();
assert_eq!(result, "10");
```

## 📄 License

MIT OR Apache-2.0

## 🤝 Contributing

Contributions welcome! Please open an issue or submit a pull request.