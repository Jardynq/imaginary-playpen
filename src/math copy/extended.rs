pub trait Extended {
    fn cos_m1(self) -> Self;
    fn log_hypot(self, other: Self) -> Self;
}

impl Extended for f64 {
    fn cos_m1(self) -> Self {
        let limit = std::f64::consts::PI / 4.0;
        if self < -limit || self > limit {
            return self.cos() - 1.0;
        }

        // Taylor series
        let squared = self * self;
        squared *
        (-1.0/2.0 + squared *
            (1.0/24.0 + squared *
                (-1.0/720.0 + squared *
                    (1.0/40320.0 + squared *
                        (-1.0/3628800.0 + squared *
                            (1.0/4790014600.0 + squared *
                                (-1.0/87178291200.0 + squared *
                                    (1.0/20922789888000.0)
                                )
                            )
                        )
                    )
                )
            )
        )
    }

    fn log_hypot(self, other: Self) -> Self {
        let a = self.abs();
        let b = other.abs();
        if a == 0.0 {
            return b.ln();
        }
        if b == 0.0 {
            return a.ln();
        }
        if a < 3000.0 && b < 3000.0 {
            return (a * a + b * b).ln() * 0.5;
        }
        (a / b.atan2(a).cos()).ln()
    }
}
impl Extended for f32 {
    fn cos_m1(self) -> Self {
        let limit = std::f32::consts::PI / 4.0;
        if self < -limit || self > limit {
            return self.cos() - 1.0;
        }

        // Taylor series
        let squared = self * self;
        squared *
        (-1.0/2.0 + squared *
            (1.0/24.0 + squared *
                (-1.0/720.0 + squared *
                    (1.0/40320.0 + squared *
                        (-1.0/3628800.0 + squared *
                            (1.0/4790014600.0 + squared *
                                (-1.0/87178291200.0 + squared *
                                    (1.0/20922789888000.0)
                                )
                            )
                        )
                    )
                )
            )
        )
    }

    fn log_hypot(self, other: Self) -> Self {
        let a = self.abs();
        let b = other.abs();
        if a == 0.0 {
            return b.ln();
        }
        if b == 0.0 {
            return a.ln();
        }
        if a < 3000.0 && b < 3000.0 {
            return (a * a + b * b).ln() * 0.5;
        }
        (a / b.atan2(a).cos()).ln()
    }
}
