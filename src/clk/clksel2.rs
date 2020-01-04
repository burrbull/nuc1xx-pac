#[doc = "Reader of register CLKSEL2"]
pub type R = crate::R<u32, super::CLKSEL2>;
#[doc = "Writer for register CLKSEL2"]
pub type W = crate::W<u32, super::CLKSEL2>;
#[doc = "Register CLKSEL2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CLKSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `I2S_S`"]
pub type I2S_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_S`"]
pub struct I2S_S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FRQDIV_S`"]
pub type FRQDIV_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRQDIV_S`"]
pub struct FRQDIV_S_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQDIV_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PWM45_S`"]
pub type PWM45_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM45_S`"]
pub struct PWM45_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM45_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PWM67_S`"]
pub type PWM67_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM67_S`"]
pub struct PWM67_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM67_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2S clock source select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn i2s_s(&self) -> I2S_S_R {
        I2S_S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Clock Divider Clock Source Select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn frqdiv_s(&self) -> FRQDIV_S_R {
        FRQDIV_S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PWM4 and PWM5 Clock Source Select.(Medium Density Only) PWM4 and PWM5 used the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm45_s(&self) -> PWM45_S_R {
        PWM45_S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PWM6 and PWM7 Clock Source Select.(Medium Density Only) PWM6 and PWM7 used the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm67_s(&self) -> PWM67_S_R {
        PWM67_S_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2S clock source select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn i2s_s(&mut self) -> I2S_S_W {
        I2S_S_W { w: self }
    }
    #[doc = "Bits 2:3 - Clock Divider Clock Source Select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn frqdiv_s(&mut self) -> FRQDIV_S_W {
        FRQDIV_S_W { w: self }
    }
    #[doc = "Bits 4:5 - PWM4 and PWM5 Clock Source Select.(Medium Density Only) PWM4 and PWM5 used the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm45_s(&mut self) -> PWM45_S_W {
        PWM45_S_W { w: self }
    }
    #[doc = "Bits 6:7 - PWM6 and PWM7 Clock Source Select.(Medium Density Only) PWM6 and PWM7 used the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm67_s(&mut self) -> PWM67_S_W {
        PWM67_S_W { w: self }
    }
}
