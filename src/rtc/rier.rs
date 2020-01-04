#[doc = "Reader of register RIER"]
pub type R = crate::R<u32, super::RIER>;
#[doc = "Writer for register RIER"]
pub type W = crate::W<u32, super::RIER>;
#[doc = "Register RIER `reset()`'s with value 0"]
impl crate::ResetValue for super::RIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AIER`"]
pub type AIER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIER`"]
pub struct AIER_W<'a> {
    w: &'a mut W,
}
impl<'a> AIER_W<'a> {
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
#[doc = "Reader of field `TIER`"]
pub type TIER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIER`"]
pub struct TIER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_W<'a> {
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
    #[doc = "Bit 0 - Alarm Interrupt Enable 1 = RTC Alarm Interrupt is enabled 0 = RTC Alarm Interrupt is disabled"]
    #[inline(always)]
    pub fn aier(&self) -> AIER_R {
        AIER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Tick Interrupt Enable 1 = RTC Time Tick Interrupt is enabled 0 = RTC Time Tick Interrupt is disabled"]
    #[inline(always)]
    pub fn tier(&self) -> TIER_R {
        TIER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm Interrupt Enable 1 = RTC Alarm Interrupt is enabled 0 = RTC Alarm Interrupt is disabled"]
    #[inline(always)]
    pub fn aier(&mut self) -> AIER_W {
        AIER_W { w: self }
    }
    #[doc = "Bit 1 - Time Tick Interrupt Enable 1 = RTC Time Tick Interrupt is enabled 0 = RTC Time Tick Interrupt is disabled"]
    #[inline(always)]
    pub fn tier(&mut self) -> TIER_W {
        TIER_W { w: self }
    }
}
