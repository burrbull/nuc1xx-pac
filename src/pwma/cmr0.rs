#[doc = "Reader of register CMR0"]
pub type R = crate::R<u32, super::CMR0>;
#[doc = "Writer for register CMR0"]
pub type W = crate::W<u32, super::CMR0>;
#[doc = "Register CMR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMR`"]
pub type CMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMR`"]
pub struct CMR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Comparator Register CMR determines the PWM duty. PWM frequency = PWM01_CLK/(prescale+1)*(clock divider)/(CNR+1). Duty ratio = (CMR+1)/(CNR+1). CMR >= CNR: PWM output is always high. CMR < CNR: PWM low width = (CNR-CMR) unit; PWM high width = (CMR+1) unit. CMR = 0: PWM low width = (CNR) unit; PWM high width = 1 unit (Unit : 1 PWM clock cycle) Note: Any write to CMR will take effect in next PWM cycle."]
    #[inline(always)]
    pub fn cmr(&self) -> CMR_R {
        CMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Comparator Register CMR determines the PWM duty. PWM frequency = PWM01_CLK/(prescale+1)*(clock divider)/(CNR+1). Duty ratio = (CMR+1)/(CNR+1). CMR >= CNR: PWM output is always high. CMR < CNR: PWM low width = (CNR-CMR) unit; PWM high width = (CMR+1) unit. CMR = 0: PWM low width = (CNR) unit; PWM high width = 1 unit (Unit : 1 PWM clock cycle) Note: Any write to CMR will take effect in next PWM cycle."]
    #[inline(always)]
    pub fn cmr(&mut self) -> CMR_W {
        CMR_W { w: self }
    }
}
