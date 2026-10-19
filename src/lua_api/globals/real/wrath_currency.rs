//! Supplied one-level Wrath currency state; no modern catalog inference.
#[cfg(feature = "client-wrath")]
mod api;
#[cfg(feature = "client-wrath")]
pub(crate) use api::register;

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

#[cfg(feature = "client-wrath")]
impl CurrencyListState {
    fn displayed_rows(&self) -> impl Iterator<Item = (usize, Option<usize>)> + '_ {
        self.categories
            .iter()
            .enumerate()
            .flat_map(|(category, data)| {
                let shown = if data.expanded {
                    data.currencies.len()
                } else {
                    0
                };
                std::iter::once((category, None))
                    .chain((0..shown).map(move |token| (category, Some(token))))
            })
    }

    fn displayed_row(&self, index: i32) -> Option<(usize, Option<usize>)> {
        let offset = usize::try_from(index.checked_sub(1)?).ok()?;
        self.displayed_rows().nth(offset)
    }
}
