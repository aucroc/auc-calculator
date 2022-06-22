// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct PNPoint {
    pos: f64,
    neg: f64,
}

impl PNPoint {
    fn new(pos: f64, neg: f64) -> PNPoint {

        if pos < 0.0 || neg < 0.0 {
            return PNPoint { pos: 0.0, neg: 0.0 }
        }

        PNPoint { pos, neg }
    }
}


#[cfg(test)]
mod pnpoint_tests {

    use super::*;

    #[test]
    fn test_initialize_pnpoint() {
        let pnt = PNPoint::new(1.0, 1.0);
        assert_eq!(pnt.pos, 1.0);
        assert_eq!(pnt.neg, 1.0);
    }

    #[test]
    fn test_initialize_negative_pos_pnpoint() {
        let pnt = PNPoint::new(-1.0, 1.0);
        assert_eq!(pnt.pos, 0.0);
        assert_eq!(pnt.neg, 0.0);
    }

    #[test]
    fn test_initialize_negative_neg_pnpoint() {
        let pnt = PNPoint::new(1.0, -1.0);
        assert_eq!(pnt.pos, 0.0);
        assert_eq!(pnt.neg, 0.0);
    }

    #[test]
    fn test_compare_equal_points() {
        let pnt1 = PNPoint::new(1.0, 1.0);
        let pnt2 = PNPoint::new(1.0, 1.0);
        assert_eq!(pnt1, pnt2);
    }

    #[test]
    fn test_compare_unequal_points1() {
        let pnt1 = PNPoint::new(1.0, 1.0);
        let pnt2 = PNPoint::new(1.0, 3.0);
        assert_ne!(pnt1, pnt2);
    }

    #[test]
    fn test_compare_unequal_points2() {
        let pnt1 = PNPoint::new(3.0, 1.0);
        let pnt2 = PNPoint::new(1.0, 1.0);
        assert_ne!(pnt1, pnt2);
    }

    #[test]
    fn test_sort_pnpoints1() {
        let mut actual = vec![
            PNPoint::new(2.0, 1.0),
            PNPoint::new(1.0, 1.0),
            PNPoint::new(3.0, 1.0),
        ];

        let expected = vec![
            PNPoint::new(1.0, 1.0),
            PNPoint::new(2.0, 1.0),
            PNPoint::new(3.0, 1.0),
        ];

        actual.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sort_pnpoints2() {
        let mut actual = vec![
            PNPoint::new(1.0, 2.0),
            PNPoint::new(1.0, 1.0),
            PNPoint::new(1.0, 3.0),
        ];

        let expected = vec![
            PNPoint::new(1.0, 1.0),
            PNPoint::new(1.0, 2.0),
            PNPoint::new(1.0, 3.0),
        ];

        actual.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sort_pnpoints3() {
        let mut actual = vec![
            PNPoint::new(3.0, 2.0),
            PNPoint::new(2.0, 2.0),
            PNPoint::new(2.0, 1.0),
            PNPoint::new(1.0, 2.0),
            PNPoint::new(3.0, 1.0),
        ];

        let expected = vec![
            PNPoint::new(1.0, 2.0),
            PNPoint::new(2.0, 1.0),
            PNPoint::new(2.0, 2.0),
            PNPoint::new(3.0, 1.0),
            PNPoint::new(3.0, 2.0),
        ];

        actual.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(actual, expected);
    }
}
