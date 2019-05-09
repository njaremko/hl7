pub struct BPO {
    bpo1_set_id:String,
    bpo2_bp_universal_service_identifier: String,
    bpo3_bp_processing_requirements: Option<Vec<String>>,
    bpo4_bp_quantity: String,
    bpo5_bp_amount: Option<String>,
    bpo6_bp_units: Option<String>,
    bpo7_bp_intended_use_date_time: Option<String>,
    bpo8_bp_intended_dispense_from_location: Option<String>,
    bpo9_bp_intended_dispense_from_address: Option<String>,
    bpo10_bp_requested_dispense_date_time: Option<String>,
    bpo11_bp_requested_dispense_to_location: Option<String>,
    bpo12_bp_requested_dispense_to_address: Option<String>,
    bpo13_bp_indication_for_use: Option<Vec<String>>,
    bpo14_bp_informed_consent_indicator: Option<String>, 
}