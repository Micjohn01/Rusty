struct UnivariatePoly {
    coefficients: Vec<f64>,
}

impl UnivariatePoly {
    fn degree(&self) -> usize {
        self.coefficients.len()
    }

    fn evaluate(&self, x: f64) -> f64 {
        // let mut evaluation = 0.0;
        // for i in 0..self.coefficients.len() {
        //    evaluation += self.coefficients[i] * x.powf(i as f64)
        // }
        // evaluation
        let mut evaluation = 0.0;
        let mut current_x = 1.0;
        for i in 0..self.coefficients.len() {
           evaluation += self.coefficients[i] * current_x;
            current_x *= x;
        }
        evaluation

        self.coefficients.iter().enumerate().map(|(i, coffe)| coffe * x.powf(i as f64)).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::UnivariatePoly;
    #[test]
    fn test_degree() {
        let p = UnivariatePoly { coefficients: vec![1.0, 2.0, 3.0] };
        assert_eq!(p.degree(), 2);
    }
}