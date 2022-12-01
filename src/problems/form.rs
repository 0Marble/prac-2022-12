use std::collections::HashMap;

pub struct Form {
    fields: HashMap<String, String>,
    field_names: Vec<String>,
}

impl Form {
    pub fn new(field_names: Vec<String>) -> Self {
        Self {
            fields: HashMap::from_iter(field_names.iter().cloned().map(|n| (n, "".to_string()))),
            field_names,
        }
    }

    pub fn set(&mut self, name: &str, val: String) {
        if let Some(cur_val) = self.fields.get_mut(name) {
            *cur_val = val;
        }
    }

    pub fn get_fields(&self) -> impl Iterator<Item = (&str, &str)> {
        self.field_names
            .iter()
            .map(|n| self.fields.get_key_value(n).unwrap())
            .map(|(name, val)| (name.as_str(), val.as_str()))
    }

    pub fn add_field(&mut self, name: &str) {
        if !self.fields.contains_key(name) {
            self.field_names.push(name.to_string());
            self.fields.insert(name.to_string(), "".to_string());
        }
    }
}
