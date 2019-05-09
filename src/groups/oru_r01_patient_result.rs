use crate::groups::*;
use crate::pub_struct;

pub_struct!(ORU_R01_PATIENT_RESULT {
    oru_r01_patient: Option<ORU_R01_PATIENT>,
    oru_r01_patient_observation: Vec<ORU_R01_PATIENT_OBSERVATION>,
});
