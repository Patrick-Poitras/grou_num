#![feature(bigint_helper_methods)]

pub mod grou {
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Grou {
        data: Vec<u32>,
    }

    impl Grou {
        // Empty array. Preallocated to size.
        pub fn empty(size: usize) -> Grou {
            Grou { data: Vec::with_capacity(size) }
        }
    }

    impl From<u32> for Grou {
        fn from(small_num: u32) -> Grou {
            Grou {
                data: vec![small_num],
            }
        }
    }

    impl From<Vec<u32>> for Grou {
        fn from(num: Vec<u32>) -> Grou {
            Grou { data: num }
        }
    }

    use itertools::EitherOrBoth;
    use itertools::Itertools;

    macro_rules! iter_zip_addition {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Iterate over a zip of the lhs and rhs.
            for val in $lhs.data.iter().zip_longest($rhs.data.iter()) {
                // When either side's iterator is None, because it reached the end, the carrying_add 
                // method will be called with 0 in the place of the other.
                let (value, tmp_carry) = match val {
                    EitherOrBoth::Both(i, j)                       => i.carrying_add(*j, carry),
                    EitherOrBoth::Left(i) | EitherOrBoth::Right(i) => 0u32.carrying_add(*i, carry),
                };

                carry = tmp_carry;
                $result.push(value);
            }

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
    }

    impl std::ops::Add for Grou {
        type Output = Grou;

        fn add(self, other: Grou) -> Grou {
            let preallocation_size = std::cmp::max(self.data.len(), other.data.len()) + 1;
            let mut result = Grou::empty(preallocation_size);
            iter_zip_addition!(self, other, result.data);
            return result;
        }
    }

    impl std::ops::AddAssign for Grou {
        fn add_assign(self: &mut Grou, other: Grou) {
            let preallocation_size = std::cmp::max(self.data.len(), other.data.len()) + 1;
            let mut final_vec : Vec<u32> = Vec::with_capacity(preallocation_size);
            iter_zip_addition!(self, other, final_vec);
            self.data = final_vec;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::grou::Grou;

    #[test]
    fn test_small_addition() {
        let u = Grou::from(100);
        let v = Grou::from(u32::MAX);

        let w = u.clone() + v.clone();
        assert_eq!(w, Grou::from(vec![99, 1]));
        let w3 = w.clone() + w.clone() + w;
        assert_eq!(w3, Grou::from(vec![297, 3]));

        let mut x = Grou::from(u32::MAX);
        x += u.clone();
        assert_eq!(v + u, x);
    }

    #[test]
    fn test_uneven_lengths() {
        let u = Grou::from(vec![1, 2, 3, 4, 5]);
        let v = Grou::from(vec![1]);

        assert_eq!(Grou::from(vec![2, 2, 3, 4, 5]), u + v);

        let u = Grou::from(vec![1, 2, 3, 4, 5]);
        let v = Grou::from(vec![]);

        assert_eq!(Grou::from(vec![1, 2, 3, 4, 5]), u + v);

        assert_eq!(Grou::from(vec![]), Grou::from(vec![]) + Grou::from(vec![]));
    }
}
