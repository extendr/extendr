#[derive(Debug, PartialEq, Clone)]
pub struct Promise<C, E, V> {
    pub code: C,
    pub env: E,
    pub value: V,
    pub seen: bool,
}
