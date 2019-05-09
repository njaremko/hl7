pub_struct!(AIS {
    ais1_set_id: String,
    ais2_segment_action_code: Option<String>,
    ais3_universal_service_identifier: String,
    ais4_start_date_time: Option<String>,
    ais5_start_date_time_offset: Option<String>,
    ais6_start_date_time_offset_units: Option<String>,
    ais7_duration: Option<String>,
    ais8_duration_units: Option<String>,
    ais9_allow_substitution_code: Option<String>,
    ais10_filler_status_code: Option<String>,
    ais11_placer_supplemental_service_information: Option<Vec<String>>,
    ais12_filler_supplemental_service_information: Option<Vec<String>>,
});
