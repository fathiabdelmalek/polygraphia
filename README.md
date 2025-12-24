# Polygraphia

A comprehensive Rust library for classical and modern cryptographic algorithms.

## Features

- **Classical Ciphers**: Caesar, Affine, Hill, Playfair
- **Text Modes**: Preserve all characters or alphabetic only
- **Key Derivation**: PBKDF2-HMAC-SHA512/256
- **FFI Support**: C-compatible shared library (coming soon!)
- **Type-Safe**: Leverages Rust's type system for security

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
polygraphia = "0.1.0"
```

## Quick Start
```rust
use polygraphia::classical::Caesar;
use polygraphia::traits::Cipher;

let cipher = Caesar::new(3);
let encrypted = cipher.encrypt("hello")?;
let decrypted = cipher.decrypt(&encrypted)?;
```

## Algorithms

### Caesar Cipher
```rust
use polygraphia::classical::Caesar;
let cipher = Caesar::new(3);
```

### Affine Cipher
```rust
use polygraphia::classical::Affine;
let cipher = Affine::new(5, 8)?;
```

### Playfair Cipher
```rust
use polygraphia::classical::Playfair;
let cipher = Playfair::new("secret")?;
```

### Hill Cipher
```rust
use polygraphia::classical::Hill;
let cipher = Hill::new("hill")?;
```

## License

Apache-2.0
```