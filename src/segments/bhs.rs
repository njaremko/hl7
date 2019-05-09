use crate::pub_struct;
pub_struct!(BHS {
    bhs1_batch_field_separator: String,
    bhs2_batch_encoding_characters: String,
    bhs3_batch_sending_application: Option<String>,
    bhs4_batch_sending_facility: Option<String>,
    bhs5_batch_receiving_application: Option<String>,
    bhs6_batch_receiving_facility: Option<String>,
    bhs7_batch_creation_date_time: Option<String>,
    bhs8_batch_security: Option<String>,
    bhs9_batch_name_id_type: Option<String>,
    bhs10_batch_comment: Option<String>,
    bhs11_batch_control_id: Option<String>,
    bhs12_reference_batch_control_id: Option<String>,
    bhs13_batch_sending_network_address: Option<String>,
    bhs14_batch_receiving_network_address: Option<String>,
});
