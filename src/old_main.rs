enum Interval {
    Daily,
    Monthly,
    Quarterly,
    Yearly,
}

struct Returns {
    interval: Interval,
    returns: Vec<f64>
}

impl Returns {
    pub fn new(interval: Interval, returns: Vec<f64>) -> Self {
        Self {
            interval,
            returns,
        }
    }
}

// has return, risk, and covariance with other assets
// this can all be calculated from the historical returns
struct Asset {
    name: String,
    returns: Returns,
}

impl Asset {
    fn new(name: String, interval: Interval, returns: Vec<f64>) -> Self {
        Self {
            name,
            interval,
            returns,
        }
    }

    pub fn mean_return(&self) -> f64 {
        self.returns.iter().sum::<f64>() / self.returns.len() as f64
    } 

    pub fn variance(&self) -> f64 {
        let mean = self.mean_return();
        self.returns.iter()
            .map(|&r| (r - mean).powi(2))
            .sum::<f64>() / self.returns.len() as f64
    }

    pub fn standard_deviation(&self) -> f64 {
        let variance = self.variance();
        variance.sqrt()
    }

}

struct Weight {
    quantize: f32,
    weight: f32,
}

impl Weight {
    fn from(weight: f32) -> Self {
        // quantize weight before returning
        Weight { quantize: 0.1, weight: weight }
    }
}

//struct AssetDistribution {
//    // reference to asset list?
//    weights
//}

struct Portfolio {
    asset_distribution: Vec<f32>,
}

trait SolverBuilder {
    fn add_asset(asset: Asset);
    fn get_portfolio() -> Portfolio;
}


impl Portfolio {
    pub fn new(assets: Vec<Asset>) -> Self {
        Self {
            asset_distribution,
        }
    }
    pub fn expected_return(&self) -> f64 {
        self.asset_distribution.iter().zip(self.asset_distribution.iter())
            .map(|(asset, &weight)| asset.mean_return() * weight)
            .sum()
    }

}

struct AssetList {
    assets: Vec<Asset>
}

// A solver needs to allocate at least 100% of the available capital as cash could be seen as an
// asset, just with low risk and negative returns.
//
// 
trait Solver {
    fn new(asset_list: Vec<Asset>) -> Self;
    //fn 
}

struct NaiveSolver {
    //leverage: f32,
    asset_list: AssetList,
    //step_size: u8, // for search
}

impl Solver for NaiveSolver {
    fn new(portfolio: Portfolio) -> Self {
        Self {
            portfolio,
        }
    }
}

fn main() {
    let amzn = Asset::new("AMZN".to_string(), vec![2.0, -1.0, 3.0]);
    let solver = NaiveSolver::new(vec![amzn]);

    let portfolios = solver.get_efficient_portfolios();

    //let portfolio = Portfolio::new(vec![asset1]);

}
