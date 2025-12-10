use async_graphql::*;
use finance_query_core::models::movers::MarketMover as MarketMoverModel;

#[derive(SimpleObject, Clone)]
pub struct MarketMover {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub change: String,
    #[graphql(name = "percentChange")]
    pub percent_change: String,
}

impl From<MarketMoverModel> for MarketMover {
    fn from(mover: MarketMoverModel) -> Self {
        MarketMover {
            symbol: mover.symbol,
            name: mover.name,
            price: mover.price,
            change: mover.change,
            percent_change: mover.percent_change,
        }
    }
}
