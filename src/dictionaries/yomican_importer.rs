use crate::dictionaries::importer::{DictImporter, ImportError};
use serde_json::{Value};

pub struct YomichanImporter {}

impl DictImporter for YomichanImporter {
  fn import(&self, file: std::fs::File) -> Result<(), String> { 
    let yomi_terms: Value = serde_json::from_reader(file).unwrap();

    for yomi_term in yomi_terms.as_array().unwrap().iter() {
      println!("{}", yomi_term[0]);
      println!("{}", yomi_term[1])
    }
    Ok(())
  }
}