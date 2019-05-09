use crate::pub_struct;
pub_struct!(BPO {
    bpo_1_set_id: String,
    bpo_2_bp_universal_service_identifier: String,
    bpo_3_bp_processing_requirements: Option<Vec<String>>,
    bpo_4_bp_quantity: String,
    bpo_5_bp_amount: Option<String>,
    bpo_6_bp_units: Option<String>,
    bpo_7_bp_intended_use_date_time: Option<String>,
    bpo_8_bp_intended_dispense_from_location: Option<String>,
    bpo_9_bp_intended_dispense_from_address: Option<String>,
    bpo_10_bp_requested_dispense_date_time: Option<String>,
    bpo_11_bp_requested_dispense_to_location: Option<String>,
    bpo_12_bp_requested_dispense_to_address: Option<String>,
    bpo_13_bp_indication_for_use: Option<Vec<String>>,
    bpo_14_bp_informed_consent_indicator: Option<String>,
});
