#[cfg(test)]
mod addition_tests {
    use grou_num::grou::Grou;

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

        let u = u32::MAX;
        let mut g = Grou::from(vec![0,1,1,1,1,1,1,1,1,1]);
        g +=        Grou::from(vec![u,u,u,u,u,u,u,u,u,u]);
        assert_eq!( Grou::from(vec![u,0,1,1,1,1,1,1,1,1,1]), g);
    }
}

#[cfg(test)]
mod partial_ordering_tests {
    use grou_num::grou::Grou;
    #[test]
    fn test_partial_ordering() {
        // u < v < w < x < y
        let u = Grou::from(vec![0,2,0,4,5,0]);
        let v = Grou::from(vec![1,2,3,4,5,0,0,0,0]);
        let w = Grou::from(vec![2,2,3,4,5]);
        let x = Grou::from(vec![0,2,0,4,6,0]);
        let y = Grou::from(vec![1,2,3,4,6]);

        let vals = vec![u,v,w,x,y];
        for i in 0..(vals.len()) {
            for j in i+1..(vals.len()) {
                assert_eq!(vals[i] < vals[j], true, "{:?} < {:?} failed", vals[i], vals[j]);
                assert_eq!(vals[i] > vals[j], false, "{:?} > {:?} failed", vals[i], vals[j]);
            }
        }

        assert_eq!(Grou::from(vec![]) > Grou::from(vec![]), false);
        assert_eq!(Grou::from(vec![]) < Grou::from(vec![]), false);

    }
}

#[cfg(test)]
mod subtraction_tests {
    #[test]
    fn test_subtract(){
        use grou_num::grou::Grou;
        assert_eq!(Grou::from(vec![1,2,3]) - Grou::from(vec![1,2,3]), Grou::from(0));
        assert_eq!(Grou::from(vec![6,4,3]) - Grou::from(vec![6,5,1]), Grou::from(vec![0,u32::MAX,1]));

        let u = Grou::from(6);
        let v = Grou::from(10);
        assert_eq!(v.clone()-u.clone(), Grou::from(4));
        assert_eq!(&v-u.clone(), Grou::from(4));
        assert_eq!(v.clone()-&u, Grou::from(4));
        assert_eq!(&v - &u, Grou::from(4));
    }
}


