use crate::segments::*;

pub struct ORU_R01_PATIENT_OBSERVATION {
    obx: OBX,
    prt: Option<Vec<PRT>>,
}
