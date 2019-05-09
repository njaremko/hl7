pub struct AL1 {
    al1_set_id: String,
    allergen_type_code: Option<String>,
    allergen_code_mnemonic_description: String,
    allergy_severity_code: Option<String>,
    allergy_reaction_code: Option<Vec<String>>,
    identification_date: Option<String>,
}