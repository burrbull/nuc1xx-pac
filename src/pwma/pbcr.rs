#[doc = "Reader of register PBCR"]
pub type R = crate::R<u32, super::PBCR>;
#[doc = "Writer for register PBCR"]
pub type W = crate::W<u32, super::PBCR>;
#[doc = "Register PBCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCn`"]
pub type BCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCn`"]
pub struct BCN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM Backward Compatible Register 0 = PWM register action is compatible with Medium Density 1 = PWM register action is not compatible with Medium Density Please reference CCR0/CCR2 register bit 6, 7, 22, 23 description"]
    #[inline(always)]
    pub fn bcn(&self) -> BCN_R {
        BCN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Backward Compatible Register 0 = PWM register action is compatible with Medium Density 1 = PWM register action is not compatible with Medium Density Please reference CCR0/CCR2 register bit 6, 7, 22, 23 description"]
    #[inline(always)]
    pub fn bcn(&mut self) -> BCN_W {
        BCN_W { w: self }
    }
}
