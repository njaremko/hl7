use crate::pub_struct;
pub_struct!(AFF {
    aff_1_set_id: String,
    aff_2_professional_organization: String,
    aff_2_professional_organization_address: Option<String>,
    aff_2_professional_organization_affiliation_date_range: Option<Vec<String>>,
    aff_2_professional_affiliation_additional_information: Option<String>,
});
