//! 仿真模块 - 旧模块入口（兼容旧路径，内容已拆分为独立模块

pub mod casting;
pub mod acoustic;

pub use casting::simulate_casting;
pub use acoustic::simulate_acoustic;

pub use crate::alloy_analyzer::*;
pub use crate::process_comparator::*;
pub use crate::tower_acoustics::*;
pub use crate::vr_bell_strike::*;

#[deprecated(since = "1.1.0", note = "Use alloy_analyzer 模块")]
pub mod alloy_comparison {
    pub use crate::alloy_analyzer::*;
}

#[deprecated(since = "1.1.0", note = "Use process_comparator 模块")]
pub mod casting_comparison {
    pub use crate::process_comparator::*;
}

#[deprecated(since = "1.1.0", note = "Use tower_acoustics 模块")]
pub mod tower_acoustic {
    pub use crate::tower_acoustics::*;
}

#[deprecated(since = "1.1.0", note = "Use vr_bell_strike 模块")]
pub mod virtual_strike {
    pub use crate::vr_bell_strike::*;
}
