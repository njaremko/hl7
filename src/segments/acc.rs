use crate::pub_struct;
pub_struct!(ACC {
    acc_1_accident_date_time: Option<String>,
    acc_2_accident_code: Option<String>,
    acc_3_accident_location: Option<String>,
    acc_4_auto_accident_state: Option<String>,
    acc_5_accident_job_related_indicator: Option<String>,
    acc_6_accident_death_indicator: Option<String>,
    acc_7_entered_by: Option<String>,
    acc_8_accident_description: Option<String>,
    acc_9_brought_in_by: Option<String>,
    acc_10_police_notified_indicator: Option<String>,
    acc_11_accident_address: Option<String>,
    acc_12_degree_of_patient_liability: Option<String>,
    acc_13_accident_identifier: Option<Vec<String>>,
});
