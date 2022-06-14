// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License

#[derive(Debug, PartialOrd, PartialEq)]
struct ClassSort {
    probability: f64,
    classification: i64,
}

impl ClassSort {
    fn new(probability: f64, classification: i64) -> ClassSort {

        if probability > 1.0 || probability < 0.0 {
            panic!("Probabilities must be between 0.0 and 1.0.");
        }

        ClassSort { probability, classification }
    }
}


#[cfg(test)]
mod class_sort_tests {

    use super::*;

    #[test]
    fn test_initialize_classsort() {
        let cls = ClassSort::new(0.5, 1);
        assert_eq!(cls.probability, 0.5);
        assert_eq!(cls.classification, 1);
    }

    #[test]
    fn test_initialize_classsort0() {
        let cls = ClassSort::new(0.75, 0);
        assert_eq!(cls.probability, 0.75);
        assert_eq!(cls.classification, 0);
    }
}
