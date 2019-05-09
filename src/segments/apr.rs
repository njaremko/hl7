use crate::pub_struct;
pub_struct!(APR {
    apr_1_time_selection_criteria: Option<Vec<String>>,
    apr_2_resource_selection_criteria: Option<Vec<String>>,
    apr_3_location_selection_criteria: Option<Vec<String>>,
    apr_4_slot_spacing_criteria: Option<String>,
    apr_5_filler_override_criteria: Option<Vec<String>>,
});
