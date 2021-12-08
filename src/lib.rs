#![feature(bigint_helper_methods)]

pub mod grou {
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Grou {
        data: Vec<u64>,
    }

    impl Grou {
        // Empty array. Preallocated to size.
        pub fn empty(size: usize) -> Grou {
            Grou { data: Vec::with_capacity(size) }
        }
    }

    impl From<u64> for Grou {
        fn from(small_num: u64) -> Grou {
            Grou {
                data: vec![small_num],
            }
        }
    }

    impl From<Vec<u64>> for Grou {
        fn from(num: Vec<u64>) -> Grou {
            Grou { data: num }
        }
    }

    impl std::cmp::PartialOrd for Grou {
        fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> {
            let mut self_len = self.data.len();
            let mut iter_self =  self.data.iter().rev().peekable();

            let mut other_len = other.data.len();
            let mut iter_other =  other.data.iter().rev().peekable();

            while iter_self.next_if_eq(&&0u64).is_some() {
                self_len -= 1;
            }

            while iter_other.next_if_eq(&&0u64).is_some() {
                other_len -= 1;
            }


            if self_len != other_len {
                return self_len.partial_cmp(&other_len);
            } else {
                for (self_val, other_val) in iter_self.zip(iter_other) {
                    if self_val != other_val {
                        return self_val.partial_cmp(&other_val);
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
                if *val == 0u64 {
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
                    let (value, tmp_carry) = large.carrying_add(0u64, carry);
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
                    let mut final_vec : Vec<u64> = Vec::with_capacity(preallocation_size);
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
            let mut result = Vec::<u64>::new();
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
                    let (value, tmp_borrow) = i.borrowing_sub(0u64, borrow);
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

    impl Grou {
        pub fn subset<'a>(self: &'a Self, start: usize, end: usize) -> GrouSubset<'a> {
            return GrouSubset{data: &self.data[start..end]};
        }

        pub fn subset_all<'a>(self: &'a Self) -> GrouSubset<'a> {
            GrouSubset {data: &self.data[..]}
        }

        // TODO: Refactor this mess.
        pub fn split_2<'a>(self: &'a Self) -> (GrouSubset<'a>, GrouSubset<'a>) {
            let mut x = self.make_chunks(2).map(|x| GrouSubset {data:x});

            let i = x.next().unwrap_or(GrouSubset{data:&[]});
            let j = x.next().unwrap_or(GrouSubset{data:&[]});

            (i, j)
        }

        pub fn split_3<'a>(self: &'a Self) -> (GrouSubset<'a>, GrouSubset<'a>, GrouSubset<'a>) {
            let mut x = self.make_chunks(3).map(|x| GrouSubset {data:x});

            let i = x.next().unwrap_or(GrouSubset{data:&[]});
            let j = x.next().unwrap_or(GrouSubset{data:&[]});
            let k = x.next().unwrap_or(GrouSubset{data:&[]});

            (i,j,k)
        }

        // Splits the number into N chunks. Thanks to "The Lua Moon" on Discord
        // for the suggestion.
        fn make_chunks<'a>(self: &'a Self, n: usize) -> std::slice::Chunks<'a, u64> {
            let offset = match self.data.len() % n {
                0 => 0,
                _ => 1
            };
            let chunk_length = self.data.len() / n + offset;
            return self.data[..].chunks(chunk_length);
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct GrouSubset<'a> {
        pub data: &'a [u64],
    }

    macro_rules! impl_addition_grousubset {
        ($type1: ty, $type2: ty) => {
            impl std::ops::Add<$type2> for $type1 {
                type Output = Grou;
                fn add(self, other: $type2 ) -> Grou {
                    let mut results = Vec::new();
                    iter_addition!(self, other, results);
                    return Grou{data:results};
                }
            }
        };
    }

    impl_addition_grousubset!(GrouSubset<'_>, GrouSubset<'_>);
    impl_addition_grousubset!(GrouSubset<'_>, &GrouSubset<'_>);
    impl_addition_grousubset!(&GrouSubset<'_>, GrouSubset<'_>);
    impl_addition_grousubset!(&GrouSubset<'_>, &GrouSubset<'_>);

    
}


#[test]
fn test_grousubset() {
    use crate::grou::Grou;

    let g = Grou::from(vec![1,2,3,4,5,6,7,8,9]);
    let gs1 = g.subset(0, 3);
    let gs2 = g.subset(3, 6);

    let mut result = Vec::new();
    let mut carry = false;

    let gs3 = gs1.clone();
    let gs4 = gs2.clone();

    for (i, j) in gs1.data.iter().zip(gs2.data.iter()) {
        let (value, tmp_carry) = i.carrying_add(*j, carry);
        result.push(value);
        carry = tmp_carry;
    }

    assert_eq!(result, vec![5,7,9]);
    
    assert_eq!(Grou::from(result), (gs3 + &gs4));
}

#[test]
fn test_split() {
    use crate::grou::Grou;

    let g = Grou::from(vec![1,2,3,4,5,6,7,8,9,10,11]);

    //Split 2
    let (g1, g2) = g.split_2();
    assert_eq!(g1, Grou::from(vec![1,2,3,4,5,6]).subset_all());
    assert_eq!(g2, Grou::from(vec![7,8,9,10,11]).subset_all());

    let (g1, g2, g3) = g.split_3();
    assert_eq!(g1, Grou::from(vec![1,2,3,4]).subset_all());
    assert_eq!(g2, Grou::from(vec![5,6,7,8]).subset_all());
    assert_eq!(g3, Grou::from(vec![9,10,11]).subset_all());

    let g = Grou::from(vec![1,]);
    let (g1, g2) = g.split_2();
    assert_eq!(g1, Grou::from(vec![1]).subset_all());
    assert_eq!(g2, Grou::from(vec![]).subset_all());

    let g = Grou::from(vec![1,2]);
    let (g1, g2, g3) = g.split_3();
    assert_eq!(g1, Grou::from(vec![1]).subset_all());
    assert_eq!(g2, Grou::from(vec![2]).subset_all());
    assert_eq!(g3, Grou::from(vec![]).subset_all());
}