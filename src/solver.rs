use crate::{Portfolio, AssetList};

pub trait PortfolioEvaluator {
    fn evaluate(&self, portfolio: &Portfolio) -> f64;
}

pub struct MeanVarianceEvaluator {
    pub risk_aversion: f64, // User-defined risk tolerance level
}

impl PortfolioEvaluator for MeanVarianceEvaluator {
    fn evaluate(&self, portfolio: &Portfolio) -> f64 {
        let mean = portfolio.expected_return();
        let variance = portfolio.variance();
        mean - self.risk_aversion * variance
    }
} 

pub struct SharpeRatioEvaluator {
    pub risk_free_rate: f64,
}

impl PortfolioEvaluator for SharpeRatioEvaluator {
    fn evaluate(&self, portfolio: &Portfolio) -> f64 {
        let mean_return = portfolio.mean_return();
        let portfolio_std = portfolio.std_deviation(); // Assume we have a std_deviation method
        (mean_return - self.risk_free_rate) / portfolio_std
    }
}

pub trait Optimizer {
    fn get_efficient_portfolios(&self) -> Vec<Portfolio>;
}

// PortfolioOptimizer with step_size for quantized search space
pub struct PortfolioOptimizer {
    step_size: f64,
    asset_list: AssetList,
}

impl PortfolioOptimizer {
    pub fn new(asset_list: AssetList, step_size: f64) -> Self {
        Self { asset_list, step_size }
    }

    // Generate all possible weight combinations based on step_size
    fn generate_weight_combinations(&self) -> Vec<Vec<f64>> {
        let num_assets = self.asset_list.assets.len();
        let mut combinations = Vec::new();
        let step = self.step_size as usize;

        // Generate weight combinations
        let mut weights = vec![0.0; num_assets];
        self.generate_combinations_recursive(&self.asset_list, &mut combinations, &mut weights, 0, step);

        combinations
    }

    fn generate_combinations_recursive(
        &self,
        asset_list: &AssetList,
        combinations: &mut Vec<Vec<f64>>,
        weights: &mut Vec<f64>,
        index: usize,
        step: usize
    ) {
        if index == asset_list.assets.len() {
            // Ensure the total weight is 1.0
            if weights.iter().sum::<f64>() > 0.999 && weights.iter().sum::<f64>() < 1.001 {
                combinations.push(weights.clone());
            }
            return;
        }

        // Recursively generate combinations
        for i in 0..=step {
            weights[index] = (i as f64) * self.step_size as f64;
            self.generate_combinations_recursive(asset_list, combinations, weights, index + 1, step);
        }
    }
}

// Implement the trait for PortfolioOptimizer
impl MPTOptim for PortfolioOptimizer {
    fn get_efficient_portfolios(&self) -> Vec<Portfolio> {
        let weight_combinations = self.generate_weight_combinations();
        weight_combinations.into_iter().map(|weights| {
            Portfolio::new(&self.asset_list, weights)
        }).collect()
    }
}
