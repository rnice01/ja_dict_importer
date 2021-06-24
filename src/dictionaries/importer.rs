use std::fmt;
pub struct ImportError;

impl fmt::Display for ImportError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error")
  }
}

impl fmt::Debug for ImportError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ file {}, line: {} }}", file!(), line!())
  }
}

pub struct Term {
  expression: String,
  reading: String,
  meanings: Vec<String>,
  parts_of_speech: Vec<String>
}

pub trait DictImporter {
  fn import(&self, f : std::fs::File) -> Result<(), String>;
}