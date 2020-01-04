#[doc = "Reader of register ADCMPR0"]
pub type R = crate::R<u32, super::ADCMPR0>;
#[doc = "Writer for register ADCMPR0"]
pub type W = crate::W<u32, super::ADCMPR0>;
#[doc = "Register ADCMPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCMPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
#[doc = "Reader of field `CMPIE`"]
pub type CMPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPIE`"]
pub struct CMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIE_W<'a> {
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
#[doc = "Reader of field `CMPCOND`"]
pub type CMPCOND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPCOND`"]
pub struct CMPCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCOND_W<'a> {
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
#[doc = "Reader of field `CMPCH`"]
pub type CMPCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPCH`"]
pub struct CMPCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `CMPMATCNT`"]
pub type CMPMATCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPMATCNT`"]
pub struct CMPMATCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMATCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMPD`"]
pub type CMPD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPD`"]
pub struct CMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Compare Enable 1 = Enable compare. 0 = Disable compare. Set this bit to 1 to enable ADC controller to compare CMPD\\[11:0\\]
with specified channel conversion result when converted data is loaded into ADDR register."]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare Interrupt Enable 1 = Enable 0 = Disable If the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPF0 bit will be asserted, in the meanwhile, if CMPIE is set to 1, a compare interrupt request is generated."]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Condition 1 = Set the compare condition as that when a 12-bit A/D conversion result is greater or equal to the 12-bit CMPD (ADCMPR0\\[27:16\\]), the internal match counter will increase one. 0 = Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPD (ADCMPR0\\[27:16\\]), the internal match counter will increase one. When the internal counter reaches the value to (CMPMATCNT +1), the CMPF0 bit will be set."]
    #[inline(always)]
    pub fn cmpcond(&self) -> CMPCOND_R {
        CMPCOND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Compare Channel Selection 000 = Channel 0 conversion result is selected to be compared. 001 = Channel 1 conversion result is selected to be compared. 010 = Channel 2 conversion result is selected to be compared. 011 = Channel 3 conversion result is selected to be compared. 100 = Channel 4 conversion result is selected to be compared. 101 = Channel 5 conversion result is selected to be compared. 110 = Channel 6 conversion result is selected to be compared. 111 = Channel 7 conversion result is selected to be compared."]
    #[inline(always)]
    pub fn cmpch(&self) -> CMPCH_R {
        CMPCH_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Compare Match Count When the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND\\[2\\], the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPF0 bit will be set."]
    #[inline(always)]
    pub fn cmpmatcnt(&self) -> CMPMATCNT_R {
        CMPMATCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Comparison Data The 12 bits data is used to compare with conversion result of specified channel. Software can use it to monitor the external analog input pin voltage transition in scan mode without imposing a load on software. The following description is only supported in Low Density. When DMOF bit is set to 0, ADC comparator compares CMPD with conversion result with unsigned format. CMPD should be filled in unsigned format. When DMOF bit is set to 1, ADC comparator compares CMPD with conversion result with 2's complement format. CMPD should be filled in 2's complement format."]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Enable 1 = Enable compare. 0 = Disable compare. Set this bit to 1 to enable ADC controller to compare CMPD\\[11:0\\]
with specified channel conversion result when converted data is loaded into ADDR register."]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 1 - Compare Interrupt Enable 1 = Enable 0 = Disable If the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPF0 bit will be asserted, in the meanwhile, if CMPIE is set to 1, a compare interrupt request is generated."]
    #[inline(always)]
    pub fn cmpie(&mut self) -> CMPIE_W {
        CMPIE_W { w: self }
    }
    #[doc = "Bit 2 - Compare Condition 1 = Set the compare condition as that when a 12-bit A/D conversion result is greater or equal to the 12-bit CMPD (ADCMPR0\\[27:16\\]), the internal match counter will increase one. 0 = Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPD (ADCMPR0\\[27:16\\]), the internal match counter will increase one. When the internal counter reaches the value to (CMPMATCNT +1), the CMPF0 bit will be set."]
    #[inline(always)]
    pub fn cmpcond(&mut self) -> CMPCOND_W {
        CMPCOND_W { w: self }
    }
    #[doc = "Bits 3:5 - Compare Channel Selection 000 = Channel 0 conversion result is selected to be compared. 001 = Channel 1 conversion result is selected to be compared. 010 = Channel 2 conversion result is selected to be compared. 011 = Channel 3 conversion result is selected to be compared. 100 = Channel 4 conversion result is selected to be compared. 101 = Channel 5 conversion result is selected to be compared. 110 = Channel 6 conversion result is selected to be compared. 111 = Channel 7 conversion result is selected to be compared."]
    #[inline(always)]
    pub fn cmpch(&mut self) -> CMPCH_W {
        CMPCH_W { w: self }
    }
    #[doc = "Bits 8:11 - Compare Match Count When the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND\\[2\\], the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPF0 bit will be set."]
    #[inline(always)]
    pub fn cmpmatcnt(&mut self) -> CMPMATCNT_W {
        CMPMATCNT_W { w: self }
    }
    #[doc = "Bits 16:27 - Comparison Data The 12 bits data is used to compare with conversion result of specified channel. Software can use it to monitor the external analog input pin voltage transition in scan mode without imposing a load on software. The following description is only supported in Low Density. When DMOF bit is set to 0, ADC comparator compares CMPD with conversion result with unsigned format. CMPD should be filled in unsigned format. When DMOF bit is set to 1, ADC comparator compares CMPD with conversion result with 2's complement format. CMPD should be filled in 2's complement format."]
    #[inline(always)]
    pub fn cmpd(&mut self) -> CMPD_W {
        CMPD_W { w: self }
    }
}
