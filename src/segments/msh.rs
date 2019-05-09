use crate::segments::SegmentParsingError;
use crate::pub_struct;
use std::str::FromStr;

pub_struct!(MSH {
    msh_1_field_separator: String,
    msh_2_encoding_characters: String,
    msh_3_sending_application: Option<String>,
    msh_4_sending_facility: Option<String>,
    msh_5_receiving_application: Option<String>,
    msh_6_receiving_facility: Option<String>,
    msh_7_date_time_of_message: String,
    msh_8_security: Option<String>,
    msh_9_message_type: String,
    msh_10_message_control_id: String,
    msh_11_processing_id: String,
    msh_12_version_id: String,
    msh_13_sequence_number: Option<String>,
    msh_14_continuation_pointer: Option<String>,
    msh_15_accept_acknowledgment_type: Option<String>,
    msh_16_application_acknowledgment_type: Option<String>,
    msh_17_country_code: Option<String>,
    msh_18_character_set: Option<Vec<String>>,
    msh_19_principal_language_of_message: Option<String>,
    msh_20_alternate_character_set_handling_scheme: Option<String>,
    msh_21_message_profile_identifier: Option<Vec<String>>,
    msh_22_sending_responsible_organization: Option<String>,
    msh_23_receiving_responsible_organization: Option<String>,
    msh_24_sending_network_address: Option<String>,
    msh_25_receiving_network_address: Option<String>,
});

fn some_if_not_empty(x: &str) -> Option<String> {
    if x.len() > 0 {
        Some(x.to_string())
    } else {
        None
    }
}

fn split_repeated(repeat_delim: &str, x: &str) -> Option<Vec<String>> {
    let y: Vec<String> = x.split(repeat_delim).map(|y| y.to_string()).collect();
    if y.is_empty() {
        None
    } else {
        Some(y)
    }
}

impl FromStr for MSH {
    type Err = SegmentParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut msh = MSH::default();
        let delimiter = &s[3..4];
        let mut split_input = s.split(delimiter);

        assert!(split_input.next().unwrap() == "MSH");

        let encoding_chars = split_input.next().unwrap();
        let component_delim = &encoding_chars[0..1];
        let repeat_delim = &encoding_chars[1..2];
        let escape_char = &encoding_chars[2..3];
        let sub_component_delim = &encoding_chars[3..4];

        msh.msh_1_field_separator = delimiter.to_string();
        msh.msh_2_encoding_characters = encoding_chars.to_string();
        split_input
            .next()
            .and_then(|x| {
                msh.msh_3_sending_application = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_4_sending_facility = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_5_receiving_application = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_6_receiving_facility = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_7_date_time_of_message = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_8_security = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_9_message_type = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_10_message_control_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_11_processing_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_12_version_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_13_sequence_number = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_14_continuation_pointer = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_15_accept_acknowledgment_type = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_16_application_acknowledgment_type = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_17_country_code = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_18_character_set = split_repeated(repeat_delim, x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_19_principal_language_of_message = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_20_alternate_character_set_handling_scheme = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_21_message_profile_identifier = split_repeated(repeat_delim, x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_22_sending_responsible_organization = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_23_receiving_responsible_organization = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_24_sending_network_address = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_25_receiving_network_address = some_if_not_empty(x);
                split_input.next()
            });
        Ok(msh)
    }
}
