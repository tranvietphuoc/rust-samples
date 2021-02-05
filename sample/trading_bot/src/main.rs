pub struct TradingBot {
    pub trading_config: TradingConfig,
    pub market: Box<dyn Market>,
}

pub trait Market {
    async fn get_balance(&self) -> Result<f32, Box<dyn Error>>;
    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>>;
    async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
    async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
}

impl TradingBot {
    // main trading logic
    // high sell, low buy
    //

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let current_price = self.get_market_price().await?;
        info!("[PRICE] current market price: {:?} $", current_price);

        let percentage_diff = (current_price - self.trading_config.last_operation_price)
    }
}

// get the buy point
// buy action

async fn try_to_buy(&mut self, diff: f32) -> Result<f32, Box<dyn Error>> {
    if diff >= self.trading_config.upward_trend_threshold || diff <= self.trading_config.dip_threshold {
        let current_balance = self.get_balance().await?;
        info!("[BALANCE] current amount balance {:?} $ USD", current_balance);
        self.trading_config.last_operation_price = self.place_buy_order(current_balance).await?;
        self.trading_config.next_operation = State::SELL;
        info!("[BUY] Bought 0.002 BTC for {:?} $ USD", self.trading_config.last_operation_price);
    }
    Ok(self.trading_config.last_operation_price)
}
