use crate::pub_struct;
pub_struct!(AIL {
    ail_1_set_id: String,
    ail_2_segment_action_code: Option<String>,
    ail_3_location_resource_id: Option<Vec<String>>,
    ail_4_location_type: Option<String>,
    ail_5_location_group: Option<String>,
    ail_6_start_date_time: Option<String>,
    ail_7_start_date_time_offset: Option<String>,
    ail_8_start_date_time_offset_units: Option<String>,
    ail_9_duration: Option<String>,
    ail_10_duration_units: Option<String>,
    ail_11_allow_substitution_code: Option<String>,
    ail_12_filler_status_code: Option<String>,
});
