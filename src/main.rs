mod solver;
use crate::solver::{MPTOptim, PortfolioOptimizer};

#[derive(Debug, Clone, Copy)]
enum Interval {
    //Daily,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug)]
struct Returns {
    interval: Interval,
    returns: Vec<f64>,
}

impl Returns {
    pub fn new(interval: Interval, returns: Vec<f64>) -> Self {
        Self { interval, returns }
    }

    pub fn len(&self) -> usize {
        self.returns.len()
    }
}

impl Iterator for Returns {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.returns.pop()
    }
}

#[derive(Debug)]
struct Asset {
    name: String,
    returns: Returns,
}

impl Asset {
    fn new(name: String, interval: Interval, returns: Vec<f64>) -> Self {
        Self {
            name,
            returns: Returns::new(interval, returns),
        }
    }

    pub fn mean_return(&self) -> f64 {
        let sum: f64 = self.returns.returns.iter().sum();
        sum / self.returns.returns.len() as f64
    }

    pub fn variance(&self) -> f64 {
        let mean = self.mean_return();
        self.returns
            .returns
            .iter()
            .map(|&r| (r - mean).powi(2))
            .sum::<f64>()
            / self.returns.returns.len() as f64
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}

#[derive(Debug)]
struct AssetList {
    assets: Vec<Asset>,
}

impl AssetList {
    pub fn new(assets: Vec<Asset>) -> Self {
        Self { assets }
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn get(&self, index: usize) -> Option<&Asset> {
        self.assets.get(index)
    }

    pub fn builder() -> AssetListBuilder {
        AssetListBuilder::new()
    }
}

struct AssetListBuilder {
    assets: Vec<Asset>,
}

impl AssetListBuilder {
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }

    pub fn add_asset(mut self, asset: Asset) -> Self {
        self.assets.push(asset);
        self
    }

    pub fn build(self) -> AssetList {
        AssetList::new(self.assets)
    }
}

#[derive(Debug)]
struct Portfolio<'a> {
    asset_list: &'a AssetList,
    weights: Vec<f64>, // TODO: BETTER WEIGHTS 
}

impl<'a> Portfolio<'a> {
    pub fn new(asset_list: &'a AssetList, weights: Vec<f64>) -> Self {
        assert_eq!(asset_list.assets.len(), weights.len(), "Assets and weights must have the same length.");
        Self { asset_list, weights }
    }

    pub fn expected_return(&self) -> f64 {
        self.asset_list.assets.iter().zip(&self.weights)
            .map(|(asset, &weight)| asset.mean_return() * weight)
            .sum()
    }

    pub fn variance(&self) -> f64 {
        let mut variance = 0.0;
        for i in 0..self.asset_list.assets.len() {
            for j in 0..self.asset_list.assets.len() {
                let asset_i = &self.asset_list.assets[i];
                let asset_j = &self.asset_list.assets[j];
                variance += self.weights[i] * self.weights[j] * covariance(asset_i, asset_j);
            }
        }
        variance
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}

fn covariance(asset_a: &Asset, asset_b: &Asset) -> f64 {
    let mean_a = asset_a.mean_return();
    let mean_b = asset_b.mean_return();
    let n = asset_a.returns.len().min(asset_b.returns.len()) as f64;

    asset_a.returns.returns.iter()
        .zip(asset_b.returns.returns.iter())
        .map(|(&r1, &r2)| (r1 - mean_a) * (r2 - mean_b))
        .sum::<f64>() / (n - 1.0)
}


fn main() {
    let amzn = Asset::new("AMZN".to_string(), Interval::Monthly, vec![0.02, -0.01, 0.03, 0.04]);
    let aapl = Asset::new("AAPL".to_string(), Interval::Monthly, vec![0.03, -0.02, 0.05, 0.01]);

    let asset_list = AssetList::builder()
        .add_asset(amzn) // add asset interval checks
        .add_asset(aapl)
        .build();

    let solver = PortfolioOptimizer::new(asset_list, 0.1);

    println!("{:?}", solver.get_efficient_portfolios());

    //let portfolio = Portfolio::new(&asset_list, vec![0.8, 0.2]);
    //
    //println!("Expected Return: {:.2}%", portfolio.expected_return() * 100.0);
    //println!("Portfolio Standard Deviation: {:.2}%", portfolio.standard_deviation() * 100.0);
}

