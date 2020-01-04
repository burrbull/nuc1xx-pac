#[doc = "Reader of register CFLR0"]
pub type R = crate::R<u32, super::CFLR0>;
#[doc = "Reader of field `CFLR`"]
pub type CFLR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture Falling Latch Register Latch the PWM counter when Channel 0 has Falling transition."]
    #[inline(always)]
    pub fn cflr(&self) -> CFLR_R {
        CFLR_R::new((self.bits & 0xffff) as u16)
    }
}
