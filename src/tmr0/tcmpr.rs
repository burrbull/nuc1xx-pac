#[doc = "Reader of register TCMPR"]
pub type R = crate::R<u32, super::TCMPR>;
#[doc = "Writer for register TCMPR"]
pub type W = crate::W<u32, super::TCMPR>;
#[doc = "Register TCMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCMP`"]
pub type TCMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TCMP`"]
pub struct TCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer Compared Value TCMP is a 24-bit compared register. When the internal 24-bit up-timer counts and its value is equal to TCMP value, a Timer Interrupt is requested if the timer interrupt is enabled with TCSR.IE\\[29\\]=1. The TCMP value defines the timer counting cycle time. Time out period = (Period of timer clock input) * (8-bit PRESCALE + 1) * (24-bit TCMP) NOTE1: Never write 0x0 or 0x1 in TCMP, or the core will run into unknown state. NOTE2: No matter CEN is 0 or 1, whenever software write a new value into this register, TIMER will restart counting using this new value and abort previous count."]
    #[inline(always)]
    pub fn tcmp(&self) -> TCMP_R {
        TCMP_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer Compared Value TCMP is a 24-bit compared register. When the internal 24-bit up-timer counts and its value is equal to TCMP value, a Timer Interrupt is requested if the timer interrupt is enabled with TCSR.IE\\[29\\]=1. The TCMP value defines the timer counting cycle time. Time out period = (Period of timer clock input) * (8-bit PRESCALE + 1) * (24-bit TCMP) NOTE1: Never write 0x0 or 0x1 in TCMP, or the core will run into unknown state. NOTE2: No matter CEN is 0 or 1, whenever software write a new value into this register, TIMER will restart counting using this new value and abort previous count."]
    #[inline(always)]
    pub fn tcmp(&mut self) -> TCMP_W {
        TCMP_W { w: self }
    }
}
