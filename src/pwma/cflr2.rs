#[doc = "Reader of register CFLR2"]
pub type R = crate::R<u32, super::CFLR2>;
#[doc = "Reader of field `CFLR`"]
pub type CFLR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture Falling Latch Register Latch the PWM counter when Channel 2 has Falling transition."]
    #[inline(always)]
    pub fn cflr(&self) -> CFLR_R {
        CFLR_R::new((self.bits & 0xffff) as u16)
    }
}
