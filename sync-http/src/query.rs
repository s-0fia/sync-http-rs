pub struct Query {
    pub queries: Vec<KeyValue>,
}

#[derive(Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Query {
    pub fn parse_all(queries: String) -> Self {
        todo!()
    }

    pub fn parse(query: String) -> Option<Self> {
        todo!()
    }
}

impl Default for Query {
    fn default() -> Self {
        Self { queries: vec![] }
    }
}
