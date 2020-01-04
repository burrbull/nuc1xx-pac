#[doc = "Reader of register TTR"]
pub type R = crate::R<u32, super::TTR>;
#[doc = "Writer for register TTR"]
pub type W = crate::W<u32, super::TTR>;
#[doc = "Register TTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TTR`"]
pub type TTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TTR`"]
pub struct TTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TWKE`"]
pub type TWKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWKE`"]
pub struct TWKE_W<'a> {
    w: &'a mut W,
}
impl<'a> TWKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Time Tick Register The RTC time tick period for Periodic Time Tick Interrupt request. TTR\\[2:0\\]
Time tick (second) 0 1 1 1/2 2 1/4 3 1/8 4 1/16 5 1/32 6 1/64 7 1/128 Note: This register can be read back after the RTC register access enable bit ENF(AER\\[16\\]) is active."]
    #[inline(always)]
    pub fn ttr(&self) -> TTR_R {
        TTR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - RTC Timer Wakeup CPU Function Enable Bit If TWKE is set before CPU is in power-down mode, when a RTC Time Tick occurs, CPU will be wakened up by RTC controller. 1 = Enable the Wakeup function that CPU can be waken up from power-down mode by Time Tick. 0 = Disable Wakeup CPU function by Time Tick. Note: 1. Tick timer setting follows TTR\\[2:0\\]
description. 2. The CPU can also be wakeup by alarm match occur."]
    #[inline(always)]
    pub fn twke(&self) -> TWKE_R {
        TWKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Time Tick Register The RTC time tick period for Periodic Time Tick Interrupt request. TTR\\[2:0\\]
Time tick (second) 0 1 1 1/2 2 1/4 3 1/8 4 1/16 5 1/32 6 1/64 7 1/128 Note: This register can be read back after the RTC register access enable bit ENF(AER\\[16\\]) is active."]
    #[inline(always)]
    pub fn ttr(&mut self) -> TTR_W {
        TTR_W { w: self }
    }
    #[doc = "Bit 3 - RTC Timer Wakeup CPU Function Enable Bit If TWKE is set before CPU is in power-down mode, when a RTC Time Tick occurs, CPU will be wakened up by RTC controller. 1 = Enable the Wakeup function that CPU can be waken up from power-down mode by Time Tick. 0 = Disable Wakeup CPU function by Time Tick. Note: 1. Tick timer setting follows TTR\\[2:0\\]
description. 2. The CPU can also be wakeup by alarm match occur."]
    #[inline(always)]
    pub fn twke(&mut self) -> TWKE_W {
        TWKE_W { w: self }
    }
}
