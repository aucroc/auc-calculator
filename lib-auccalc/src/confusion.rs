// Copyright © 2022 Alexander L. Hayes
// Apache 2.0 License or MIT License

include!("class_sort.rs");
include!("pnpoint.rs");

#[derive(Debug)]
pub struct Confusion {
    tot_pos: f64,
    tot_neg: f64,
    cfs: Vec<PNPoint>,
}

impl Confusion {

    fn from_pnpoints(tot_pos: f64, tot_neg: f64, cfs: Vec<PNPoint>) -> Confusion {
        // TODO(hayesall): Find a better name, or make this the default for new struct initialization.
        Confusion {
            tot_pos,
            tot_neg,
            cfs,
        }
    }

    fn new(tot_pos: f64, tot_neg: f64) -> Confusion {

        // TODO(hayesall): This kind of makes sense for backwards compatability, but having fewer
        //      than 1 positive/negative examples seems like the problem should be undefined.
        //      How does other AUC software behave here?

        if tot_pos < 1.0 || tot_neg < 1.0 {
            return Confusion {
                tot_pos: 1.0,
                tot_neg: 1.0,
                cfs: Vec::new()
            }
        }

        Confusion {
            tot_pos,
            tot_neg,
            cfs: Vec::new()
        }
    }

    pub fn calculate_auc_pr(&self, min_recall: f64) -> f64 {
        if (min_recall < 0.0) || (min_recall > 1.0) {
            return 0.0;
        }

        // TODO(hayesall): A guard against empty vectors should probably be added elsewhere.

        let d1 = min_recall * self.tot_pos;
        let mut b = 0;

        let mut pnpoint1 = self.cfs[0];

        let mut pnpoint2 = self.cfs[0];

        while pnpoint1.pos < d1 {
            pnpoint2 = pnpoint1;
            b += 1;
            pnpoint1 = self.cfs[b];
        }

        let mut d2 = (pnpoint1.pos - d1) / self.tot_pos;
        let mut d3 = pnpoint1.pos / (pnpoint1.pos + pnpoint1.neg);
        let mut d4 = d2 * d3;

        if pnpoint2 != self.cfs[0] {
            let d5 = pnpoint1.pos / self.tot_pos - pnpoint2.pos / self.tot_pos;
            let d10 = pnpoint2.pos / (pnpoint2.pos + pnpoint2.neg);
            let d6 = d3 - d10;
            let d7 = d6 / d5;
            let d8 = d10 + d7 * (d1 - pnpoint2.pos) / self.tot_pos;
            let d9 = 0.5 * d2 * (d8 - d3);
            d4 += d9;
        }

        d2 = pnpoint1.pos / self.tot_pos;
        for i in b+1 .. self.cfs.len() {
            let pnpoint = self.cfs[i];
            let d5 = pnpoint.pos / self.tot_pos;
            let d6 = pnpoint.pos / (pnpoint.pos + pnpoint.neg);
            let d7 = (d5 - d2) * d6;
            let d8 = 0.5 * (d5 - d2) * (d3 - d6);
            d4 += d7 + d8;
            d2 = d5;
            d3 = d6;
        }

        return d4;
    }

    pub fn calculate_aucroc(&self) -> f64 {
        // TODO(hayesall): size() == 0 guard

        let pnpoint = self.cfs[0];
        let mut d1 = pnpoint.pos / self.tot_pos;
        let mut d2 = pnpoint.neg / self.tot_neg;
        let mut d3 = 0.5 * d1 * d2;

        for i in 1 .. self.cfs.len() {
            let pnpoint1 = self.cfs[i];
            let d4 = pnpoint1.pos / self.tot_pos;
            let d5 = pnpoint1.neg / self.tot_neg;
            let d6 = (d4 - d1) * d5;
            let d7 = 0.5 * (d4 - d1) * (d5 - d2);

            d3 += d6 - d7;
            d1 = d4;
            d2 = d5;
        }

        d3 = 1.0 - d3;
        return d3;
    }

    pub fn from_predictions(y_prob_pred: Vec<f64>, y_true: Vec<i64>) -> Confusion {
        if y_prob_pred.len() != y_true.len() {
            panic!("Cannot have unequal lengths here.")
        }

        let mut cls_array: Vec<ClassSort> = Vec::new();

        for i in 0..y_prob_pred.len() {
            cls_array.push(ClassSort::new(y_prob_pred[i], y_true[i]));
        }

        return classsort_to_confusion(cls_array);
    }

    fn add_point(&mut self, pnpoint: &PNPoint) {
        if !(self.cfs.contains(pnpoint)) {

            // TODO(hayesall): This feels like a weird check. It almost seems like `cfs`
            //      wants to be a set object that avoids duplicate points being added.

            // TODO(hayesall): It feels odd to clone here. This method could probably be
            //  simplified.
            self.cfs.push(pnpoint.clone());
        }
    }

