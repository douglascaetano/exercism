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

const MAP: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let mut ret = vec![];

    for map in &MAP {
        if n.divisible_by(map.0) {
            ret.push(map.1);
        }
    }
    if ret.len() > 0 {
        ret.join("")
    } else {
        format!("{}", n)
    }
}
