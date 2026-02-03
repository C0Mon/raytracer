use std::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surround(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn empty() -> Interval {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }
    pub fn universe() -> Interval {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Interval::new(3.0, 5.0), 2.0)]
    #[case(Interval::new(3.0, 5.0), 2.0)]
    #[case(Interval::new(0.0, 0.0), 0.0)]
    #[case(Interval::new(1.0, 0.0), -1.0)]
    fn test_size(#[case] a: Interval, #[case] expected: f64) {
        assert_eq!(a.size(), expected);
    }
    #[rstest]
    #[case(Interval::new(3.0, 5.0), 4.0, true)]
    #[case(Interval::new(3.0, 5.0), 5.0, true)]
    #[case(Interval::new(0.0, 0.0), 0.0, true)]
    #[case(Interval::new(1.0, 0.0), -1.0, false)]
    fn test_contains(#[case] a: Interval, #[case] b: f64, #[case] expected: bool) {
        assert_eq!(a.contains(b), expected);
    }
    #[rstest]
    #[case(Interval::new(3.0, 5.0), 4.0, true)]
    #[case(Interval::new(3.0, 5.0), 5.0, false)]
    #[case(Interval::new(0.0, 0.0), 0.0, false)]
    #[case(Interval::new(1.0, 0.0), -1.0, false)]
    fn test_surrounds(#[case] a: Interval, #[case] b: f64, #[case] expected: bool) {
        assert_eq!(a.surround(b), expected);
    }
}
