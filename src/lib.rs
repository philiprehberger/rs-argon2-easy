//! Dead-simple password hashing with Argon2id.
//!
//! This crate provides a minimal API for hashing and verifying passwords
//! using Argon2id with secure defaults. Just call [`hash()`] and [`verify()`].
//! Use [`timing_safe_eq()`] for constant-time string comparison when comparing
//! hashes or tokens directly.
//!
//! # Example
//!
//! ```
//! use philiprehberger_argon2_easy::{hash, verify};
//!
//! let hashed = hash("my-password").unwrap();
//! assert!(verify("my-password", &hashed).unwrap());
//! assert!(!verify("wrong-password", &hashed).unwrap());
//! ```

use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use password_hash::SaltString;
use rand_core::OsRng;
use std::fmt;

/// Preset parameter profiles for Argon2id hashing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// Fast hashing suitable for interactive login (19 MiB memory, 2 iterations, 1 parallelism).
    Interactive,
    /// Default profile with OWASP-recommended parameters (46 MiB memory, 1 iteration, 1 parallelism).
    Default,
    /// Slow hashing for high-security use cases (64 MiB memory, 3 iterations, 4 parallelism).
    Sensitive,
}

impl Profile {
    /// Returns the Argon2 parameters for this profile.
    fn params(&self) -> Params {
        match self {
            Profile::Interactive => {
                Params::new(19_456, 2, 1, None).expect("valid interactive params")
            }
            Profile::Default => Params::new(47_104, 1, 1, None).expect("valid default params"),
            Profile::Sensitive => {
                Params::new(65_536, 3, 4, None).expect("valid sensitive params")
            }
        }
    }
}

/// Errors that can occur during hashing or verification.
#[derive(Debug)]
pub enum HashError {
    /// Failed to generate hash.
    HashingFailed(String),
    /// Invalid hash format.
    InvalidHash(String),
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashError::HashingFailed(msg) => write!(f, "hashing failed: {msg}"),
            HashError::InvalidHash(msg) => write!(f, "invalid hash: {msg}"),
        }
    }
}

impl std::error::Error for HashError {}

/// Hash a password using the [`Profile::Default`] parameters.
///
/// Returns a PHC-format string containing the algorithm, parameters, salt, and hash.
///
/// # Example
///
/// ```
/// let hashed = philiprehberger_argon2_easy::hash("my-password").unwrap();
/// assert!(hashed.starts_with("$argon2id$"));
/// ```
pub fn hash(password: &str) -> Result<String, HashError> {
    hash_with(password, Profile::Default)
}

/// Hash a password using the specified [`Profile`].
///
/// Returns a PHC-format string containing the algorithm, parameters, salt, and hash.
///
/// # Example
///
/// ```
/// use philiprehberger_argon2_easy::{hash_with, Profile};
///
/// let hashed = hash_with("my-password", Profile::Interactive).unwrap();
/// assert!(hashed.starts_with("$argon2id$"));
/// ```
pub fn hash_with(password: &str, profile: Profile) -> Result<String, HashError> {
    let params = profile.params();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| HashError::HashingFailed(e.to_string()))?;

    Ok(hash.to_string())
}

/// Verify a password against a PHC-format hash string.
///
/// Returns `Ok(true)` if the password matches, `Ok(false)` if it does not,
/// or `Err` if the hash string is malformed.
///
/// # Example
///
/// ```
/// use philiprehberger_argon2_easy::{hash, verify};
///
/// let hashed = hash("my-password").unwrap();
/// assert!(verify("my-password", &hashed).unwrap());
/// assert!(!verify("wrong-password", &hashed).unwrap());
/// ```
pub fn verify(password: &str, hash: &str) -> Result<bool, HashError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| HashError::InvalidHash(e.to_string()))?;

    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => Err(HashError::InvalidHash(e.to_string())),
    }
}

