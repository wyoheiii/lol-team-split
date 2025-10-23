#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
  Top,
  Jg,
  Mid,
  Adc,
  Sup,
}

impl Role {
  const All: [Role; 5] = [Role::Top, Role::Jg, Role::Mid, Role::Adc, Role::Sup];

  fn name(&self) -> &'static str {
    match self {
      Role::Top => "Top",
      Role::Jg => "Jg",
      Role::Mid => "Mid",
      Role::Adc => "Adc",
      Role::Sup => "Sup",
    }
  }
}
