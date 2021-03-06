use crate::pub_struct;
pub_struct!(BPX {
    bpx_1_set_id: String,
    bpx_2_bp_dispense_status: String,
    bpx_3_bp_status: String,
    bpx_4_bp_date_time_of_status: String,
    bpx_5_bc_donation_id: Option<String>,
    bpx_6_bc_component: Option<String>,
    bpx_7_bc_donation_type_intended_use: Option<String>,
    bpx_8_cp_commercial_product: Option<String>,
    bpx_9_cp_manufacturer: Option<String>,
    bpx_10_cp_lot_number: Option<String>,
    bpx_11_bp_blood_group: Option<String>,
    bpx_12_bc_special_testing: Option<Vec<String>>,
    bpx_13_bp_expiration_date_time: Option<String>,
    bpx_14_bp_quantity: String,
    bpx_15_bp_amount: Option<String>,
    bpx_16_bp_units: Option<String>,
    bpx_17_bp_unique_id: Option<String>,
    bpx_18_bp_actual_dispensed_to_location: Option<String>,
    bpx_19_bp_actual_dispensed_to_address: Option<String>,
    bpx_20_bp_dispensed_to_receiver: Option<String>,
    bpx_21_bp_dispensing_individual: Option<String>,
});
