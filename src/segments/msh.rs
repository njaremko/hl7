use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Default)]
pub struct MSH {
    pub msh1_field_separator: String,
    pub msh2_encoding_characters: String,
    pub msh3_sending_application: Option<String>,
    pub msh4_sending_facility: Option<String>,
    pub msh5_receiving_application: Option<String>,
    pub msh6_receiving_facility: Option<String>,
    pub msh7_date_time_of_message: String,
    pub msh8_security: Option<String>,
    pub msh9_message_type: String,
    pub msh10_message_control_id: String,
    pub msh11_processing_id: String,
    pub msh12_version_id: String,
    pub msh13_sequence_number: Option<String>,
    pub msh14_continuation_pointer: Option<String>,
    pub msh15_accept_acknowledgment_type: Option<String>,
    pub msh16_application_acknowledgment_type: Option<String>,
    pub msh17_country_code: Option<String>,
    pub msh18_character_set: Option<Vec<String>>,
    pub msh19_principal_language_of_message: Option<String>,
    pub msh20_alternate_character_set_handling_scheme: Option<String>,
    pub msh21_message_profile_identifier: Option<Vec<String>>,
    pub msh22_sending_responsible_organization: Option<String>,
    pub msh23_receiving_responsible_organization: Option<String>,
    pub msh24_sending_network_address: Option<String>,
    pub msh25_receiving_network_address: Option<String>,
}

fn some_if_not_empty(x: &str) -> Option<String> {
    if x.len() > 0 {
                Some(x.to_string())
            } else {
                None
            }
}

impl FromStr for MSH {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut msh = MSH::default();
        assert!(&s[0..3] == "MSH");
        let delimiter = &s[3..4];
        let mut split_input = s.split(delimiter);
        msh.msh1_field_separator = delimiter.to_string();
        split_input.next();
        msh.msh2_encoding_characters = split_input.next().unwrap().to_string();
        split_input.next().and_then(|x| {
            msh.msh3_sending_application = some_if_not_empty(x);
            split_input.next()
        }).and_then(|x| {
            msh.msh4_sending_facility = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh5_receiving_application = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh6_receiving_facility = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh7_date_time_of_message = x.to_string();
            split_input.next()
        })
        .and_then(|x| {
            msh.msh8_security = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh9_message_type = x.to_string();
            split_input.next()
        })
        .and_then(|x| {
            msh.msh10_message_control_id = x.to_string();
            split_input.next()
        })
        .and_then(|x| {
            msh.msh11_processing_id = x.to_string();
            split_input.next()
        })
        .and_then(|x| {
            msh.msh12_version_id = x.to_string();
            split_input.next()
        })
        .and_then(|x| {
            msh.msh13_sequence_number = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh14_continuation_pointer = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh15_accept_acknowledgment_type = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh16_application_acknowledgment_type = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh17_country_code = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh18_character_set = None;
            split_input.next()
        })
        .and_then(|x| {
            msh.msh19_principal_language_of_message = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh20_alternate_character_set_handling_scheme = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh21_message_profile_identifier = None;
            split_input.next()
        })
        .and_then(|x| {
            msh.msh22_sending_responsible_organization = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh23_receiving_responsible_organization = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh24_sending_network_address = some_if_not_empty(x);
            split_input.next()
        })
        .and_then(|x| {
            msh.msh25_receiving_network_address = some_if_not_empty(x);
            split_input.next()
        });
        Ok(msh)
    }
}