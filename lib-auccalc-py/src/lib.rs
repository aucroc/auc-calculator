// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

use pyo3::prelude::*;
use lib_auccalc::confusion::Confusion;

#[pyclass]
struct ConfusionPy {
    confusion: Confusion,
}

#[pymethods]
impl ConfusionPy {
    #[new]
    fn new(ypred: Vec<f64>, ytrue: Vec<i64>) -> PyResult<Self> {
        let confusion = Confusion::from_predictions(ypred, ytrue);
        Ok(Self { confusion })
    }

    #[args()]
    fn aucroc(&self) -> PyResult<f64> {
        Ok(self.confusion.calculate_aucroc())
    }

    #[args(min_recall="0.0")]
    fn aucpr(&self, min_recall: f64) -> PyResult<f64> {
        Ok(self.confusion.calculate_auc_pr(min_recall))
    }
}

#[pymodule]
fn auccalc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConfusionPy>()?;
    Ok(())
}


#[cfg(test)]
mod lib_auccalc_py_tests {

    extern crate float_cmp;
    use float_cmp::approx_eq;

    use crate::Confusion;

    #[test]
    fn test_confusion_from_lib_auccalc() {
        let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
        let ytrue = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];
        let confusion = Confusion::from_predictions(ypred, ytrue);

        assert!(approx_eq!(f64, confusion.calculate_auc_pr(0.0), 0.8243055555555555, epsilon = 0.0001));
        assert!(approx_eq!(f64, confusion.calculate_aucroc(), 0.75, epsilon = 0.0001));
    }
}
