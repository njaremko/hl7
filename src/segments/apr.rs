use crate::pub_struct;
pub_struct!(APR {
    apr1_time_selection_criteria: Option<Vec<String>>,
    apr2_resource_selection_criteria: Option<Vec<String>>,
    apr3_location_selection_criteria: Option<Vec<String>>,
    apr4_slot_spacing_criteria: Option<String>,
    apr5_filler_override_criteria: Option<Vec<String>>,
});
