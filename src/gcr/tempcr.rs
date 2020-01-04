#[doc = "Reader of register TEMPCR"]
pub type R = crate::R<u32, super::TEMPCR>;
#[doc = "Writer for register TEMPCR"]
pub type W = crate::W<u32, super::TEMPCR>;
#[doc = "Register TEMPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TEMPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VTEMP_EN`"]
pub type VTEMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTEMP_EN`"]
pub struct VTEMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VTEMP_EN_W<'a> {
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
    #[doc = "Bit 0 - Temperature sensor Enable This bit is used to enable/disable temperature sensor function. 1 = Enabled temperature sensor function 0 = Disabled temperature sensor function (default) After this bit is set to 1, the value of temperature can get from ADC conversion result by ADC channel selecting channel 7 and alternative multiplexer channel selecting temperature sensor. Detail ADC conversion function please reference ADC function chapter."]
    #[inline(always)]
    pub fn vtemp_en(&self) -> VTEMP_EN_R {
        VTEMP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor Enable This bit is used to enable/disable temperature sensor function. 1 = Enabled temperature sensor function 0 = Disabled temperature sensor function (default) After this bit is set to 1, the value of temperature can get from ADC conversion result by ADC channel selecting channel 7 and alternative multiplexer channel selecting temperature sensor. Detail ADC conversion function please reference ADC function chapter."]
    #[inline(always)]
    pub fn vtemp_en(&mut self) -> VTEMP_EN_W {
        VTEMP_EN_W { w: self }
    }
}
