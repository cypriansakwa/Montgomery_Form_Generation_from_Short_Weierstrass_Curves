const P: i64 = 13; // Prime field F_p for modulo operations

/// Converts a short Weierstrass curve (y^2 = x^3 + ax + b) to a Montgomery curve (By^2 = x^3 + Ax^2 + x).
/// Returns the Montgomery parameters A and B if the conversion is valid.
fn short_weierstrass_to_montgomery(a: i64, b: i64) -> Option<(i64, i64)> {
    // Step 1: Find z0, a root of the polynomial P(z) = z^3 + az + b mod P
    let mut z0 = None;
    for z in 0..P {
        if (z.pow(3) + a * z + b) % P == 0 {
            z0 = Some(z);
            break;
        }
    }

    let z0 = z0?; // If no root is found, return None

    // Step 2: Calculate s = (sqrt(3 * z0^2 + a))^(-1) mod P
    let term = (3 * z0.pow(2) + a) % P;
    if let Some(sqrt_term) = modular_sqrt(term, P) {
        let s = modular_inverse(sqrt_term, P)?;

        // Step 3: Calculate A = 3 * z0 * s mod P
        let a_montgomery = (3 * z0 * s) % P;

        // B is equal to s in this formulation
        let b_montgomery = s;

        Some((a_montgomery, b_montgomery))
    } else {
        println!("No square root found for the term (3z0^2 + a) mod P.");
        None
    }
}

/// Helper function to compute modular square root (returns None if no square root exists)
fn modular_sqrt(a: i64, p: i64) -> Option<i64> {
    for i in 0..p {
        if (i * i) % p == a % p {
            return Some(i);
        }
    }
    None
}

/// Helper function to compute modular inverse using the extended Euclidean algorithm
fn modular_inverse(a: i64, p: i64) -> Option<i64> {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, p, a);
    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        std::mem::swap(&mut t, &mut new_t);
        r = r - quotient * new_r;
        std::mem::swap(&mut r, &mut new_r);
    }
    if r > 1 {
        None // No inverse exists
    } else {
        Some((t + p) % p)
    }
}

fn main() {
    let a = 8; // Example Short Weierstrass parameter
    let b = 8; // Example Short Weierstrass parameter

    match short_weierstrass_to_montgomery(a, b) {
        Some((a_montgomery, b_montgomery)) => {
            println!("Montgomery parameters: A = {}, B = {}", a_montgomery, b_montgomery);
        },
        None => println!("Conversion to Montgomery form is not possible for the given curve parameters."),
    }
}
