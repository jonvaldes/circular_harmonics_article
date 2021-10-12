// https://blackpawn.com/texts/ch/default.html

use std::f32::consts::PI;
use std::ops::{Add, Sub};

fn term_to_band(term: usize) -> usize {
    (term + 1) / 2
}

fn calculate_term(term: usize, angle: f32) -> f32 {
    let band = term_to_band(term);
    match term {
        0 => 1.0 / (2.0 * PI).sqrt(),
        term if (term % 2) != 0 => (angle * band as f32).cos() / PI.sqrt(),
        _ => (angle * band as f32).sin() / PI.sqrt(),
    }
}

// Returns the indefinite integral for a box, for each term
fn term_integral_box(term: usize, angle: f32) -> f32 {
    let band = term_to_band(term);
    match term {
        0 => angle / (2.0 * PI).sqrt(),
        term if (term % 2) != 0 => {
            // https://www.wolframalpha.com/input/?i=integral++cos%284theta%29%2Fsqrt%28pi%29
            (angle * band as f32).sin() / (band as f32 * PI.sqrt())
        }
        _ => {
            // https://www.wolframalpha.com/input/?i=integral++sin%284theta%29%2Fsqrt%28pi%29
            -(angle * band as f32).cos() / (band as f32 * PI.sqrt())
        }
    }
}

pub struct CircularHarmonics {
    // Band:         0 |   1    |   2    |   3    | ...
    // Coefficients: 0 | [1, 2] | [3, 4] | [5, 6] | ...
    coeffs: Vec<f32>,
}

impl CircularHarmonics {
    pub fn new(band_count: usize) -> CircularHarmonics {
        CircularHarmonics {
            coeffs: vec![0.0; band_count * 2 - 1],
        }
    }

    pub fn from_coeffs(coeffs: Vec<f32>) -> CircularHarmonics {
        CircularHarmonics { coeffs }
    }

    pub fn from_impulse(band_count: usize, angle: f32, strength: f32) -> CircularHarmonics {
        let coeff_count = band_count * 2 - 1;
        let mut coeffs = vec![0.0; coeff_count];
        for i in 0..coeff_count {
            coeffs[i] = strength * calculate_term(i, angle); // TODO -- Is multiplying by strength OK here?
        }
        CircularHarmonics { coeffs }
    }

    pub fn from_pulse(band_count: usize, pulse_width: f32, strength: f32) -> CircularHarmonics {
        let coeff_count = band_count * 2 - 1;
        let mut coeffs = vec![0.0; coeff_count];
        for i in 0..coeff_count {
            coeffs[i] = strength
                * (term_integral_box(i, pulse_width * 0.5)
                    - term_integral_box(i, -pulse_width * 0.5));
            // TODO -- Is multiplying by strength OK here?
        }
        CircularHarmonics { coeffs }
    }

    pub fn coeff_count(&self) -> usize {
        self.coeffs.len()
    }

    pub fn band_count(&self) -> usize {
        (self.coeffs.len() + 1) / 2
    }

    pub fn evaluate(&self, angle: f32) -> f32 {
        let mut accum = 0.0;
        for (i, v) in self.coeffs.iter().enumerate() {
            accum += v * calculate_term(i, angle);
        }

        accum
    }

    pub fn rotate(&self, angle: f32) -> CircularHarmonics {
        let mut result = Self::new(self.band_count());
        result.coeffs[0] = self.coeffs[0];
        for band in 1..self.band_count() {
            let (s, c) = (angle * band as f32).sin_cos();
            let (bandx, bandy) = self.band(band);
            let rotx = bandx * c - bandy * s;
            let roty = bandy * c + bandx * s;
            result.set_band(band, rotx, roty);
        }

        result
    }

    pub fn band(&self, n: usize) -> (f32, f32) {
        if n >= self.band_count() {
            panic!("Accessing bands this CH does not have");
        }

        if n == 0 {
            panic!("Band 0 cannot be accessed through this interface");
        }
        (self.coeffs[(n * 2) - 1], self.coeffs[n * 2])
    }

    pub fn band0(&self) -> f32 {
        self.coeffs[0]
    }

    pub fn set_band(&mut self, n: usize, a: f32, b: f32) {
        if n >= self.band_count() {
            panic!("Accessing bands this CH does not have");
        }

        if n == 0 {
            panic!("Band 0 cannot be accessed through this interface");
        }
        self.coeffs[(n * 2) - 1] = a;
        self.coeffs[n * 2] = b;
    }

    pub fn set_band0(&mut self, v: f32) {
        self.coeffs[0] = v;
    }

    /*
    pub fn convolve(&self, other: &CircularHarmonics) -> CircularHarmonics {

        THIS IS WRONG
        let result_term_count = self.coeff_count();

        let mut result_terms = vec!(0.0; result_term_count);

        let h0 = other.coeffs.get(0).unwrap_or(&1.0);

        for i in 0..result_term_count {
            let a = self.coeffs.get(i).unwrap_or(&1.0);

            result_terms[i] = a * h0;
        }

        CircularHarmonics {
            coeffs: result_terms,
        }
    }
    */
}

impl<'a, 'b> Add<&'b CircularHarmonics> for &'a CircularHarmonics {
    type Output = CircularHarmonics;

    fn add(self, other: &'b CircularHarmonics) -> CircularHarmonics {
        let result_term_count = self.coeff_count().max(other.coeff_count());

        let mut result_terms = vec![0.0; result_term_count];

        for i in 0..result_term_count {
            let a = self.coeffs.get(i).unwrap_or(&0.0);
            let b = other.coeffs.get(i).unwrap_or(&0.0);

            result_terms[i] = a + b;
        }

        CircularHarmonics {
            coeffs: result_terms,
        }
    }
}

impl<'a, 'b> Sub<&'b CircularHarmonics> for &'a CircularHarmonics {
    type Output = CircularHarmonics;

    fn sub(self, other: &'b CircularHarmonics) -> CircularHarmonics {
        let result_term_count = self.coeff_count().max(other.coeff_count());

        let mut result_terms = vec![0.0; result_term_count];

        for i in 0..result_term_count {
            let a = self.coeffs.get(i).unwrap_or(&0.0);
            let b = other.coeffs.get(i).unwrap_or(&0.0);

            result_terms[i] = a - b;
        }

        CircularHarmonics {
            coeffs: result_terms,
        }
    }
}
