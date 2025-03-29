pub struct ColumnSpec {
    pub name: String,
    pub data_type: String,
    pub width: usize,
}

impl ColumnSpec {
    fn parse_individual(spec: &str) -> ColumnSpec {
        let mut parts = spec.split(":");

        ColumnSpec {
            name: parts.next().unwrap().to_string(),
            data_type: parts.next().unwrap().to_string(),
            width: parts.next().unwrap().parse::<usize>().unwrap(), //TODO: handle the '*' case, which means open-ended
        }
    }

    pub fn parse(spec: &str) -> Vec<ColumnSpec> {
        spec.split(",")
            .map(|x| ColumnSpec::parse_individual(x))
            .collect()
    }

    pub fn format(&self, input: &str) -> String {
        match self.data_type.as_str() {
            "s" => format!("\"{}\"", input.to_string()),
            _ => input.to_string(),
        }
    }
}
