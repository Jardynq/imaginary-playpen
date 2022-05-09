use super::Extended;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
// Constructors
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self {
            re: re,
            im: im,
        }
    }
    pub fn real(value: f64) -> Self {
        Self::new(value, 0.0)
    }
    pub fn imaginary(value: f64) -> Self {
        Self::new(0.0, value)
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn infinity() -> Self {
        Self::new(f64::INFINITY, f64::INFINITY)
    }
    pub fn nan() -> Self {
        Self::new(f64::NAN, f64::NAN)
    }
}

// Valdiation
impl Complex {
    pub fn is_zero(&self) -> bool {
        self.re == 0.0 && self.im == 0.0
    }
    pub fn is_nan(&self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }
    pub fn is_infinite(&self) -> bool {
        self.re.is_infinite() || self.im.is_infinite()
    }
    pub fn is_finite(&self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    pub fn is_normal(&self) -> bool {
        self.re.is_normal() && self.im.is_normal()
    }
    pub fn is_real(&self) -> bool {
        self.re != 0.0 && self.im == 0.0
    }
    pub fn is_imaginary(&self) -> bool {
        self.im != 0.0 && self.re == 0.0
    }
}

// Common
impl Complex {
    pub fn abs(&self) -> f64 {
        self.re.hypot(self.im)
    }
    pub fn arg(&self) -> f64 {
        self.re.atan2(self.im)
    }

    pub fn fract(self) -> Self {
        Self::new(
            self.re.fract(), 
            self.im.fract(),
        )
    }
    pub fn sign(self) -> Self {
        let abs = self.abs();
        Self::new(
            self.re / abs, 
            self.im / abs,
        )
    }
    pub fn dot(self, other: Self) -> Self {
        Self::new(
            self.re * other.re,
            self.im * other.im,
        )
    }
    pub fn conjugate(self) -> Self {
        Self::new(
            self.re,
            -self.im,
        )
    }

    pub fn sqrt(self) -> Self {
        if self.is_real() {
            return Self::real(self.re.sqrt());
        }

        let c = self.re * self.re + self.im * self.im;
        Self::new(
            ((self.re + c.sqrt()) * 0.5).sqrt(),
            ((-self.re + c.sqrt()) * 0.5).sqrt() * (self.im / self.im.abs()),
        )
    }
    pub fn exp(self) -> Self {
        let exp = self.re.exp();
        Self::new(
            exp * self.im.cos(),
            exp * self.im.sin(),
        )
    }
    pub fn exp_m1(self) -> Self {
        Self::new(
            self.re.exp_m1() * self.im.cos() + self.im.cos_m1(),
            self.re.exp() * self.im.sin(),
        )
    }
    pub fn log(self) -> Self {
        Self::new(
            self.re.log_hypot(self.im),
            self.im.atan2(self.re),
        )
    }

    pub fn pow(self, exponent: Self) -> Self {
        if self.is_zero() && exponent.re > 0.0 && exponent.im >= 0.0 {
            return Complex::zero();
        }
        if exponent.is_zero() {
            return Self::new(1.0, 0.0);
        }
        if exponent.is_real() {
            if self.is_real() {
                return Self::new(self.re.powf(exponent.re), 0.0);
            }
        }

        let arg = self.im.atan2(self.re);
        let loh = self.re.log_hypot(self.im);

        let a = (exponent.re * loh - exponent.im * arg).exp();
        let b = exponent.im * loh + exponent.re * arg;
        Self::new(
            a * b.cos(),
            a * b.sin(),
        )
    }
}

// Trigonometry
impl Complex {
    pub fn sin(self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }
    pub fn asin(self) -> Self {
        let a = Self::new(
            self.im * self.im - self.re * self.re + 1.0,
            -2.0 * self.re * self.im
        ).sqrt();
        let b = Self::new(
            a.re - self.im,
            a.im + self.re
        ).log();

        Self::new(
            b.im, 
            -b.re
        )
    }

    pub fn cos(self) -> Self {
        Self::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }
    pub fn acos(self) -> Self {
        let a = Self::new(
            self.im * self.im - self.re * self.re + 1.0,
            -2.0 * self.re * self.im
        ).sqrt();
        let b = Self::new(
            a.re - self.im,
            a.im + self.re
        ).log();

        Self::new(
            std::f64::consts::PI / 2.0 - b.im, 
            b.re
        )
    }

    pub fn tan(self) -> Self {
        let re = 2.0 * self.re;
        let im = 2.0 * self.im;
        let det = re.cos() + im.cosh();
        Self::new(
            re.sin() / det,
            im.sinh() / det,
        )
    }
    pub fn atan(self) -> Self {
        if self.re == 0.0 {
            if self.im == 1.0 {
                return Self::imaginary(f64::INFINITY)
            }
            if self.im == -1.0 {
                return Self::imaginary(f64::NEG_INFINITY)
            }
        }

        let det = self.re * self.re + (1.0 - self.im) * (1.0 - self.im);
        let a = Self::new(
            (1.0 - self.im * self.im - self.re * self.re) / det,
            (-2.0 * self.re) / det
        ).log();
        
        Self::new(
            -0.5 * a.im,
            0.5 * a.re,
        )
    }

