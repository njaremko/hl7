use crate::segments::*;

pub struct ORU_R01_VISIT {
    pv1: PV1,
    pv2: Option<PV2>,
    prt: Option<Vec<PRT>>,
}
