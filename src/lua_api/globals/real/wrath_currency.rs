//! Supplied one-level Wrath currency fixture; no modern catalog inference.
#[derive(Clone, Debug, Default)]
pub struct CurrencyToken {
    pub name: String,
    pub count: i32,
    pub extra_currency_type: i32,
    pub icon: String,
    pub item_id: Option<i32>,
    pub watched: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CurrencyCategory {
    pub name: String,
    pub expanded: bool,
    pub currencies: Vec<CurrencyToken>,
}

#[derive(Default)]
pub struct CurrencyListState {
    pub categories: Vec<CurrencyCategory>,
}
