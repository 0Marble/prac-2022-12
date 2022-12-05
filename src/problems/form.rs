use std::{collections::HashMap, slice::Iter};

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

    pub fn get(&self, name: &str) -> Option<&String> {
        self.fields.get(name)
    }

    pub fn get_fields(&self) -> FieldsIter {
        FieldsIter {
            field_names: self.field_names.iter(),
            fields: &self.fields,
        }
    }

    pub fn add_field(&mut self, name: String) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.fields.entry(name.clone()) {
            self.field_names.push(name);
            e.insert("".to_string());
        }
    }
}

pub struct FieldsIter<'a> {
    field_names: Iter<'a, String>,
    fields: &'a HashMap<String, String>,
}

impl<'a> Iterator for FieldsIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.field_names
            .next()
            .map(|name| self.fields.get_key_value(name).unwrap())
            .map(|(name, val)| (name.as_str(), val.as_str()))
    }
}
