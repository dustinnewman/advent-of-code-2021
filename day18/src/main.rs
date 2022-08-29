#[derive(Clone)]
enum Number {
    Value(usize),
    Tuple(Box<Number>, Box<Number>),
}

impl Number {
    fn add(a: Number, b: Number) -> Self {
        Number::Tuple(Box::new(a), Box::new(b))
    }

    fn split(self) -> Option<Self> {
        match self {
            Number::Value(value) if value >= 10 => {
                let left = value / 2;
                let right = value - left;
                Some(Self::Tuple(Box::new(Number::Value(left)), Box::new(Number::Value(right))))
            }
            Number::Tuple(left, right) => {
                if let Some(split) = left.clone().split() {
                    Some(Self::Tuple(Box::new(split), right))
                } else if let Some(split) = right.split() {
                    Some(Self::Tuple(left.clone(), Box::new(split)))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn add_left(self, n: usize) -> Number {
        match self {
            Number::Value(value) => Number::Value(value + n),
            Number::Tuple(left, right) => Number::Tuple(Box::new(left.add_left(n)), right),
        }
    }

    fn add_right(self, n: usize) -> Number {
        match self {
            Number::Value(value) => Number::Value(value + n),
            Number::Tuple(left, right) => Number::Tuple(left, Box::new(right.add_right(n))),
        }
    }

    fn explode(self, n: usize) -> (Option<Self>, usize, usize) {
        match self {
            Number::Tuple(left, right) if n >= 4 => {
                match (*left, *right) {
                    (Number::Value(left), Number::Value(right)) => (Some(Self::Value(0)), left, right),
                    (left, right) => {
                        let (lhs, lhs_left, lhs_right) = left.clone().explode(n + 1);
                        let (rhs, rhs_left, rhs_right) = right.clone().explode(n + 1);
                        if let Some(lhs) = lhs {
                            (Some(Number::Tuple(Box::new(lhs), Box::new(right.add_left(lhs_right)))), lhs_left, 0)
                        } else if let Some(rhs) = rhs {
                            (Some(Number::Tuple(Box::new(left.add_right(rhs_left)), Box::new(rhs))), 0, rhs_right)
                        } else {
                            (None, 0, 0)
                        }
                    }
                }
            }
            _ => (None, 0, 0)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
