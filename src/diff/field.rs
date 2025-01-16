use crate::diff::{Diff, DiffResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    Binary,
    Unprotected,
    Protected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub kind: ValueType,
    pub use_verbose: bool,
    pub mask_passwords: bool,
}

impl Diff for Field {
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self> {
        // Windows inserts different newline chars in notes than Linux and MacOS. Ignore those to
        // focus on real differences
        let own_normalized_value = self.value.replace("\r\n", "\n");
        let other_normalized_value = other.value.replace("\r\n", "\n");
        if own_normalized_value == other_normalized_value {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        } else {
            DiffResult::Changed {
                left: self,
                right: other,
            }
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.use_verbose {
            write!(
                f,
                "Field '{}' = '{}'",
                self.name,
                match (self.mask_passwords, self.kind) {
                    (true, ValueType::Protected) => "***".to_owned(),
                    _ => self.value.to_owned(),
                }
            )
        } else {
            write!(
                f,
                "{} = {}",
                self.name,
                match (self.mask_passwords, self.kind) {
                    (true, ValueType::Protected) => "***".to_owned(),
                    _ => self.value.to_owned(),
                }
            )
        }
    }
}
