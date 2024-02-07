use crate::{Currency, Precision};

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with verified decimal precision information
pub struct Precise<T> {
    pub(crate) currency: T,
    pub(crate) decimals: u8,
}
impl<T: Currency> Precision for Precise<T> {
    type C = T;

    fn currency(&self) -> &T {
        &self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with no decimal precision information
pub struct Imprecise<T: Currency> {
    pub(crate) currency: T,
}
impl<T: Currency> Precision for Imprecise<T> {
    type C = T;
    fn currency(&self) -> &T {
        &self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A currency with unverified decimal precision information
pub struct Unverified<T: Currency> {
    pub(crate) currency: T,
    pub(crate) decimals: u8,
}
impl<T: Currency> Precision for Unverified<T> {
    type C = T;
    fn currency(&self) -> &T {
        &self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PrecisionType<T: Currency> {
    Precise(Precise<T>),
    Imprecise(Imprecise<T>),
    Unverified(Unverified<T>),
}

impl<T: Currency> PrecisionType<T> {
    pub fn currency(&self) -> &T {
        match self {
            PrecisionType::Precise(p) => p.currency(),
            PrecisionType::Imprecise(p) => p.currency(),
            PrecisionType::Unverified(p) => p.currency(),
        }
    }
}
