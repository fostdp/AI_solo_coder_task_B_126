pub mod casting;
pub mod acoustic;
pub mod alloy_comparison;
pub mod casting_comparison;
pub mod tower_acoustic;
pub mod virtual_strike;

pub use casting::simulate_casting;
pub use acoustic::simulate_acoustic;
pub use alloy_comparison::{compare_alloys, get_alloy_composition_suggestion};
pub use casting_comparison::{compare_casting_methods, get_casting_method_key_list, get_recommended_method_for_bell};
pub use tower_acoustic::{simulate_tower_acoustics, get_preset_tower_configs};
pub use virtual_strike::{compute_strike_impact, get_position_options, get_mallet_options, generate_strike_tutorial};
