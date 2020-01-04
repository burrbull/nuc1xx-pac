#[doc = "Reader of register TDR"]
pub type R = crate::R<u32, super::TDR>;
#[doc = "Reader of field `TDR`"]
pub type TDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Timer Data Register When TCSR.TDR_EN is set to 1, the internal 24-bit up-timer value will be loaded into TDR. User can read this register for the up-timer value."]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
