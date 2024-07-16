#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    EUR,
    USD,
    JPY,
    GBP,
    AUD,
    CAD,
    CHF,
    CNH,
    HKD,
    NZD,
    Other([char; 3])
}