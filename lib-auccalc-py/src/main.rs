// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

use lib_auccalc::confusion::Confusion;

fn main() {

    let confusion = Confusion::from_predictions(
        vec![0.5, 0.7, 0.1, 0.8],
        vec![0, 1, 1, 0],
    );

    println!("AUCPR:  {}", confusion.calculate_auc_pr(0.0));
    println!("AUCROC: {}", confusion.calculate_aucroc());
}
