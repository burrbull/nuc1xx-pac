#[doc = "Reader of register ADCALR"]
pub type R = crate::R<u32, super::ADCALR>;
#[doc = "Writer for register ADCALR"]
pub type W = crate::W<u32, super::ADCALR>;
#[doc = "Register ADCALR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCALR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALEN`"]
pub type CALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALEN`"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
#[doc = "Reader of field `CALDONE`"]
pub type CALDONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Self Calibration Enable 1 = Enable self calibration 0 = Disable self calibration Software can set this bit to 1 enables A/D converter to do self calibration function. It needs 127 ADC clocks to complete calibration. This bit must be kept at 1 after CALDONE asserted. Clearing this bit will disable self calibration function."]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Calibration is Done 1 = A/D converter self calibration is done 0 = A/D converter has not been calibrated or calibration is in progress if CALEN bit is set. When 0 is written to CALEN bit, CALDONE bit is cleared by hardware immediately. It is a read only bit."]
    #[inline(always)]
    pub fn caldone(&self) -> CALDONE_R {
        CALDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Self Calibration Enable 1 = Enable self calibration 0 = Disable self calibration Software can set this bit to 1 enables A/D converter to do self calibration function. It needs 127 ADC clocks to complete calibration. This bit must be kept at 1 after CALDONE asserted. Clearing this bit will disable self calibration function."]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
}
