use crate::groups::*;
use crate::pub_struct;
use crate::segments::*;

pub_struct!(ORU_R01_PATIENT {
    pid: PID,
    pd1: Option<PD1>,
    prt: Option<Vec<PRT>>,
    nte: Option<Vec<NTE>>,
    nk1: Option<Vec<NK1>>,
    arv: Option<Vec<ARV>>,
    oru_r01_patient_observation: Option<Vec<ORU_R01_PATIENT_OBSERVATION>>,
    oru_r01_visit: Option<ORU_R01_VISIT>,
});
