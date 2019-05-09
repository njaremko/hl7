use crate::segments::*;
use crate::groups::*;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Default)]
pub struct ORU_R01 {
    msh: MSH,
    sft: Option<Vec<SFT>>,
    uac: Option<UAC>,
    oru_r01_patient_result: Vec<ORU_R01_PATIENT_RESULT>,
    dsc: Option<DSC>,
}

impl FromStr for ORU_R01 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut oru = ORU_R01::default();
        let mut lines = s.lines();
        let msh: Option<MSH> = lines.next().map(|msh| msh.parse::<MSH>().unwrap());
        Ok(ORU_R01::default())
    }
}