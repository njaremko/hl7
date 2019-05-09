use crate::pub_struct;
pub_struct!(AIG {
    aig_1_set_id: String,
    aig_2_segment_action_code: Option<String>,
    aig_3_resource_id: Option<String>,
    aig_4_resource_type: Option<String>,
    aig_5_resource_group: Option<Vec<String>>,
    aig_6_resource_quantity: Option<String>,
    aig_7_resource_quantity_units: Option<String>,
    aig_8_start_date_time: Option<String>,
    aig_9_start_date_time_offset: Option<String>,
    aig_10_start_date_time_offset_units: Option<String>,
    aig_11_duration: Option<String>,
    aig_12_duration_units: Option<String>,
    aig_13_allow_substitution_code: Option<String>,
    aig_14_filler_status_code: Option<String>,
});
