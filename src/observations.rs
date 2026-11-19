use std::{fmt::Display, fs::File, io::BufRead, path::Path};

use crate::{error::Error, tables::one_sided_value};

use rand::prelude::*;

#[derive(Clone)]
pub struct Observations {
    /// A list of observations.
    xs: Vec<f64>,
    /// The population mean. For example, if `self.xs` is a list of differences between program A
    /// and program B, the population mean would be 0, as the null hypothesis is that there is a
    /// speed-up of 0 time units.
    population_mean: f64,
}
impl Observations {
    pub fn new(mut xs: Vec<f64>, population_mean: f64) -> Self {
        xs.sort_unstable_by(f64::total_cmp);
        Self { xs, population_mean }
    }
    pub fn min(&self) -> f64 {
        self.xs[0]
    }
    pub fn p25(&self) -> f64 {
        self.xs[self.xs.len() / 4]
    }
    pub fn median(&self) -> f64 {
        self.xs[self.xs.len() / 2]
    }
    pub fn p75(&self) -> f64 {
        self.xs[(self.xs.len() / 4) * 3]
    }
    pub fn max(&self) -> f64 {
        self.xs[self.xs.len() - 1]
    }
    pub fn mean(&self) -> f64 {
        let n = f64::from(u32::try_from(self.xs.len()).unwrap());
        self.xs.iter().sum::<f64>() / n
    }
    pub fn std_deviation(&self) -> f64 {
        let mean = self.mean();
        let degrees_of_freedom = f64::from(u32::try_from(self.xs.len() - 1).unwrap());
        let sum_of_diffs = self
            .xs
            .iter()
            .map(|x| {
                let diff_from_mean = *x - mean;
                diff_from_mean * diff_from_mean
            })
            .sum::<f64>();
        (sum_of_diffs / degrees_of_freedom).sqrt()
    }
    /// Bootstrapped confidence interval and the bootstrap estimator. Choose a confidence level
    /// `alpha`, a sample size `n`, and a number `b` of bootstrapped samples.
    ///
    /// <div class="warning">It is assumed that <code>n <= self.xs.len()</code>!</div>
    pub fn bootstrapped_ci(
        &self,
        alpha: f64,
        n: usize,
        b: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> ([f64; 2], f64) {
        let mut sample_means = Vec::with_capacity(b);
        for _ in 0..b {
            let mut sample = Vec::new();
            for _ in 0..n {
                sample.push(self.xs.iter().choose(rng).unwrap());
            }
            sample_means
                .push(sample.into_iter().sum::<f64>() / f64::from(u32::try_from(n).unwrap()))
        }
        sample_means.sort_unstable_by(f64::total_cmp);
        let confidence_tail = (1.0 - alpha) / 2.0;
        let b_f64 = f64::from(u32::try_from(b - 1).unwrap());
        let ci = [
            sample_means[(confidence_tail * b_f64).floor() as usize],
            sample_means[((1.0 - confidence_tail) * b_f64).floor() as usize],
        ];
        let bootstrap_estimator = (0..b)
            .into_iter()
            .map(|j| {
                let difference = sample_means[j] - self.mean();
                difference * difference
            })
            .sum();
        (ci, bootstrap_estimator)
    }
    pub fn t(&self) -> f64 {
        let n = f64::from(u32::try_from(self.xs.len()).unwrap());
        (self.mean() - self.population_mean) / (self.std_deviation() / n.sqrt())
    }
    /// Is p < 0.05?
    pub fn p(&self) -> bool {
        let t = self.t();
        let degrees_of_freedom = self.xs.len() - 1;
        let table_value = one_sided_value(degrees_of_freedom);
        t > table_value
    }
    pub fn to_json(&self) -> String {
        let mut rng = rand::rng();
        let n = self.xs.len();
        let min = self.min();
        let p25 = self.p25();
        let median = self.median();
        let p75 = self.p75();
        let max = self.max();
        let mean = self.mean();
        let std_deviation = self.std_deviation();
        let sample_n = self.xs.len();
        let ci = self.bootstrapped_ci(0.95, sample_n, 1000, &mut rng).0;
        let ci_lower = ci[0];
        let ci_upper = ci[1];
        let t = self.t();
        let p = self.p();
        format!(
            r#"
    "summary": {{
        "N": {n},
        "min": {min},
        "p25": {p25},
        "median": {median},
        "p75": {p75},
        "max": {max},
        "mean": {mean},
        "std_deviation": {std_deviation},
        "ci": [{ci_lower}, {ci_upper}],
        "t": {t},
        "is_p_less_than_5": {p}
    }}"#
        )
    }
}

impl Display for Observations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min,p25,median,p75,max,x̄,σ²,t,p<0.05?
{}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.min(),
            self.p25(),
            self.median(),
            self.p75(),
            self.max(),
            self.mean(),
            self.std_deviation(),
            self.t(),
            self.p(),
        )
    }
}

pub fn parse(path: &Path) -> Result<Vec<f64>, Error> {
    let file = File::open(path)?;
    std::io::BufReader::new(file)
        .lines()
        .map(|res| res.map_err(|e| -> Error { e.into() }))
        .filter(|s| if let Ok(s) = &s { !s.is_empty() } else { true })
        .map(|res| {
            res.and_then(|s| s.parse().map_err(Into::into))
        })
        .collect()
}
