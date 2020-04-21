trait DivisibleBy {
    fn divisible_by(self, divisor: Self) -> bool;
}

impl<T> DivisibleBy for T
where
    T: Copy + std::ops::Rem,
    <T as std::ops::Rem>::Output: PartialEq,
{
    fn divisible_by(self, divisor: Self) -> bool {
        // comparison must be against std::ops::Rem::Output,
        // so obtain a zero with this type by mod'ing self with itself
        let zero = self % self;

        self % divisor == zero
    }
}

pub fn is_leap_year(year: u64) -> bool {
    year.divisible_by(4) && (!year.divisible_by(100) || year.divisible_by(400))
}