/// Check if a hash uses outdated parameters and should be rehashed.
///
/// Compares the parameters in the given hash against the [`Profile::Default`]
/// parameters. Returns `Ok(true)` if the parameters differ (meaning the hash
/// should be regenerated), or `Ok(false)` if they match.
///
/// # Example
///
/// ```
/// use philiprehberger_argon2_easy::{hash, hash_with, needs_rehash, Profile};
///
/// let default_hash = hash("password").unwrap();
/// assert!(!needs_rehash(&default_hash).unwrap());
///
/// let interactive_hash = hash_with("password", Profile::Interactive).unwrap();
/// assert!(needs_rehash(&interactive_hash).unwrap());
/// ```
pub fn needs_rehash(hash: &str) -> Result<bool, HashError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| HashError::InvalidHash(e.to_string()))?;

    let default_params = Profile::Default.params();

    // Extract parameters from the hash
    let m_cost = parsed_hash
        .params
        .get_str("m")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);
    let t_cost = parsed_hash
        .params
        .get_str("t")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);
    let p_cost = parsed_hash
        .params
        .get_str("p")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let needs_update = m_cost != default_params.m_cost()
        || t_cost != default_params.t_cost()
        || p_cost != default_params.p_cost();

    Ok(needs_update)
}

/// Constant-time string comparison to prevent timing attacks.
///
/// Returns `true` if the two strings are equal, `false` otherwise.
/// The comparison always examines every byte to avoid leaking information
/// about where a mismatch occurs.
///
/// # Example
///
/// ```
/// use philiprehberger_argon2_easy::timing_safe_eq;
///
/// assert!(timing_safe_eq("same-value", "same-value"));
/// assert!(!timing_safe_eq("value-a", "value-b"));
/// ```
pub fn timing_safe_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    let mut result: u8 = 0;
    for (x, y) in a_bytes.iter().zip(b_bytes.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_produces_phc_format() {
        let hashed = hash("test-password").unwrap();
        assert!(hashed.starts_with("$argon2id$"));
    }

    #[test]
    fn verify_correct_password() {
        let hashed = hash("correct-password").unwrap();
        assert!(verify("correct-password", &hashed).unwrap());
    }

    #[test]
    fn verify_wrong_password() {
        let hashed = hash("correct-password").unwrap();
        assert!(!verify("wrong-password", &hashed).unwrap());
    }

    #[test]
    fn verify_malformed_hash() {
        let result = verify("password", "not-a-valid-hash");
        assert!(result.is_err());
    }

    #[test]
    fn hash_with_interactive_profile() {
        let hashed = hash_with("password", Profile::Interactive).unwrap();
        assert!(hashed.starts_with("$argon2id$"));
    }

    #[test]
    fn hash_with_default_profile() {
        let hashed = hash_with("password", Profile::Default).unwrap();
        assert!(hashed.starts_with("$argon2id$"));
    }

    #[test]
    fn hash_with_sensitive_profile() {
        let hashed = hash_with("password", Profile::Sensitive).unwrap();
        assert!(hashed.starts_with("$argon2id$"));
    }

    #[test]
    fn needs_rehash_false_for_default() {
        let hashed = hash("password").unwrap();
        assert!(!needs_rehash(&hashed).unwrap());
    }

    #[test]
    fn needs_rehash_true_for_interactive() {
        let hashed = hash_with("password", Profile::Interactive).unwrap();
        assert!(needs_rehash(&hashed).unwrap());
    }

    #[test]
    fn hash_produces_different_output_each_time() {
        let hash1 = hash("same-password").unwrap();
        let hash2 = hash("same-password").unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn empty_password_hash_and_verify() {
        let hashed = hash("").unwrap();
        assert!(verify("", &hashed).unwrap());
        assert!(!verify("not-empty", &hashed).unwrap());
    }

    #[test]
    fn unicode_password_hash_and_verify() {
        let password = "p\u{00e4}ssw\u{00f6}rd-\u{1f512}";
        let hashed = hash(password).unwrap();
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("plain-ascii", &hashed).unwrap());
    }

    #[test]
    fn timing_safe_eq_equal_strings() {
        assert!(timing_safe_eq("hello", "hello"));
    }

    #[test]
    fn timing_safe_eq_different_strings() {
        assert!(!timing_safe_eq("hello", "world"));
    }

    #[test]
    fn timing_safe_eq_different_lengths() {
        assert!(!timing_safe_eq("short", "much-longer"));
    }

    #[test]
    fn timing_safe_eq_empty_strings() {
        assert!(timing_safe_eq("", ""));
    }

    #[test]
    fn timing_safe_eq_single_bit_difference() {
        assert!(!timing_safe_eq("a", "b"));
    }
}
