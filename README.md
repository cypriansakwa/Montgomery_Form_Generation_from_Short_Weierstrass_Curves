## Overview
This Rust program converts a short Weierstrass elliptic curve of the form $y^2 = x^3 + ax + b$ to its equivalent Montgomery form, represented by $B y^2 = x^3 + A x^2 + x$, where parameters $A$ and $B$ are calculated based on inputs $a$ and $b$ over a finite field $\mathbb{F}_p$.

## Conversion Formula

For the conversion, we use the following approach:

1. **Finding the root $z_0$**: Find $z_0$ as a root of the polynomial $P(z) = z^3 + az + b$ modulo $p$.
2. **Calculating $s$**: Define $s = \left( \sqrt{3z_0^2 + a} \right)^{-1}$, where $s$ is the modular inverse of the square root.
3. **Montgomery Parameters**:
   - **$A$**: $A = 3z_0s \mod p$
   - **$B$**: $B = s$

## Prerequisites

- Rust and Cargo should be installed. To install Rust, follow the instructions on [rust-lang.org](https://www.rust-lang.org/).

## Usage
- Clone this repository and navigate to the project directory:
   ```bash
   git clone https://github.com/cypriansakwa/Montgomery_Form_Generation_from_Short_Weierstrass_Curves.git
   cd Montgomery_Form_Generation_from_Short_Weierstrass_Curves
- Update the values of parameters $a, b$, and $p$ in src/main.rs to match the curve you want to convert. Here, p is the prime defining the finite field 
$\mathbb{F}_p$.

- Build and run the program:

   ```bash
   cargo run
- The program outputs the Montgomery parameters $A$ and $B$ if a valid conversion is possible, or an error message if the conversion fails.
## Example
For a short Weierstrass curve defined by parameters $a=8$, $b=8$, and $p=13$:
  >```
  >  let a = 8;
  >  let b = 8;
Running the program will output the Montgomery parameters for this curve.
