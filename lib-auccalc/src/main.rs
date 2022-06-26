// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

include!("confusion.rs");

fn main() {
    let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
    let ytrue = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];

    let confusion = Confusion::from_predictions(ytrue, ypred);

    println!(
        "Area Under the Curve for Precision - Recall is {}",
        confusion.calculate_auc_pr(0.0)
    );
    println!(
        "Area Under the Curve for ROC is {}",
        confusion.calculate_aucroc()
    );
}
