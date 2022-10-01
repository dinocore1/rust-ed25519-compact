//! A compact Ed25519 and X25519 implementation for Rust.
//!
//! * Formally-verified Curve25519 field arithmetic
//! * `no_std`-friendly
//! * WebAssembly-friendly
//! * Compute@Edge-friendly
//! * Lightweight
//! * Zero dependencies if randomness is provided by the application
//! * Only one portable dependency (`getrandom`) if not
//! * Safe and simple Rust interface.
//!
//! Example usage:
//!
//! ```rust
//! use ed25519_compact::*;
//!
//! // A message to sign and verify.
//! let message = b"test";
//!
//! // Generates a new key pair using a random seed.
//! // A given seed will always produce the same key pair.
//! let key_pair = KeyPair::from_seed(Seed::generate());
//!
//! // Computes a signature for this message using the secret part of the key pair.
//! let signature = key_pair.sk.sign(message, Some(Noise::generate()));
//!
//! // Verifies the signature using the public part of the key pair.
//! key_pair
//!     .pk
//!     .verify(message, &signature)
//!     .expect("Signature didn't verify");
//!
//! // Verification of a different message using the same signature and public key fails.
//! key_pair
//!     .pk
//!     .verify(b"A different message", &signature)
//!     .expect_err("Signature shouldn't verify");
//!
//! // All these structures can be viewed as raw bytes simply by dereferencing them:
//! let signature_as_bytes: &[u8] = signature.as_ref();
//! println!("Signature as bytes: {:?}", signature_as_bytes);
//! ```
//!
//! Cargo features:
//!
//! * `self-verify`: after having computed a new signature, verify that is it
//!   valid. This is slower, but improves resilience against fault attacks. It
//!   is enabled by default on WebAssembly targets.
//! * `std`: disables `no_std` compatibility in order to make errors implement
//!   the standard `Error` trait.
//! * `random` (enabled by default): adds `Default` and `generate`
//!   implementations to the `Seed` and `Noise` objects, in order to securely
//!   create random keys and noise.
//! * `traits`: add support for the traits from the ed25519 and signature
//!   crates.
//! * `pem`: add support for importing/exporting keys as OpenSSL-compatible PEM
//!   files.
//! * `blind-keys`: add support for key blinding.
//! * `opt_size`: Enable size optimizations (based on benchmarks, 8-15% size
//!   reduction at the cost of 6.5-7% performance).
//! * `x25519`: Enable support for the X25519 key exchange system.
//! * `disable-signatures`: Disable support for signatures, and only compile
//!   support for X25519.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::needless_range_loop,
    clippy::many_single_char_names,
    clippy::unreadable_literal,
    clippy::let_and_return,
    clippy::needless_lifetimes,
    clippy::cast_lossless,
    clippy::suspicious_arithmetic_impl,
    clippy::identity_op
)]

mod common;
mod error;
mod field25519;
pub mod sha512;

pub use crate::common::*;
pub use crate::error::*;

#[cfg(not(feature = "disable-signatures"))]
mod ed25519;
#[cfg(not(feature = "disable-signatures"))]
mod edwards25519;

#[cfg(not(feature = "disable-signatures"))]
pub use crate::ed25519::*;

#[cfg(feature = "x25519")]
pub mod x25519;

#[cfg(not(feature = "disable-signatures"))]
#[cfg(feature = "pem")]
mod pem;
