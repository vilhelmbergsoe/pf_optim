combinatorically, if you had a list of assets:

asset 1: (risk1, return1) asset 2: (risk2, return2) asset 3: (risk3, return3)

all the possible combinations can be represented by a percentage allocated to
all assets:

portfolio:

(50%, 0%, 50%)

technically there are infinitely many portfolios to construct from just one
asset, but in reality decimal changes below a certain amount, amount to no real
difference in the portfolios performance and therefore should not be counted.

if we assume that only the real rational continuous number would be quantized
to what would essentially amount to whole discrete numbers 0-100, representing
the percentage of bankroll allocated to the specific asset, the problem become
tractable.

assuming only whole percentages can be allocated, we get 100 x 100 x 100
possible solutions, which amounts to 1\_000\_000 possible portfolios
constructed from three assets.

the amount of portfolios (Pn) for n assets, where σ is the quantized possible
percentage allocated, is therefore: Pn = σ^n.

# questions:

- how does leverage come into play

(200%, 90%, 10%)

- how do different allocations of the same assets make a difference in the
  whole risk / return problem, mathematically.


## MODEL

Assets:

- measured over a period
- performance (return)
- standard deviation (volatility/risk)

- covariance:
covariance describes how an asset moves with another asset

    - for covariance to be calculated for all asset combinations, an individual
      asset would be better described using it's historical returns as the
      correlation coefficient needs to be calculated between all assets and
      storing each other assets correlation coefficient under one asset is
      stupid.

Portfolio:

- collection of assets
- has an overall expected portfolio return (derived from assets)
- has an overall expected risk (derived from assets):
    - risk can measured:
        - naively (zero covariance assumption)
        - smartly (covariance assumption)

- covariance term in risk of portfolio provides a useful indication of hedging
  or over-allocation in one asset class

    - portfolio full of correlated assets means that any return, will most
      likely be seen amongst all assets (therefore higher volatility and higher
      returns)

    - portfolio full of uncorrelated assets means that any return (negative or
      positive), will likely be a hedge toward some other asset in the
      portfolio if the two assets are uncorrelated (P(A,B) = -1) meaning (lower
      overall risk and return)

Optimization:

- is all about finding a portfolio with an efficient allocation of capital
  amongst n assets
- here a pareto front is established between risk and expected return
- the lower the risk and the higher the expected returns the better the
  portfolio
- intractability:
    - finding all possible efficient portfolios make up an intractable
      calculation
    - efficient optimization algorithms are needed and are possible as it might
      be obvious from certain test allocations that a combination of different
      assets might need less of a (bad asset = high risk, low return) in order
      to get higher returns and lower risk.



