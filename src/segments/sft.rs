use crate::pub_struct;
pub_struct!(SFT {
    sft_1_software_vendor_organization: String,
    sft_2_software_certified_version_or_release_number: String,
    sft_3_software_product_name: String,
    sft_4_software_binary_id: String,
    sft_5_software_product_information: Option<String>,
    sft_6_software_install_date: Option<String>,
});
