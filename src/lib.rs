#![feature(bigint_helper_methods)]
#![feature(destructuring_assignment)]

pub mod radix_convert;

pub mod grou {
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Grou {
        data: Vec<u64>,
    }

    impl Grou {
        // Empty array. Preallocated to size.
        pub fn empty(size: usize) -> Grou {
            Grou {
                data: Vec::with_capacity(size),
            }
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

    /// The radix to Grou conversion takes in a string, and
    /// converts it to a Grou unsigned integer. This conversion
    /// is dependant on the prefix of the string, where:
    /// 0x: indicates hexadecimal
    /// 0b: indicates binary
    ///   : indicates decimal
    /// 
    /// To get an explicit conversion, you can use the functions
    /// Grou::from_dec(), Grou::from_hex(), and Grou::from_bin().
    //impl From<&str> for Grou;

    macro_rules! impl_partial_cmp {
        ($type1: ty) => {
            impl std::cmp::PartialOrd for $type1 {
                fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> {
                    let mut self_len = self.data.len();
                    let mut iter_self = self.data.iter().rev().peekable();

                    let mut other_len = other.data.len();
                    let mut iter_other = other.data.iter().rev().peekable();

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
        };
    }

    impl_partial_cmp!(Grou);
    impl_partial_cmp!(GrouSubset<'_>);

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

    impl Grou {
        pub fn addition_small(&mut self, rhs:u64) {
            if self.data.len() == 0 {
                self.data.push(rhs);
                return;
            }
            let mut carry = false;
            let mut local_rhs = rhs;
            for val in self.data.iter_mut() {
                let (value, tmp_carry) = val.carrying_add(local_rhs, carry);
                *val = value;
                carry = tmp_carry;

                local_rhs = 0;
                if !carry {
                    break;
                }
            }

            if carry {
                self.data.push(1);
            }
        }
    }

    impl std::ops::Add<u64> for Grou {
        type Output = Grou;
        fn add(self, rhs:u64) -> Grou {
            let mut ret_grou = self.clone();
            ret_grou.addition_small(rhs);
            ret_grou
        }
    }

    impl std::ops::AddAssign<u64> for Grou {
        fn add_assign(&mut self, rhs:u64) {
            self.addition_small(rhs);
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
                } else {
                    // if no carry bit, then u32 + 0 can't overflow.
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

    addition_impl_grou!(Grou, Grou);
    addition_impl_grou!(&Grou, Grou);
    addition_impl_grou!(Grou, &Grou);
    addition_impl_grou!(&Grou, &Grou);

    macro_rules! add_assign_impl_grou {
        ($type2:ty) => {
            impl std::ops::AddAssign<$type2> for Grou {
                fn add_assign(self: &mut Grou, other: $type2) {
                    let preallocation_size = std::cmp::max(self.data.len(), other.data.len()) + 1;
                    let mut final_vec: Vec<u64> = Vec::with_capacity(preallocation_size);
                    iter_addition!(self, other, final_vec);
                    self.data = final_vec;
                }
            }
        };
    }

    add_assign_impl_grou!(Grou);
    add_assign_impl_grou!(&Grou);

    macro_rules! impl_sub_unchecked {
        ($lhs: ty, $rhs: ty) => {
            impl $lhs {
                fn sub_unchecked(self: &Self, other: &$rhs) -> Grou {
                    let mut borrow = false;
                    let mut result = Vec::<u64>::new();

                    // Assume that self > other.
                    let mut lhs = self.data.iter();
                    let rhs = other.data.iter();
                    for (j, i) in rhs.zip(lhs.by_ref()) {
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
        };
    }

    impl_sub_unchecked!(Grou, Grou);
    impl_sub_unchecked!(GrouSubset<'_>, GrouSubset<'_>);

    macro_rules! impl_sub {
        ($lhs: ty, $rhs: ty) => {
            impl std::ops::Sub<$rhs> for $lhs {
                type Output = Grou;
                fn sub(self, other: $rhs) -> Grou {
                    match self.partial_cmp(&other).unwrap() {
                        std::cmp::Ordering::Less => {
                            panic!("Subtraction leads to underflow");
                        }
                        std::cmp::Ordering::Equal => {
                            return Grou::from(0);
                        }
                        std::cmp::Ordering::Greater => {
                            return self.sub_unchecked(&other);
                        }
                    }
                }
            }
        };
    }

    impl_sub!(Grou, Grou);
    impl_sub!(Grou, &Grou);
    impl_sub!(&Grou, Grou);
    impl_sub!(&Grou, &Grou);

    macro_rules! impl_sub_with_sign {
        ($lhs:ty, $rhs:ty) => {
            impl $lhs {
                pub fn sub_with_sign(&self, other: $rhs) -> (bool, Grou) {
                    match self.partial_cmp(other).unwrap() {
                        std::cmp::Ordering::Less => return (true, other.sub_unchecked(self)),
                        std::cmp::Ordering::Equal => return (false, Grou::from(0)),
                        std::cmp::Ordering::Greater => return (true, self.sub_unchecked(other)),
                    }
                }
            }
        };
    }

    impl_sub_with_sign!(Grou, &Grou);
    impl_sub_with_sign!(GrouSubset<'_>, &GrouSubset<'_>);

    impl Grou {
        pub fn subset<'a>(self: &'a Self, start: usize, end: usize) -> GrouSubset<'a> {
            return GrouSubset {
                data: &self.data[start..end],
            };
        }

        pub fn subset_all<'a>(self: &'a Self) -> GrouSubset<'a> {
            GrouSubset {
                data: &self.data[..],
            }
        }

        pub fn split_off_block<'a>(self: &'a Self, length: usize, start: usize) -> GrouSubset<'a> {
            if self.data.len() <= start {
                return GrouSubset {
                    data: &self.data[0..0],
                };
            } else if self.data.len() - start < length {
                return GrouSubset {
                    data: &self.data[start..],
                };
            } else {
                return GrouSubset {
                    data: &self.data[start..start + length],
                };
            }
        }

        // TODO: Refactor this mess.
        pub fn split_2<'a>(self: &'a Self) -> (GrouSubset<'a>, GrouSubset<'a>) {
            let mut x = self.make_chunks(2).map(|x| GrouSubset { data: x });

            let i = x.next().unwrap_or(GrouSubset { data: &[] });
            let j = x.next().unwrap_or(GrouSubset { data: &[] });

            (i, j)
        }

        pub fn split_3<'a>(self: &'a Self) -> (GrouSubset<'a>, GrouSubset<'a>, GrouSubset<'a>) {
            let mut x = self.make_chunks(3).map(|x| GrouSubset { data: x });

            let i = x.next().unwrap_or(GrouSubset { data: &[] });
            let j = x.next().unwrap_or(GrouSubset { data: &[] });
            let k = x.next().unwrap_or(GrouSubset { data: &[] });

            (i, j, k)
        }

        // Splits the number into N chunks. Thanks to "The Lua Moon" on Discord
        // for the suggestion.
        fn make_chunks<'a>(self: &'a Self, n: usize) -> std::slice::Chunks<'a, u64> {
            let offset = match self.data.len() % n {
                0 => 0,
                _ => 1,
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
                fn add(self, other: $type2) -> Grou {
                    let mut results = Vec::new();
                    iter_addition!(self, other, results);
                    return Grou { data: results };
                }
            }
        };
    }

