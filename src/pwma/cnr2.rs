#[doc = "Reader of register CNR2"]
pub type R = crate::R<u32, super::CNR2>;
#[doc = "Writer for register CNR2"]
pub type W = crate::W<u32, super::CNR2>;
#[doc = "Register CNR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CNR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNR`"]
pub type CNR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNR`"]
pub struct CNR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Counter/Timer Loaded Value CNR determines the PWM period. PWM frequency = PWM23_CLK/(prescale+1)*(clock divider)/(CNR+1). Duty ratio = (CMR+1)/(CNR+1). CMR >= CNR: PWM output is always high. CMR < CNR: PWM low width = (CNR-CMR) unit; PWM high width = (CMR+1) unit. CMR = 0: PWM low width = (CNR) unit; PWM high width = 1 unit (Unit : 1 PWM clock cycle) Note: Any write to CNR will take effect in next PWM cycle."]
    #[inline(always)]
    pub fn cnr(&self) -> CNR_R {
        CNR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Counter/Timer Loaded Value CNR determines the PWM period. PWM frequency = PWM23_CLK/(prescale+1)*(clock divider)/(CNR+1). Duty ratio = (CMR+1)/(CNR+1). CMR >= CNR: PWM output is always high. CMR < CNR: PWM low width = (CNR-CMR) unit; PWM high width = (CMR+1) unit. CMR = 0: PWM low width = (CNR) unit; PWM high width = 1 unit (Unit : 1 PWM clock cycle) Note: Any write to CNR will take effect in next PWM cycle."]
    #[inline(always)]
    pub fn cnr(&mut self) -> CNR_W {
        CNR_W { w: self }
    }
}
