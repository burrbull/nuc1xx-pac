#[doc = "Reader of register RIIR"]
pub type R = crate::R<u32, super::RIIR>;
#[doc = "Writer for register RIIR"]
pub type W = crate::W<u32, super::RIIR>;
#[doc = "Register RIIR `reset()`'s with value 0"]
impl crate::ResetValue for super::RIIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AIF`"]
pub type AIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIF`"]
pub struct AIF_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Alarm Interrupt Flag When RTC Alarm Interrupt is enabled (RIER.AIER=1), RTC controller will set AIF to high once the RTC real time counters TLR and CLR reach the alarm setting time registers TAR and CAR. This bit is software clear by writing 1 to it. 1 = Indicates RTC Alarm Interrupt is requested if RIER.AIER=1. 0 = Indicates RTC Alarm Interrupt condition never occurred."]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Time Tick Interrupt Flag When RTC Time Tick Interrupt is enabled (RIER.TIER=1), RTC controller will set TIF to high periodically in the period selected by TTR\\[2:0\\]. This bit is software clear by writing 1 to it. 1 = Indicates RTC Time Tick Interrupt is requested if RIER.TIER=1. 0 = Indicates RTC Time Tick Interrupt condition never occurred."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alarm Interrupt Flag When RTC Alarm Interrupt is enabled (RIER.AIER=1), RTC controller will set AIF to high once the RTC real time counters TLR and CLR reach the alarm setting time registers TAR and CAR. This bit is software clear by writing 1 to it. 1 = Indicates RTC Alarm Interrupt is requested if RIER.AIER=1. 0 = Indicates RTC Alarm Interrupt condition never occurred."]
    #[inline(always)]
    pub fn aif(&mut self) -> AIF_W {
        AIF_W { w: self }
    }
    #[doc = "Bit 1 - RTC Time Tick Interrupt Flag When RTC Time Tick Interrupt is enabled (RIER.TIER=1), RTC controller will set TIF to high periodically in the period selected by TTR\\[2:0\\]. This bit is software clear by writing 1 to it. 1 = Indicates RTC Time Tick Interrupt is requested if RIER.TIER=1. 0 = Indicates RTC Time Tick Interrupt condition never occurred."]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
}
