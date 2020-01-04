#[doc = "Reader of register CAPENR"]
pub type R = crate::R<u32, super::CAPENR>;
#[doc = "Writer for register CAPENR"]
pub type W = crate::W<u32, super::CAPENR>;
#[doc = "Register CAPENR `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPENR`"]
pub type CAPENR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPENR`"]
pub struct CAPENR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPENR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Capture Input Enable Register There are four capture inputs from pad. Bit0~Bit3 are used to control each inputs enable or disable. 0 = Disable (PWMx multi-function pin input does not affect input capture function) 1 = Enable (PWMx multi-function pin input will affect its input capture function.) CAPENR Bit 3210 for PWM group A Bit xxx1 Capture channel 0 is from pin PA.12 Bit xx1x Capture channel 1 is from pin PA.13 Bit x1xx Capture channel 2 is from pin PA.14 Bit 1xxx Capture channel 3 is from pin PA.15 Bit 3210 for PWM group B Bit xxx1 Capture channel 0 is from pin PE.11 Bit xx1x Capture channel 1 is from pin PE.5 Bit x1xx Capture channel 2 is from pin PE.0 Bit 1xxx Capture channel 3 is from pin PE.1"]
    #[inline(always)]
    pub fn capenr(&self) -> CAPENR_R {
        CAPENR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Capture Input Enable Register There are four capture inputs from pad. Bit0~Bit3 are used to control each inputs enable or disable. 0 = Disable (PWMx multi-function pin input does not affect input capture function) 1 = Enable (PWMx multi-function pin input will affect its input capture function.) CAPENR Bit 3210 for PWM group A Bit xxx1 Capture channel 0 is from pin PA.12 Bit xx1x Capture channel 1 is from pin PA.13 Bit x1xx Capture channel 2 is from pin PA.14 Bit 1xxx Capture channel 3 is from pin PA.15 Bit 3210 for PWM group B Bit xxx1 Capture channel 0 is from pin PE.11 Bit xx1x Capture channel 1 is from pin PE.5 Bit x1xx Capture channel 2 is from pin PE.0 Bit 1xxx Capture channel 3 is from pin PE.1"]
    #[inline(always)]
    pub fn capenr(&mut self) -> CAPENR_W {
        CAPENR_W { w: self }
    }
}
