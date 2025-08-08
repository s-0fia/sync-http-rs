#[derive(Default, Debug, PartialEq)]
pub struct Query {
    pub queries: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Query {
    pub fn parse(queries: String) -> Self {
        let queries = queries.trim().trim_matches('?').to_string();
        Self {
            queries: queries
                .split('+')
                .map(str::to_string)
                .filter_map(KeyValue::parse)
                .collect(),
        }
    }
}

impl KeyValue {
    pub fn parse(query: String) -> Option<Self> {
        let parts: Vec<&str> = query.split('=').collect();
        if parts.len() > 2 || parts.is_empty() {
            None?;
        }

        Some(KeyValue {
            key: parts[0].to_string(),
            value: match parts.get(1) {
                Some(value) => value.to_string(),
                None => String::new(),
            },
        })
    }
}
