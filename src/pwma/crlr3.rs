#[doc = "Reader of register CRLR3"]
pub type R = crate::R<u32, super::CRLR3>;
#[doc = "Reader of field `CRLR`"]
pub type CRLR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture Rising Latch Register Latch the PWM counter when Channel 3 has rising transition."]
    #[inline(always)]
    pub fn crlr(&self) -> CRLR_R {
        CRLR_R::new((self.bits & 0xffff) as u16)
    }
}
