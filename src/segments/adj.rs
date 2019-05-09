use crate::pub_struct;
pub_struct!(ADJ {
    adj_1_provider_adjustment_number: String,
    adj_2_payer_adjustment_number: String,
    adj_3_adjustment_sequence_number: String,
    adj_4_adjustment_category: String,
    adj_5_adjustment_amount: Option<String>,
    adj_6_adjustment_quantity: Option<String>,
    adj_7_adjustment_reason_pa: Option<String>,
    adj_8_adjustment_description: Option<String>,
    adj_9_original_value: Option<String>,
    adj_10_substitute_value: Option<String>,
    adj_11_adjustment_action: Option<String>,
    adj_12_provider_adjustment_number_cross_reference: Option<String>,
    adj_13_provider_product_service_line_item_number_cross_reference: Option<String>,
    adj_14_adjustment_date: String,
    adj_15_responsible_organization: Option<String>,
});
