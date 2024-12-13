use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Algo {
  pub r#type: AlgoType,
}

#[derive(Debug, Clone, Copy)]
pub enum AlgoType {
  Blake3,
  Default,
}

impl Algo {
  pub fn new(r#type: AlgoType) -> Self {
    Algo { r#type }
  }

  pub fn hash(&self, v: String) -> String {
    match self.r#type {
      AlgoType::Blake3 => {
        let hash = blake3::hash(v.as_bytes());
        hash.to_string()
      }
      AlgoType::Default => {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish().to_string()
      }
    }
  }

  pub fn get_name(&self) -> &str {
    match self.r#type {
      AlgoType::Blake3 => "blake3",
      AlgoType::Default => "default",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hash_should_work() {
    let algo = Algo::new(AlgoType::Blake3);
    assert_eq!(
      algo.hash("hello".to_string()),
      "c4e6e9e7e7e7e7e7e7e7e7e7e7e7e7e7"
    );

    let algo = Algo::new(AlgoType::Default);
    assert_eq!(
      algo.hash("hello".to_string()),
      "5d41402abc4b2a76b9719d911017c592"
    );
  }
}
