// Shamir Secret Sharing: Extended Implementation in Rust

use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PRIME: u64 = 7919; // Example prime, choose larger primes for real-world use

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub x: u64,
    pub y: u64,
}

pub struct ShamirSecretSharing;

impl ShamirSecretSharing {
    // Generate cryptographically secure coefficients for the polynomial
    fn generate_coefficients(secret: u64, threshold: usize) -> Vec<u64> {
        let mut coefficients = vec![secret];
        let mut rng = OsRng;

        for _ in 1..threshold {
            coefficients.push(rng.next_u64() % PRIME);
        }

        coefficients
    }

    // Evaluate the polynomial at x
    fn evaluate_polynomial(coefficients: &[u64], x: u64) -> u64 {
        coefficients
            .iter()
            .rev()
            .fold(0, |acc, &coeff| (acc * x + coeff) % PRIME)
    }

    // Generate n shares from the secret
    pub fn generate_shares(secret: u64, n: usize, threshold: usize) -> Vec<Share> {
        let coefficients = Self::generate_coefficients(secret, threshold);
        (1..=n)
            .map(|x| Share {
                x: x as u64,
                y: Self::evaluate_polynomial(&coefficients, x as u64),
            })
            .collect()
    }

    // Reconstruct the secret from t shares using Lagrange interpolation
    pub fn reconstruct_secret(shares: &[Share]) -> u64 {
        let mut secret = 0;

        for (i, share_i) in shares.iter().enumerate() {
            let mut numerator = 1;
            let mut denominator = 1;

            for (j, share_j) in shares.iter().enumerate() {
                if i != j {
                    numerator = (numerator * (PRIME + share_j.x - share_i.x)) % PRIME;
                    denominator = (denominator * (PRIME + share_j.x - share_i.x)) % PRIME;
                }
            }

            let lagrange_coefficient = (numerator * mod_inverse(denominator, PRIME)) % PRIME;
            secret = (secret + share_i.y * lagrange_coefficient) % PRIME;
        }

        secret
    }

    // Serialize a share to a string
    pub fn serialize_share(share: &Share) -> String {
        serde_json::to_string(share).unwrap()
    }

    // Deserialize a share from a string
    pub fn deserialize_share(serialized: &str) -> Share {
        serde_json::from_str(serialized).unwrap()
    }
}

// Modular inverse using the extended Euclidean algorithm
fn mod_inverse(a: u64, m: u64) -> u64 {
    let (mut mn, mut xy) = ((m, a), (0, 1));
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    ((xy.0 % m + m) % m) as u64
}

// CLI tool for generating and reconstructing secrets
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_secret_sharing() {
        let secret = 1234;
        let n = 5;
        let threshold = 3;

        let shares = ShamirSecretSharing::generate_shares(secret, n, threshold);

        assert_eq!(shares.len(), n);

        let reconstructed_secret = ShamirSecretSharing::reconstruct_secret(&shares[..threshold]);

        assert_eq!(reconstructed_secret, secret);
    }

    #[test]
    fn test_serialization() {
        let share = Share { x: 1, y: 2345 };
        let serialized = ShamirSecretSharing::serialize_share(&share);
        let deserialized = ShamirSecretSharing::deserialize_share(&serialized);

        assert_eq!(share, deserialized);
    }
}

fn main() {
    let secret = 1234;
    let n = 5;
    let threshold = 3;

    // Generate shares
    let shares = ShamirSecretSharing::generate_shares(secret, n, threshold);
    println!("Generated Shares: {:?}", shares);

    // Serialize and deserialize shares
    let serialized = shares
        .iter()
        .map(|share| ShamirSecretSharing::serialize_share(share))
        .collect::<Vec<_>>();

    println!("Serialized Shares: {:?}", serialized);

    let deserialized = serialized
        .iter()
        .map(|s| ShamirSecretSharing::deserialize_share(s))
        .collect::<Vec<_>>();

    // Reconstruct the secret
    let reconstructed_secret = ShamirSecretSharing::reconstruct_secret(&deserialized[..threshold]);
    println!("Reconstructed Secret: {}", reconstructed_secret);
}
