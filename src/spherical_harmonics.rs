use nannou::geom::Vec3;
use std::f32::consts::PI;

pub struct SphericalHarmonics {
    levels: usize,
    pub terms: Vec<f32>,
}

impl SphericalHarmonics {
    pub fn from_terms(levels: usize, terms: Vec<f32>) -> SphericalHarmonics {
        SphericalHarmonics { levels, terms }
    }

    pub fn evaluate(&self, direction: Vec3) -> f32 {
        let direction_sh = sh_basis(self.levels, direction);

        // https://cseweb.ucsd.edu/~ravir/papers/envmap/envmap.pdf equation 8

        const A: [f32; 5] = [PI, PI * 2.0 / 3.0, PI * 1.0 / 4.0, 0.0, -PI * 1.0 / 24.0];

        let term = |i| self.terms[i] * direction_sh.terms[i];

        let mut result = term(0) * A[0];

        if self.levels >= 1 {
            result += term(1) * A[1];
            result += term(2) * A[1];
            result += term(3) * A[1];
        }

        if self.levels >= 2 {
            result += term(4) * A[2];
            result += term(5) * A[2];
            result += term(6) * A[2];
            result += term(7) * A[2];
            result += term(8) * A[2];
        }

        if self.levels >= 3 {
            result += term(9) * A[2];
            result += term(10) * A[2];
            result += term(11) * A[2];
            result += term(12) * A[2];
            result += term(13) * A[2];
            result += term(14) * A[2];
            result += term(15) * A[2];
        }

        if self.levels >= 4 {
            result += term(16) * A[4];
            result += term(17) * A[4];
            result += term(18) * A[4];
            result += term(19) * A[4];
            result += term(20) * A[4];
            result += term(21) * A[4];
            result += term(22) * A[4];
            result += term(23) * A[4];
            result += term(24) * A[4];
        }

        return result;
    }
}

fn sh_basis(levels: usize, p: Vec3) -> SphericalHarmonics {
    // From: https://github.com/kayru/Probulator/blob/master/Source/Probulator/SphericalHarmonics.h

    assert!(levels <= 4); // "Spherical Harmonics above L4 are not supported")

    let mut result = SphericalHarmonics {
        levels,
        terms: vec![],
    };

    let x = -p.x;
    let y = -p.y;
    let z = p.z;

    let x2 = x * x;
    let y2 = y * y;
    let z2 = z * z;

    let z3 = z2 * z;

    let x4 = x2 * x2;
    let y4 = y2 * y2;
    let z4 = z2 * z2;

    let sqrt_pi = PI.sqrt();

    result.terms.push(1.0 / (2.0 * sqrt_pi));

    let sqrt = |x: f32| x.sqrt();

    if levels >= 1 {
        result.terms.push(-sqrt(3.0 / (4.0 * PI)) * y);
        result.terms.push(sqrt(3.0 / (4.0 * PI)) * z);
        result.terms.push(-sqrt(3.0 / (4.0 * PI)) * x);
    }

    if levels >= 2 {
        result.terms.push(sqrt(15.0 / (4.0 * PI)) * y * x);
        result.terms.push(-sqrt(15.0 / (4.0 * PI)) * y * z);
        result
            .terms
            .push(sqrt(5.0 / (16.0 * PI)) * (3.0 * z2 - 1.0));
        result.terms.push(-sqrt(15.0 / (4.0 * PI)) * x * z);
        result.terms.push(sqrt(15.0 / (16.0 * PI)) * (x2 - y2));
    }

    if levels >= 3 {
        result
            .terms
            .push(-sqrt(70.0 / (64.0 * PI)) * y * (3.0 * x2 - y2));
        result.terms.push(sqrt(105.0 / (4.0 * PI)) * y * x * z);
        result
            .terms
            .push(-sqrt(42.0 / (64.0 * PI)) * y * (-1.0 + 5.0 * z2));
        result
            .terms
            .push(sqrt(7.0 / (16.0 * PI)) * (5.0 * z3 - 3.0 * z));
        result
            .terms
            .push(-sqrt(42.0 / (64.0 * PI)) * x * (-1.0 + 5.0 * z2));
        result.terms.push(sqrt(105.0 / (16.0 * PI)) * (x2 - y2) * z);
        result
            .terms
            .push(-sqrt(70.0 / (64.0 * PI)) * x * (x2 - 3.0 * y2));
    }

    if levels >= 4 {
        result
            .terms
            .push(3.0 * sqrt(35.0 / (16.0 * PI)) * x * y * (x2 - y2));
        result
            .terms
            .push(-3.0 * sqrt(70.0 / (64.0 * PI)) * y * z * (3.0 * x2 - y2));
        result
            .terms
            .push(3.0 * sqrt(5.0 / (16.0 * PI)) * y * x * (-1.0 + 7.0 * z2));
        result
            .terms
            .push(-3.0 * sqrt(10.0 / (64.0 * PI)) * y * z * (-3.0 + 7.0 * z2));
        result
            .terms
            .push((105.0 * z4 - 90.0 * z2 + 9.0) / (16.0 * sqrt_pi));
        result
            .terms
            .push(-3.0 * sqrt(10.0 / (64.0 * PI)) * x * z * (-3.0 + 7.0 * z2));
        result
            .terms
            .push(3.0 * sqrt(5.0 / (64.0 * PI)) * (x2 - y2) * (-1.0 + 7.0 * z2));
        result
            .terms
            .push(-3.0 * sqrt(70.0 / (64.0 * PI)) * x * z * (x2 - 3.0 * y2));
        result
            .terms
            .push(3.0 * sqrt(35.0 / (4.0 * (64.0 * PI))) * (x4 - 6.0 * y2 * x2 + y4));

        assert!(result.terms.len() == 25);
    }

    return result;
}
