pub struct ColumnSpec {
    pub name: String,
    pub data_type: String,
    pub width: usize,
}

impl ColumnSpec {
    fn from_spec_str(spec: &str) -> ColumnSpec {
        let mut parts = spec.split(":");

        ColumnSpec {
            name: parts.next().unwrap().to_string(),
            data_type: parts.next().unwrap().to_string(),
            width: parts.next().unwrap().parse::<usize>().unwrap(), //TODO: handle the '*' case, which means open-ended
        }
    }

    pub fn from_specs_str(spec: &str) -> Vec<ColumnSpec> {
        spec.split(",")
            .map(|x| ColumnSpec::from_spec_str(x))
            .collect()
    }

    pub fn parse(&self, input: &str) -> String {
        match self.data_type.as_str() {
            "str" => format!("\"{}\"", input.to_string()),
            _ => input.to_string(),
        }
    }
}