    pub fn cot(self) -> Self {
        let re = 2.0 * self.re;
        let im = 2.0 * self.im;
        let det = re.cos() - im.cosh();
        Self::new(
            -re.sin() / det,
            im.sinh() / det,
        )
    }
    pub fn acot(self) -> Self {
        if self.im == 0.0 {
            return Self::real((1.0_f64).atan2(self.re));
        }

        let det = self.re * self.re + self.im * self.im;
        if det == 0.0 {
            Self::new(
                if self.re != 0.0 { f64::INFINITY } else { 0.0 },
                if self.im != 0.0 { f64::NEG_INFINITY } else { 0.0 },
            ).atan()
        } else {
            Self::new(
                self.re / det,
                -self.im / det,
            ).atan()
        }
    }

    pub fn sec(self) -> Self {
        let det = 0.5 * (2.0 * self.im).cosh() + 0.5 * (2.0 * self.re).cos();
        Self::new(
            self.re.cos() * self.im.cosh() / det,
            self.re.sin() * self.im.sinh() / det,
        )
    }
    pub fn asec(self) -> Self {
        if self.is_zero() {
            return Self::imaginary(f64::INFINITY);
        }

        let det = self.re * self.re + self.im * self.im;
        if det == 0.0 {
            Self::new(
                if self.re != 0.0 { f64::INFINITY } else { 0.0 },
                if self.im != 0.0 { f64::NEG_INFINITY } else { 0.0 },
            ).acos()
        } else {
            Self::new(
                self.re / det,
                -self.im / det,
            ).acos()
        }
    }
    
    pub fn csc(self) -> Self {
        let det = 0.5 * (2.0 * self.im).cosh() + 0.5 * (2.0 * self.re).cos();
        Self::new(
            self.re.cos() * self.im.cosh() / det,
            self.re.sin() * self.im.sinh() / det,
        )
    }
    pub fn acsc(self) -> Self {
        if self.is_zero() {
            return Self::new(std::f64::consts::PI / 2.0, f64::INFINITY);
        }

        let det = self.re * self.re + self.im * self.im;
        if det == 0.0 {
            Self::new(
                if self.re != 0.0 { f64::INFINITY } else { 0.0 },
                if self.im != 0.0 { f64::NEG_INFINITY } else { 0.0 },
            ).asin()
        } else {
            Self::new(
                self.re / det,
                -self.im / det,
            ).asin()
        }
    }

    // TODO HYPERBOLIC
    // https://github.com/infusion/Complex.js/blob/master/complex.js
}

// Operators
impl std::ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re:  -self.re,
            im:  -self.im,
        }
    }
}
impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (ar, ai) = (self.re, self.im);
        let (br, bi) = (other.re, other.im);
        Self {
            re:  ar + br,
            im:  ai + bi,
        }
    }
}
impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let (ar, ai) = (self.re, self.im);
        let (br, bi) = (other.re, other.im);
        Self {
            re:  ar - br,
            im:  ai - bi,
        }
    }
}
impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let (ar, ai) = (self.re, self.im);
        let (br, bi) = (other.re, other.im);
        Self {
            re:  ar * br - ai * bi,
            im:  ar * bi + ai * br,
        }
    }
}
impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let (ar, ai) = (self.re, self.im);
        let (br, bi) = (other.re, other.im);
        Self {
            re: (ar * br + ai * bi) / (br * br + bi * bi),
            im: (ai * br - ar * bi) / (br * br + bi * bi),
        }
    }
}

// Pretty printing
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let real = {
            if self.re.is_infinite() && self.re.is_sign_negative() { format!("-inf") }
            else if self.re.is_infinite() { format!("inf") }
            else if self.re.is_nan() { format!("nan") }
            else if !self.re.is_normal() { format!("") }
            else { format!("{:.4}", self.re) }
        };
        let mut imaginary = {
            if self.im.is_infinite() && self.im.is_sign_negative() { format!("-inf i") }
            else if self.im.is_infinite() { format!("inf i") }
            else if self.im.is_nan() { format!("nan i") }
            else if !self.im.is_normal() { format!("") }
            else if self.im == 1.0 { format!("i") }
            else if self.im == -1.0 { format!("-i") }
            else { format!("{:.4}i", self.im) }
        };
        let operator = if self.im > 0.0 { 
            "+"
        } else if self.im < 0.0 {
            imaginary = format!("{}", &imaginary[1..]);
            "-"
        } else {
            ""
        };

        if real == "" && imaginary == "" { write!(f, "0") } 
        else if imaginary == "" { write!(f, "{}", real) }
        else if real == "" { write!(f, "{}", imaginary) }
        else { write!(f, "{} {} {}", real, operator, imaginary) }
    }
}
impl Complex {
    pub fn to_string_phasor(&self) -> String {
        let radius = self.abs();
        let theta = self.arg().to_degrees();
        format!("{:.4} ∠ {:.4}°", radius, theta)
    }
    pub fn to_string_polar(&self) -> String {
        let radius = self.abs();
        let theta = self.arg().to_degrees();
        format!("{:.4} * cos({:.4}°) + sin({:.4}°)i", radius, theta, theta)
    }
    pub fn to_string_exponential(&self) -> String {
        let radius = self.abs();
        let theta = self.arg().to_degrees();
        format!("{:.4} * e^{:.4}°i", radius, theta)
    }
}
