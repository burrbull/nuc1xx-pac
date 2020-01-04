#[doc = "Reader of register ADSR"]
pub type R = crate::R<u32, super::ADSR>;
#[doc = "Writer for register ADSR"]
pub type W = crate::W<u32, super::ADSR>;
#[doc = "Register ADSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADF`"]
pub type ADF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADF`"]
pub struct ADF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADF_W<'a> {
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
#[doc = "Reader of field `CMPF0`"]
pub type CMPF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPF0`"]
pub struct CMPF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CMPF1`"]
pub type CMPF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPF1`"]
pub struct CMPF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHANNEL`"]
pub type CHANNEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - A/D Conversion End Flag A status flag that indicates the end of A/D conversion. ADF is set to 1 at these two conditions: When A/D conversion ends in single mode When A/D conversion ends on all specified channels in scan mode. This flag can be cleared by writing 1 to itself."]
    #[inline(always)]
    pub fn adf(&self) -> ADF_R {
        ADF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare Flag When the selected channel A/D conversion result meets the setting conditions of ADCMPR0 then this bit will be set to 1. And it can be cleared by writing 1 to itself. 1 = Conversion result in ADDR meets ADCMPR0 setting 0 = Conversion result in ADDR does not meet ADCMPR0 setting"]
    #[inline(always)]
    pub fn cmpf0(&self) -> CMPF0_R {
        CMPF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Flag When the selected channel A/D conversion result meets the setting conditions of ADCMPR1 then this bit will be set to 1. And it can be cleared by writing 1 to itself. 1 = Conversion result in ADDR meets ADCMPR1 setting 0 = Conversion result in ADDR does not meet ADCMPR1 setting"]
    #[inline(always)]
    pub fn cmpf1(&self) -> CMPF1_R {
        CMPF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BUSY/IDLE 1 = A/D converter is busy at conversion. 0 = A/D converter is in idle state. This bit is mirror of as ADST bit in ADCR. It is read only."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Current Conversion Channel This filed reflects current conversion channel when BUSY=1. When BUSY=0, it shows the next channel will be converted. It is read only."]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Data Valid flag It is a mirror of VALID bit in ADDRx It is read only."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Over Run flag It is a mirror of OVERRUN bit in ADDRx It is read only."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Conversion End Flag A status flag that indicates the end of A/D conversion. ADF is set to 1 at these two conditions: When A/D conversion ends in single mode When A/D conversion ends on all specified channels in scan mode. This flag can be cleared by writing 1 to itself."]
    #[inline(always)]
    pub fn adf(&mut self) -> ADF_W {
        ADF_W { w: self }
    }
    #[doc = "Bit 1 - Compare Flag When the selected channel A/D conversion result meets the setting conditions of ADCMPR0 then this bit will be set to 1. And it can be cleared by writing 1 to itself. 1 = Conversion result in ADDR meets ADCMPR0 setting 0 = Conversion result in ADDR does not meet ADCMPR0 setting"]
    #[inline(always)]
    pub fn cmpf0(&mut self) -> CMPF0_W {
        CMPF0_W { w: self }
    }
    #[doc = "Bit 2 - Compare Flag When the selected channel A/D conversion result meets the setting conditions of ADCMPR1 then this bit will be set to 1. And it can be cleared by writing 1 to itself. 1 = Conversion result in ADDR meets ADCMPR1 setting 0 = Conversion result in ADDR does not meet ADCMPR1 setting"]
    #[inline(always)]
    pub fn cmpf1(&mut self) -> CMPF1_W {
        CMPF1_W { w: self }
    }
}
