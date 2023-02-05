use serde_json::Value;
use std::cmp::Ordering;
use std::iter::zip;
use std::str::FromStr;

pub struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    /// # Examples
    /// ```
    /// use advent_of_code_2022_13::{Pair, Packet};
    ///
    /// assert!(Pair::new("[1,1,3,1,1]\n[1,1,5,1,1]").is_in_order());
    /// assert!(Pair::new("[[1],[2,3,4]]\n[[1],4]").is_in_order());
    /// assert!(!Pair::new("[9]\n[[8,7,6]]").is_in_order());
    /// assert!(Pair::new("[[4,4],4,4]\n[[4,4],4,4,4]").is_in_order());
    /// assert!(!Pair::new("[7,7,7,7]\n[7,7,7]").is_in_order());
    /// assert!(Pair::new("[]\n[3]").is_in_order());
    /// assert!(!Pair::new("[[[]]]\n[[]]").is_in_order());
    /// assert!(!Pair::new("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]").is_in_order());
    /// ```
    pub fn is_in_order(&self) -> bool {
        self.left <= self.right
    }

    pub fn new(instring: &str) -> Self {
        let lines: Vec<_> = instring.lines().collect();

        Pair {
            left: lines[0].parse().unwrap(),
            right: lines[1].parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left.partial_cmp(right),
            (Self::List(left), Self::List(right)) => {
                for (l, r) in zip(left, right) {
                    if l < r {
                        return Some(Ordering::Less);
                    } else if l > r {
                        return Some(Ordering::Greater);
                    }
                }

                Some(left.len().cmp(&right.len()))
            }
            (Self::Int(left), Self::List(right)) => {
                Packet::List(vec![Packet::Int(*left)]).partial_cmp(&Packet::List((*right).clone()))
            }
            (Self::List(left), Self::Int(right)) => {
                Packet::List((*left).clone()).partial_cmp(&Packet::List(vec![Packet::Int(*right)]))
            }
        }
    }
}

impl FromStr for Packet {
    type Err = std::io::Error;

    /// # Examples
    /// ```
    /// use advent_of_code_2022_13::Packet;
    ///
    /// assert_eq!(
    ///     "[1,1,3,1,1]".parse::<Packet>().unwrap(),
    ///     Packet::List([1,1,3,1,1].iter().map(|x| Packet::Int(*x)).collect())
    /// );
    /// assert_eq!(
    ///     "[9]".parse::<Packet>().unwrap(),
    ///     Packet::List(vec![Packet::Int(9)])
    /// );
    /// assert_eq!(
    ///     "[]".parse::<Packet>().unwrap(),
    ///     Packet::List(vec![])
    /// );
    /// assert_eq!(
    ///     "[[8,7,6]]".parse::<Packet>().unwrap(),
    ///     Packet::List(vec![Packet::List([8,7,6].iter().map(|x| Packet::Int(*x)).collect())])
    /// );
    /// assert_eq!(
    ///     "[[1],[2,3,4]]".parse::<Packet>().unwrap(),
    ///     Packet::List(vec![Packet::List(vec![Packet::Int(1)]),Packet::List([2,3,4].iter().map(|x| Packet::Int(*x)).collect())])
    /// );
    /// assert_eq!(
    ///     "[[4,4],4,4]".parse::<Packet>().unwrap(),
    ///     Packet::List(vec![Packet::List(vec![Packet::Int(4), Packet::Int(4)]), Packet::Int(4), Packet::Int(4)])
    /// );
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_json_val(&serde_json::from_str::<Value>(s).expect("Should parse as json"))
    }
}

impl Packet {
    fn from_json_val(val: &Value) -> Result<Self, std::io::Error> {
        match val {
            Value::Number(n) => Ok(Packet::Int(n.as_i64().unwrap() as u8)),
            Value::Array(v) => Ok(Packet::List(
                v.iter().map(|x| Self::from_json_val(x).unwrap()).collect(),
            )),
            _ => panic!("This should never happen!"),
        }
    }
}

/// Finds the pairs of packets in the correct order and returns the sum of their indices
/// # Examples
/// ```
/// use advent_of_code_2022_13::sum_correct;
///
/// assert_eq!(
///     13,
///     sum_correct(concat!(
///     "[1,1,3,1,1]\n",
///     "[1,1,5,1,1]\n",
///     "\n",
///     "[[1],[2,3,4]]\n",
///     "[[1],4]\n",
///     "\n",
///     "[9]\n",
///     "[[8,7,6]]\n",
///     "\n",
///     "[[4,4],4,4]\n",
///     "[[4,4],4,4,4]\n",
///     "\n",
///     "[7,7,7,7]\n",
///     "[7,7,7]\n",
///     "\n",
///     "[]\n",
///     "[3]\n",
///     "\n",
///     "[[[]]]\n",
///     "[[]]\n",
///     "\n",
///     "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
///     "[1,[2,[3,[4,[5,6,0]]]],8,9]"
/// )));
/// ```
pub fn sum_correct(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Pair::new)
        .enumerate()
        .filter(|(_, p)| p.is_in_order())
        .map(|(i, _)| i + 1)
        .sum()
}
