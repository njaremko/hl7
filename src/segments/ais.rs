use crate::pub_struct;
pub_struct!(AIS {
    ais_1_set_id: String,
    ais_2_segment_action_code: Option<String>,
    ais_3_universal_service_identifier: String,
    ais_4_start_date_time: Option<String>,
    ais_5_start_date_time_offset: Option<String>,
    ais_6_start_date_time_offset_units: Option<String>,
    ais_7_duration: Option<String>,
    ais_8_duration_units: Option<String>,
    ais_9_allow_substitution_code: Option<String>,
    ais_10_filler_status_code: Option<String>,
    ais_11_placer_supplemental_service_information: Option<Vec<String>>,
    ais_12_filler_supplemental_service_information: Option<Vec<String>>,
});