    fn sort_copy(&self) -> Vec<PNPoint> {

        // TODO(hayesall): The only use of `self` is:
        //      1. Cloning the `cfs` vector
        //      2. checking the value of `tot_pos` and `tot_neg`
        //      ==> Possibly suggests that these should be functions, or tot_pos/tot_neg need
        //          to be set in a different way rather than "asking what they are?"

        // TODO(hayesall): This version of sort copies from the cfs vector.

        if self.cfs.len() == 0 {
            panic!("Tried sorting an empty vector. Something is probably wrong.");
        }

        let mut cfs = self.cfs.clone();
        cfs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut pnt1 = cfs[0];
        while pnt1.pos < 0.001 && pnt1.pos > -0.001 {
            cfs.remove(0);
            pnt1 = cfs[0];
        }

        let d = pnt1.neg / pnt1.pos;

        let pnt2 = PNPoint::new(1.0, d);
        if !(cfs.contains(&pnt2)) && pnt1.pos > 1.0 {
            cfs.insert(0, pnt2);
        }

        // This block performs the "add_point" functionality, but doesn't do a borrow on
        //  ConfusionVec data.
        let new_point = PNPoint::new(self.tot_pos, self.tot_neg);
        if !(cfs.contains(&new_point)) {
            cfs.push(new_point);
        }
        // end add_point

        return cfs;
    }
}

fn interpolate(pnts: Vec<PNPoint>) -> Vec<PNPoint> {

    if pnts.len() == 0 {
        panic!("Cannot interpolate 0 points, something is wrong.");
    }

    let mut out = pnts.clone();

    for b in 0 .. pnts.len() - 1 {
        let mut pnt1 = pnts[b];
        let pnt2 = pnts[b + 1];

        let d1 = pnt2.pos - pnt1.pos;
        let d2 = pnt2.neg - pnt1.neg;
        let d3 = d2 / d1;
        let d4 = pnt1.pos;
        let d5 = pnt2.neg;

        while (pnt1.pos - pnt2.pos).abs() > 1.001 {
            let d = d5 + (pnt1.pos - d4 + 1.0) * d3;
            let pnt = PNPoint::new(pnt1.pos + 1.0, d);
            out.insert(b + 1, pnt);
            pnt1 = pnt;
        }
    }

    return out;
}

fn classsort_to_confusion(mut class_sort: Vec<ClassSort>) -> Confusion {

    class_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut b1 = 0;
    let mut b2 = 0;

    if class_sort[class_sort.len() - 1].classification == 1 {
        b1 += 1;
    } else {
        b2 += 1;
    }

    let mut pnpoints: Vec<PNPoint> = Vec::new();
    let mut d = class_sort[class_sort.len() - 1].probability;

    for i in (0 .. class_sort.len() - 1).rev() {

        let probability = class_sort[i].probability;
        let classification = class_sort[i].classification;

        // There's a print statement like this in the original Java version that I cannot turn off.
        // It might be helpful during debugging though.
        // println!("{} {}", probability, classification);

        if probability != d {
            pnpoints.push(PNPoint::new(b1 as f64, b2 as f64));
        }
        d = probability;

        if classification == 1 {
            b1 += 1;
        } else {
            b2 += 1;
        }
    }

    pnpoints.push(PNPoint::new(b1 as f64, b2 as f64));

    let mut confusion = Confusion::new(b1 as f64, b2 as f64);

    for pnpoint in pnpoints.iter() {
        confusion.add_point(pnpoint);
    }

    let new_vec = confusion.sort_copy();
    let interpolated = interpolate(new_vec);

    let confusion2 = Confusion::from_pnpoints(b1 as f64, b2 as f64, interpolated);

    return confusion2;
}


#[cfg(test)]
mod confusion_tests {

    extern crate float_cmp;
    use float_cmp::approx_eq;

    use crate::Confusion;

    #[test]
    fn test_confusion_from_predictions_1() {
        let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
        let ytrue = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];
        let confusion = Confusion::from_predictions(ypred, ytrue);

        assert!(approx_eq!(f64, confusion.calculate_auc_pr(0.0), 0.8243055555555555, epsilon = 0.0001));
        assert!(approx_eq!(f64, confusion.calculate_aucroc(), 0.75, epsilon = 0.0001));
    }

    #[test]
    fn test_confusion_from_predictions_2() {
        let ypred = vec![0.75, 0.85, 0.6, 0.4, 0.0001];
        let ytest = vec![0, 0, 0, 0, 1];
        let confusion = Confusion::from_predictions(ypred, ytest);

        assert!(approx_eq!(f64, confusion.calculate_auc_pr(0.0), 0.2, epsilon = 0.0001));
        assert!(approx_eq!(f64, confusion.calculate_aucroc(), 0.5, epsilon = 0.0001));
    }

    #[test]
    fn test_confusion_from_predictions_3() {
        let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
        let ytest = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];
        let confusion = Confusion::from_predictions(ypred, ytest);

        assert!(approx_eq!(f64, confusion.calculate_auc_pr(0.5), 0.3729166666666667, epsilon = 0.0001));
        assert!(approx_eq!(f64, confusion.calculate_aucroc(), 0.75, epsilon = 0.0001));
    }

    #[test]
    fn test_confusion_from_predictions_4() {
        let ypred = vec![0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505];
        let ytest = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 0];
        let confusion = Confusion::from_predictions(ypred, ytest);

        assert!(approx_eq!(f64, confusion.calculate_auc_pr(1.0), 0.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confusion.calculate_aucroc(), 0.75, epsilon = 0.0001));
    }
}
