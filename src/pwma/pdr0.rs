#[doc = "Reader of register PDR0"]
pub type R = crate::R<u32, super::PDR0>;
#[doc = "Reader of field `PDR`"]
pub type PDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM Data Register User can monitor PDR to know current value in 16-bit down counter."]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new((self.bits & 0xffff) as u16)
    }
}
