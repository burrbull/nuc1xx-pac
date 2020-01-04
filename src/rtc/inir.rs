#[doc = "Reader of register INIR"]
pub type R = crate::R<u32, super::INIR>;
#[doc = "Writer for register INIR"]
pub type W = crate::W<u32, super::INIR>;
#[doc = "Register INIR `reset()`'s with value 0"]
impl crate::ResetValue for super::INIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Active`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIR`"]
pub struct INIR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Active Status (Read only), 0: RTC is at reset state 1: RTC is at normal active state."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Initiation When chip is power on, RTC timer counter is at unknown state because RTC timer counter reset is individual with chip reset; user has to write a number (0x a5eb1357) to INIR to reset RTC controller to initialize RTC controller."]
    #[inline(always)]
    pub fn inir(&mut self) -> INIR_W {
        INIR_W { w: self }
    }
}
