# rs-argon2-easy

Dead-simple password hashing with Argon2id — `hash()` and `verify()`, nothing more.

## Installation

```toml
[dependencies]
philiprehberger-argon2-easy = "0.1"
```

## Usage

```rust
use philiprehberger_argon2_easy::{hash, verify};

// Hash a password
let hashed = hash("my-password")?;

// Verify a password
let is_valid = verify("my-password", &hashed)?;
assert!(is_valid);
```

### Profiles

```rust
use philiprehberger_argon2_easy::{hash_with, Profile};

// Fast hashing for interactive login
let hashed = hash_with("password", Profile::Interactive)?;

// Slow hashing for encryption keys
let hashed = hash_with("password", Profile::Sensitive)?;
```

### Check if rehashing is needed

```rust
use philiprehberger_argon2_easy::needs_rehash;

if needs_rehash(&stored_hash)? {
    let new_hash = hash(password)?;
    // Store new_hash
}
```

## API

| Function | Description |
|----------|-------------|
| `hash(password)` | Hash with default OWASP-recommended parameters |
| `hash_with(password, profile)` | Hash with a specific profile |
| `verify(password, hash)` | Verify a password against a hash |
| `needs_rehash(hash)` | Check if hash uses outdated parameters |

## Profiles

| Profile | Memory | Iterations | Parallelism | Use case |
|---------|--------|------------|-------------|----------|
| `Interactive` | 19 MiB | 2 | 1 | Login forms |
| `Default` | 46 MiB | 1 | 1 | General purpose (OWASP) |
| `Sensitive` | 64 MiB | 3 | 4 | Encryption keys |

## License

MIT
