#[doc = "Reader of register LIR"]
pub type R = crate::R<u32, super::LIR>;
#[doc = "Reader of field `LIR`"]
pub type LIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Leap Year Indication REGISTER (Real only). 1 = It indicate that this year is leap year 0 = It indicate that this year is not a leap year"]
    #[inline(always)]
    pub fn lir(&self) -> LIR_R {
        LIR_R::new((self.bits & 0x01) != 0)
    }
}
