use std::collections::HashMap;

use crate::domain::Role;

#[derive(Debug, Clone, PartialEq)]
struct RoleWeights {
  w_role: HashMap<Role, f64>,
}

// バランスが悪い場合調整する。
// 各レーンの重み付け
impl Default for RoleWeights {
  fn default() -> Self {
    let mut w_role = HashMap::new();

    w_role.insert(Role::Top, 1.0);
    w_role.insert(Role::Jg, 1.1);
    w_role.insert(Role::Mid, 1.08);
    w_role.insert(Role::Adc, 1.04);
    w_role.insert(Role::Sup, 0.96);

    Self {
      w_role,
    }
  }
}