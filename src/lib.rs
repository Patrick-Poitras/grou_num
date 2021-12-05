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
    
    fn remove_trailing_zeros_reverse<'a>(vec:&'a Vec<u32>) -> Vec<&'a u32> {
        vec.iter().rev().peekable().skip_while(|&x| *x == 0).collect()
    }

    impl std::cmp::PartialOrd for Grou {
        fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> {
            let iter_self = remove_trailing_zeros_reverse(&self.data);
            let iter_other = remove_trailing_zeros_reverse(&other.data);

            if iter_self.len() != iter_other.len() {
                return iter_self.len().partial_cmp(&iter_other.len());
            } else {
                for (self_val, other_val) in iter_self.iter().zip(iter_other.iter()) {
                    if self_val != other_val {
                        return self_val.partial_cmp(other_val);
                    }
                }
                // At this point all values are identical.
                return Some(std::cmp::Ordering::Equal);
            }
        }
    }

    impl Grou {
        #[inline]
        fn trim(self: &mut Self) {
            let mut final_length = self.data.len();
            for val in self.data.iter().rev() {
                if *val == 0u32 {
                    final_length -= 1;
                } else {
                    break;
                }
            }
            if final_length == 0 {
                // Leave one 0;
                final_length = 1;
            }
            self.data.truncate(final_length);
        }
    }

    // Shoutout for a tip from Globi on Discord that opened the way for this.
    macro_rules! iter_addition {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Create iterators
            let mut largest;
            let smallest;

            if ($lhs.data.len() > $rhs.data.len()) {
                largest = $lhs.data.iter();
                smallest = $rhs.data.iter();
            } else {
                smallest = $lhs.data.iter();
                largest = $rhs.data.iter();
            }

            // Do until smallest iterator is exhausted.
            for (small, large) in smallest.zip(largest.by_ref()) {
                    let (value, tmp_carry) = small.carrying_add(*large, carry);
                    carry = tmp_carry;
                    $result.push(value);
            }

            // Do the rest.
            for large in largest {
                if (carry) {
                    let (value, tmp_carry) = large.carrying_add(0u32, carry);
                    carry = tmp_carry;
                    $result.push(value); 
                } else { // if no carry bit, then u32 + 0 can't overflow.
                    $result.push(*large);
                }
            }

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
    }

    /*
    macro_rules! iter_addition {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Iterate over a zip of the lhs and rhs.
            let maxlen = std::cmp::max($lhs.data.len(), $rhs.data.len());
            for index in 0..maxlen {
                //let a :u32 = *$lhs.data.get(index).unwrap_or(&0);
                //let b :u32 = *$rhs.data.get(index).unwrap_or(&0);

                let a = match $lhs.data.get(index) {
                    Some(v) => *v,
                    None => 0u32,
                };
                let b = match $rhs.data.get(index) {
                    Some(v) => *v,
                    None => 0u32,
                };

                let (value, tmp_carry) = a.carrying_add(b, carry);

                carry = tmp_carry;
                $result.push(value);
            }

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
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

    */

    macro_rules! addition_impl_grou {
        ($type1:ty, $type2:ty) => {
             impl std::ops::Add<$type2> for $type1 {
                type Output = Grou;

                fn add(self: Self, other: $type2) -> Grou {
                    let preallocation_size = std::cmp::max(self.data.len(), other.data.len()) + 1;
                    let mut result = Grou::empty(preallocation_size);
                    iter_addition!(self, other, result.data);
                    return result;
                }
            }  
        };
    }

    addition_impl_grou!(Grou,  Grou);
    addition_impl_grou!(&Grou, Grou);
    addition_impl_grou!(Grou, &Grou);
    addition_impl_grou!(&Grou, &Grou);

    macro_rules! add_assign_impl_grou {
        ($type2:ty) => {
            impl std::ops::AddAssign<$type2> for Grou {
                fn add_assign(self: &mut Grou, other: $type2) {
                    let preallocation_size = std::cmp::max(self.data.len(), other.data.len()) + 1;
                    let mut final_vec : Vec<u32> = Vec::with_capacity(preallocation_size);
                    iter_addition!(self, other, final_vec);
                    self.data = final_vec;
                }
            }
        };
    }

    add_assign_impl_grou!(Grou);
    add_assign_impl_grou!(&Grou);

    // Subtraction
    impl Grou {
        fn sub_unchecked(self: &Self, other: &Grou) -> Grou {
            let mut borrow = false;
            let mut result = Vec::<u32>::new();
            // Assume that self > other.
            let mut lhs = self.data.iter();
            let rhs = other.data.iter();
            for (j,i) in rhs.zip(lhs.by_ref()) {
                let (value, tmp_borrow) = i.borrowing_sub(*j, borrow);
                result.push(value);
                borrow = tmp_borrow;
            }

            // Finish using lhs.
            for i in lhs {
                if borrow {
                    let (value, tmp_borrow) = i.borrowing_sub(0u32, borrow);
                    result.push(value);
                    borrow = tmp_borrow;
                } else {
                    result.push(*i);
                }

            }

            let mut g = Grou::from(result);
            g.trim();
            return g;
        }
    }

    macro_rules! impl_sub {
        ($lhs: ty, $rhs: ty) => {
            impl std::ops::Sub<$rhs> for $lhs {
                type Output = Grou;
                fn sub(self, other: $rhs) -> Grou {
                    match self.partial_cmp(&other).unwrap() {
                        std::cmp::Ordering::Less => {panic!("Subtraction leads to underflow");},
                        std::cmp::Ordering::Equal => {return Grou::from(0);},
                        std::cmp::Ordering::Greater => {return self.sub_unchecked(&other);}
                    }
                }
            }            
        };
    }

    impl_sub!(Grou, Grou);
    impl_sub!(Grou, &Grou);
    impl_sub!(&Grou, Grou);
    impl_sub!(&Grou, &Grou);

}
