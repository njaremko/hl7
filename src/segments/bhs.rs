use crate::pub_struct;
pub_struct!(BHS {
    bhs_1_batch_field_separator: String,
    bhs_2_batch_encoding_characters: String,
    bhs_3_batch_sending_application: Option<String>,
    bhs_4_batch_sending_facility: Option<String>,
    bhs_5_batch_receiving_application: Option<String>,
    bhs_6_batch_receiving_facility: Option<String>,
    bhs_7_batch_creation_date_time: Option<String>,
    bhs_8_batch_security: Option<String>,
    bhs_9_batch_name_id_type: Option<String>,
    bhs_10_batch_comment: Option<String>,
    bhs_11_batch_control_id: Option<String>,
    bhs_12_reference_batch_control_id: Option<String>,
    bhs_13_batch_sending_network_address: Option<String>,
    bhs_14_batch_receiving_network_address: Option<String>,
});
