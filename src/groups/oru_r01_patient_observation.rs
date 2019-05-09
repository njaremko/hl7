use crate::pub_struct;
use crate::segments::*;

pub_struct!(ORU_R01_PATIENT_OBSERVATION {
    obx: OBX,
    prt: Option<Vec<PRT>>,
});
