macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Default, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

mod abs;
mod acc;
mod add;
mod adj;
mod aff;
mod aig;
mod ail;
mod aip;
mod ais;
mod al1;
mod apr;
mod arq;
mod arv;
mod aut;
mod bhs;
mod blc;
mod blg;
mod bpo;
mod bpx;
mod bts;
mod btx;
mod bui;
mod msh;
mod nte;
mod nk1;
mod obx;
mod orc;
mod pd1;
mod pid;
mod prt;
mod pv1;
mod pv2;
mod dsc;
mod sft;
mod uac;
mod segment;

pub use abs::*;
pub use dsc::*;
pub use segment::*;
pub use acc::*;
pub use add::*;
pub use adj::*;
pub use aff::*;
pub use aig::*;
pub use ail::*;
pub use aip::*;
pub use ais::*;
pub use al1::*;
pub use apr::*;
pub use arq::*;
pub use arv::*;
pub use aut::*;
pub use bhs::*;
pub use blc::*;
pub use blg::*;
pub use bpo::*;
pub use bpx::*;
pub use bts::*;
pub use btx::*;
pub use bui::*;
pub use msh::*;
pub use nte::*;
pub use nk1::*;
pub use obx::*;
pub use orc::*;
pub use pd1::*;
pub use pid::*;
pub use prt::*;
pub use pv1::*;
pub use pv2::*;
pub use sft::*;
pub use uac::*;
