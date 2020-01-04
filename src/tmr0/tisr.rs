#[doc = "Reader of register TISR"]
pub type R = crate::R<u32, super::TISR>;
#[doc = "Writer for register TISR"]
pub type W = crate::W<u32, super::TISR>;
#[doc = "Register TISR `reset()`'s with value 0"]
impl crate::ResetValue for super::TISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIF`"]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
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
    #[doc = "Bit 0 - Timer Interrupt Flag This bit indicates the interrupt status of Timer. TIF bit is set by hardware when the up counting value of internal 24-bit timer matches the timer compared value (TCMP). It is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag This bit indicates the interrupt status of Timer. TIF bit is set by hardware when the up counting value of internal 24-bit timer matches the timer compared value (TCMP). It is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
}
