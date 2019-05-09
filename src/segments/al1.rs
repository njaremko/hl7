use crate::pub_struct;
pub_struct!(AL1 {
    al1_1_set_id: String,
    al1_2_allergen_type_code: Option<String>,
    al1_3_allergen_code_mnemonic_description: String,
    al1_4_allergy_severity_code: Option<String>,
    al1_5_allergy_reaction_code: Option<Vec<String>>,
    al1_6_identification_date: Option<String>,
});