    impl_addition_grousubset!(GrouSubset<'_>, GrouSubset<'_>);
    impl_addition_grousubset!(GrouSubset<'_>, &GrouSubset<'_>);
    impl_addition_grousubset!(&GrouSubset<'_>, GrouSubset<'_>);
    impl_addition_grousubset!(&GrouSubset<'_>, &GrouSubset<'_>);

    //Implement GrouSubset/Grou addition
    impl_addition_grousubset!(GrouSubset<'_>, Grou);
    impl_addition_grousubset!(GrouSubset<'_>, &Grou);
    impl_addition_grousubset!(&GrouSubset<'_>, Grou);
    impl_addition_grousubset!(&GrouSubset<'_>, &Grou);
    impl_addition_grousubset!(Grou, GrouSubset<'_>);
    impl_addition_grousubset!(Grou, &GrouSubset<'_>);
    impl_addition_grousubset!(&Grou, GrouSubset<'_>);
    impl_addition_grousubset!(&Grou, &GrouSubset<'_>);

    impl Grou {
        // Underlying function for Mul<u64> and MulAssign<u64>
        pub fn multiply_small(&mut self, rhs: u64) {
            let mut carry = 0u64;
            for val in self.data.iter_mut() {
                let (value, tmp_carry) = val.carrying_mul(rhs, carry);
                *val = value;
                carry = tmp_carry;
            }
            if carry > 0 {
                self.data.push(carry);
            }
        }
    }

    impl std::ops::Mul<u64> for Grou {
        type Output = Grou;
        fn mul(self, rhs:u64) -> Grou {
            let mut ret_grou = self.clone();
            ret_grou.multiply_small(rhs);
            return ret_grou;
        }
    }

    impl std::ops::MulAssign<u64> for Grou {
        fn mul_assign(&mut self, rhs:u64) {
            self.multiply_small(rhs);
        }
    }

    impl Grou {
        // Performs the multiplication of a GrouSubset and an u64, and adds the
        // result to the value in self.
        pub fn add_multiply_result<'a>(&mut self, lhs: &GrouSubset<'a>, rhs: u64, offset: usize) {
            let mut carry = 0u64;
            let mut megacarry = false;

            for (ind, val) in lhs.data.iter().enumerate() {
                let effective_ind = offset + ind;
                // Add another box if lhs.data > self.data
                while self.data.len() <= effective_ind {
                    self.data.push(0);
                }
                let (value, tmp_carry) = val.carrying_mul(rhs, carry);
                let (value, tmp_carry_grou) = self.data[effective_ind].carrying_add(value, false);
                self.data[effective_ind] = value;
                if tmp_carry_grou || megacarry {
                    let (tmp_carry, overflow) =
                        tmp_carry.carrying_add(tmp_carry_grou as u64, megacarry);
                    carry = tmp_carry;
                    megacarry = overflow;
                } else {
                    carry = tmp_carry;
                    megacarry = false;
                }
            }

            let mut current_index = lhs.data.len();
            while carry > 0 || megacarry {
                if self.data.len() == current_index {
                    self.data.push(0);
                }

                let (value, tmp_carry) = self.data[current_index].carrying_add(carry, megacarry);
                self.data[current_index] = value;
                carry = 0;
                megacarry = tmp_carry;
                current_index += 1;
            }
        }

        #[inline]
        pub fn move_vec_elements_right(&mut self, shift: usize) {
            self.data.resize(self.data.len() + shift, 0);
            self.data.rotate_right(shift);
        }
    }

    impl GrouSubset<'_> {
        // Straight multiplication of two grou subsets of length
        // A and B respectively will have A*B operations.
        fn multiply_straight<'a>(&self, rhs: &GrouSubset<'a>) -> Grou {
            let mut ret_grou = Grou::empty(self.data.len() + rhs.data.len());
            for (ind, rhs_value) in rhs.data.iter().enumerate() {
                ret_grou.add_multiply_result(self, *rhs_value, ind);
            }
            ret_grou.trim();
            return ret_grou;
        }
    }

    impl std::ops::Mul<&GrouSubset<'_>> for &GrouSubset<'_> {
        type Output = Grou;
        fn mul<'a>(self, rhs: &GrouSubset<'a>) -> Grou {
            self.multiply_straight(rhs)
        }
    }

    // Implementing Karatsuba.
    impl Grou {
        pub fn karatsuba_mul(&self, rhs: &Grou) -> Grou {
            // Step 1: Split into GrouSubsets
            let block_length = std::cmp::max(self.data.len(), rhs.data.len());
            let block_length = block_length / 2 + block_length % 2;
            let a0 = self.split_off_block(block_length, 0);
            let a1 = self.split_off_block(block_length, a0.data.len());
            let b0 = rhs.split_off_block(block_length, 0);
            let b1 = rhs.split_off_block(block_length, b0.data.len());

            // Make temporary values.
            let high = &b1 * &a1;
            let low = &b0 * &a0;
            let mut t0 = &high + &low;

            let mut t1 = high;
            t1.move_vec_elements_right(2 * block_length);
            t1 += low;

            let (sign_a, delta_a) = &a1.sub_with_sign(&a0);
            let (sign_b, delta_b) = &b1.sub_with_sign(&b0);

            let mut t2 = &delta_a.subset_all() * &delta_b.subset_all();

            t0.move_vec_elements_right(block_length);
            t2.move_vec_elements_right(block_length);

            //sign = true => addition, subtraction otherwise.
            let sign = sign_a ^ sign_b;
            if sign {
                return t0 + t1 + t2;
            } else {
                return (t0 + t1) - t2;
            }
        }
    }
}