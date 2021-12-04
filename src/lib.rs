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

    // Shoutout for a tip from Globi on Discord that opened the way for this.
    macro_rules! iter_zip_twice {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Create iterators
            let mut largest;
            let mut smallest;

            if ($lhs.data.len() > $rhs.data.len()) {
                largest = $lhs.data.iter();
                smallest = $rhs.data.iter();
            } else {
                smallest = $lhs.data.iter();
                largest = $rhs.data.iter();
            }

            let smallest_size = std::cmp::min($lhs.data.len(),$rhs.data.len());
            let largest_size = std::cmp::max($lhs.data.len(),$rhs.data.len());

            // Do until b is exhausted.
            for (small, large) in smallest.zip(largest.by_ref()) {
                    let (value, tmp_carry) = small.carrying_add(*large, carry);
                    carry = tmp_carry;
                    $result.push(value);
            }
            // Do until a is also exhausted.
            for large in largest {
                if (carry) {
                    let (value, tmp_carry) = large.carrying_add(0u32, carry);
                    carry = tmp_carry;
                    $result.push(value); 
                } else { // if no carry bit, then u32 + 0 can't overflow.
                    let value = *large;
                    $result.push(value);
                }
            }
            

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
    }


//UNSAFE USE DESCRIPTION.
//Uses `unchecked_unwrap()`, which causes undefined behavior if the Option<T> would have been None.
//This is used in `iter_addition_unchecked!` to gain a significant performance increase (~10%). In
//it, two iterators are created that correspond to the underlying data structures of the lhs and rhs of
//an addition operator. The iterators are assigned `largest` and `smallest`, with sizes Nlarge and Nsmall.
//The algorithm calls both iterators until it has called them Nsmallest times. It then iterates the largest
//iterator (Nlargest - Nsmallest) times. In this, the undefined behavior is prevented through ensuring that
//the length is not exceeded.


    use unchecked_unwrap::UncheckedUnwrap;
    // Defines the algorithm behind the addition of two grou numbers.
    macro_rules! iter_addition_unchecked {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Create iterators
            let mut largest;
            let mut smallest;

            if ($lhs.data.len() > $rhs.data.len()) {
                largest = $lhs.data.iter();
                smallest = $rhs.data.iter();
            } else {
                smallest = $lhs.data.iter();
                largest = $rhs.data.iter();
            }

            let smallest_size = std::cmp::min($lhs.data.len(),$rhs.data.len());
            let largest_size = std::cmp::max($lhs.data.len(),$rhs.data.len());

            // Do until b is exhausted.
            for _i in 0..smallest_size {
                unsafe {
                    let (value, tmp_carry) = largest.next().unchecked_unwrap().carrying_add(*smallest.next().unchecked_unwrap(), carry);
                    carry = tmp_carry;
                    $result.push(value);
                }
            }
            // Do until a is also exhausted.
            for _i in smallest_size..largest_size {
                if (carry) {
                    unsafe {
                        let (value, tmp_carry) = largest.next().unchecked_unwrap().carrying_add(0u32, carry);
                        carry = tmp_carry;
                        $result.push(value); 
                    }
                } else { // if no carry bit, then u32 + 0 can't overflow.
                    let value;
                    unsafe {
                        value = *largest.next().unchecked_unwrap();
                    }
                    $result.push(value);
                }
            }
            

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
    }

    // Defines the algorithm behind the addition of two grou numbers.
    macro_rules! iter_addition_checked {
        ($lhs:expr, $rhs:expr, $result:expr ) => {
            let mut carry = false;

            // Create iterators
            let mut largest;
            let mut smallest;

            if ($lhs.data.len() > $rhs.data.len()) {
                largest = $lhs.data.iter();
                smallest = $rhs.data.iter();
            } else {
                smallest = $lhs.data.iter();
                largest = $rhs.data.iter();
            }
            
            let smallest_size = std::cmp::min($lhs.data.len(),$rhs.data.len());
            let largest_size = std::cmp::max($lhs.data.len(),$rhs.data.len());

            // Do until b is exhausted.
            for _i in 0..smallest_size {
                let (value, tmp_carry) = largest.next().unwrap().carrying_add(*smallest.next().unwrap(), carry);
                carry = tmp_carry;
                $result.push(value);
            }
            // Do until a is also exhausted.
            for _i in smallest_size..largest_size {
                if (carry) {
                    let (value, tmp_carry) = largest.next().unwrap().carrying_add(0u32, carry);
                    carry = tmp_carry;
                    $result.push(value); 
                } else { // if no carry bit, then u32 + 0 can't overflow.
                    let value = *largest.next().unwrap();
                    $result.push(value);
                }
            }
            

            // Final carry.
            if carry {
                $result.push(1);
            }
        };
    }


    /*
    use itertools::EitherOrBoth;
    use itertools::Itertools;

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
                    iter_zip_twice!(self, other, result.data);
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
                    iter_zip_twice!(self, other, final_vec);
                    self.data = final_vec;
                }
            }
        };
    }

    add_assign_impl_grou!(Grou);
    add_assign_impl_grou!(&Grou);
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
    fn test_operators_addition() {
        let u = Grou::from(100);
        let v = Grou::from(250);

        assert_eq!(u.clone() + v.clone(), Grou::from(350));
        assert_eq!(u.clone() + &v, Grou::from(350));
        assert_eq!(&u + v.clone(), Grou::from(350));
        assert_eq!(&u + &v, Grou::from(350));

        let mut u = Grou::from(100);
        u += v.clone();
        assert_eq!(u, Grou::from(350));
        u += &v;
        assert_eq!(u, Grou::from(600));
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

    #[test]
    fn test_overflow() {
        let mut g = Grou::from(u32::MAX);
        g += Grou::from(1);
        assert_eq!(Grou::from(vec![0,1]), g);
    }
}
