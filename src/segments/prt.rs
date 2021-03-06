use crate::pub_struct;
pub_struct!(PRT {
    prt_1_participation_instance: Option<String>,
    prt_2_action_code: String,
    prt_3_action_reason: Option<String>,
    prt_4_participation: String,
    prt_5_participation_person: Option<Vec<String>>,
    prt_6_participation_person_provider_type: Option<String>,
    prt_7_participant_organization_unit_type: Option<String>,
    prt_8_participation_organization: Option<Vec<String>>,
    prt_9_participant_location: Option<Vec<String>>,
    prt_10_participation_device: Option<Vec<String>>,
    prt_11_participation_begin_date_time: Option<String>,
    prt_12_participation_end_date_time: Option<String>,
    prt_13_participation_qualitative_duration: Option<String>,
    prt_14_participation_address: Option<Vec<String>>,
    prt_15_participant_telecommunication_address: Option<Vec<String>>,
});
