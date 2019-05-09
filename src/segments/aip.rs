use crate::pub_struct;
pub_struct!(AIP {
    aip_1_set_id: String,
    aip_2_segment_action_code: Option<String>,
    aip_3_personnel_resource_id: Option<Vec<String>>,
    aip_4_resource_type: Option<String>,
    aip_5_resource_group: Option<String>,
    aip_6_start_date_time: Option<String>,
    aip_7_start_date_time_offset: Option<String>,
    aip_8_start_date_time_offset_units: Option<String>,
    aip_9_duration: Option<String>,
    aip_10_duration_units: Option<String>,
    aip_11_allow_substitution_code: Option<String>,
    aip_12_filler_status_code: Option<String>,
});
