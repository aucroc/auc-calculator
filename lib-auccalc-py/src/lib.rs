// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

use lib_auccalc::confusion::Confusion as ConfusionRs;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyclass]
struct Confusion {
    confusion: ConfusionRs,
}

#[pymethods]
impl Confusion {
    #[new]
    fn new(ytrue: Vec<i64>, ypred: Vec<f64>) -> PyResult<Self> {
        let confusion = ConfusionRs::from_predictions(ytrue, ypred);
        Ok(Self { confusion })
    }

    #[args()]
    fn aucroc(&self) -> PyResult<f64> {
        Ok(self.confusion.calculate_aucroc())
    }

    #[args(min_recall = "0.0")]
    fn aucpr(&self, min_recall: f64) -> PyResult<f64> {
        Ok(self.confusion.calculate_auc_pr(min_recall))
    }
}

#[pyfunction]
fn roc_auc_score(ytrue: Vec<i64>, ypred: Vec<f64>) -> f64 {
    let confusion = ConfusionRs::from_predictions(ytrue, ypred);
    return confusion.calculate_aucroc();
}

// `metrics` module, implementing `scikit-learn` score functions
#[pymodule]
fn metrics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(roc_auc_score, m)?)?;
    Ok(())
}

#[pymodule]
fn _auccalculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_class::<Confusion>()?;
    m.add_wrapped(wrap_pymodule!(metrics))?;
    Ok(())
}

#[cfg(test)]
mod lib_auccalc_py_tests {

    extern crate float_cmp;
    use float_cmp::approx_eq;

    use crate::ConfusionRs;

    #[test]
    fn test_confusion_from_lib_auccalc() {
        let ytrue = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];
        let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
        let confusion = ConfusionRs::from_predictions(ytrue, ypred);

        assert!(approx_eq!(
            f64,
            confusion.calculate_auc_pr(0.0),
            0.8243055555555555,
            epsilon = 0.0001
        ));
        assert!(approx_eq!(
            f64,
            confusion.calculate_aucroc(),
            0.75,
            epsilon = 0.0001
        ));
    }
}
