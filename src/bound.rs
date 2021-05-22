pub enum Assert<const COND: bool> {}

//pub type AssertN<const N: usize, const M: usize> = Assert<{N > M}>;

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
    debug_assert!(self.bits() >= v.bits() && !v.fits_word());
    debug_assert!(n + m <= $n_words);
    // D1.
    // Make sure 64th bit in v's highest word is set.
    // If we shift both self and v, it won't affect the quotient
    // and the remainder will only need to be shifted back.
    let shift = v.0[n - 1].leading_zeros();
    v <<= shift;
    // u will store the remainder (shifted)
    let mut u = self.full_shl(shift);

    // quotient
    let mut q = Self::zero();
    let v_n_1 = v.0[n - 1];
    let v_n_2 = v.0[n - 2];

    // D2. D7.
    // iterate from m downto 0
    for j in (0..=m).rev() {
        let u_jn = u[j + n];

        // D3.
        // q_hat is our guess for the j-th quotient digit
        // q_hat = min(b - 1, (u_{j+n} * b + u_{j+n-1}) / v_{n-1})
        // b = 1 << WORD_BITS
        // Theorem B: q_hat >= q_j >= q_hat - 2
        let mut q_hat = if u_jn < v_n_1 {
            let (mut q_hat, mut r_hat) = Self::div_mod_word(u_jn, u[j + n - 1], v_n_1);
            // this loop takes at most 2 iterations
            loop {
                // check if q_hat * v_{n-2} > b * r_hat + u_{j+n-2}
                let (hi, lo) = Self::split_u128(u128::from(q_hat) * u128::from(v_n_2));
                if (hi, lo) <= (r_hat, u[j + n - 2]) {
                    break;
                }
                // then iterate till it doesn't hold
                q_hat -= 1;
                let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                r_hat = new_r_hat;
                // if r_hat overflowed, we're done
                if overflow {
                    break;
                }
            }
            q_hat
        } else {
            // here q_hat >= q_j >= q_hat - 1
            u64::max_value()
        };

        // ex. 20:
        // since q_hat * v_{n-2} <= b * r_hat + u_{j+n-2},
        // either q_hat == q_j, or q_hat == q_j + 1

        // D4.
        // let's assume optimistically q_hat == q_j
        // subtract (q_hat * v) from u[j..]
        let q_hat_v = v.full_mul_u64(q_hat);
        // u[j..] -= q_hat_v;
        let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);

        // D6.
        // actually, q_hat == q_j + 1 and u[j..] has overflowed
        // highly unlikely ~ (1 / 2^63)
        if c {
            q_hat -= 1;
            // add v to u[j..]
            let c = Self::add_slice(&mut u[j..], &v.0[..n]);
            u[j + n] = u[j + n].wrapping_add(u64::from(c));
        }

        // D5.
        q.0[j] = q_hat;
    }

    // D8.
    let remainder = Self::full_shr(u, shift);

    (q, remainder)
}