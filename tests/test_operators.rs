#[cfg(test)]
mod addition {
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
