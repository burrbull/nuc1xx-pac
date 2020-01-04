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
#[doc = "I2S clock source select"]
pub type I2S_S_A = super::clksel1::ADC_S_A;
#[doc = "Reader of field `I2S_S`"]
pub type I2S_S_R = crate::R<u8, I2S_S_A>;
#[doc = "Write proxy for field `I2S_S`"]
pub struct I2S_S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(I2S_S_A::XTL12M)
    }
    #[doc = "Clock source from PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(I2S_S_A::PLL)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(I2S_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(I2S_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Clock Divider Clock Source Select"]
pub type FRQDIV_S_A = super::clksel1::PWM01_S_A;
#[doc = "Reader of field `FRQDIV_S`"]
pub type FRQDIV_S_R = crate::R<u8, FRQDIV_S_A>;
#[doc = "Write proxy for field `FRQDIV_S`"]
pub struct FRQDIV_S_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQDIV_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRQDIV_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(FRQDIV_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz low speed crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(FRQDIV_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(FRQDIV_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(FRQDIV_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "PWM4 and PWM5 Clock Source Select.(Medium Density Only)"]
pub type PWM45_S_A = super::clksel1::PWM01_S_A;
#[doc = "Reader of field `PWM45_S`"]
pub type PWM45_S_R = crate::R<u8, PWM45_S_A>;
#[doc = "Write proxy for field `PWM45_S`"]
pub struct PWM45_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM45_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM45_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(PWM45_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz low speed crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(PWM45_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(PWM45_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(PWM45_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "PWM6 and PWM7 Clock Source Select.(Medium Density Only)"]
pub type PWM67_S_A = super::clksel1::PWM01_S_A;
#[doc = "Reader of field `PWM67_S`"]
pub type PWM67_S_R = crate::R<u8, PWM67_S_A>;
#[doc = "Write proxy for field `PWM67_S`"]
pub struct PWM67_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM67_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM67_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(PWM67_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz low speed crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(PWM67_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(PWM67_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(PWM67_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2S clock source select"]
    #[inline(always)]
    pub fn i2s_s(&self) -> I2S_S_R {
        I2S_S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Clock Divider Clock Source Select"]
    #[inline(always)]
    pub fn frqdiv_s(&self) -> FRQDIV_S_R {
        FRQDIV_S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PWM4 and PWM5 Clock Source Select.(Medium Density Only)"]
    #[inline(always)]
    pub fn pwm45_s(&self) -> PWM45_S_R {
        PWM45_S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PWM6 and PWM7 Clock Source Select.(Medium Density Only)"]
    #[inline(always)]
    pub fn pwm67_s(&self) -> PWM67_S_R {
        PWM67_S_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2S clock source select"]
    #[inline(always)]
    pub fn i2s_s(&mut self) -> I2S_S_W {
        I2S_S_W { w: self }
    }
    #[doc = "Bits 2:3 - Clock Divider Clock Source Select"]
    #[inline(always)]
    pub fn frqdiv_s(&mut self) -> FRQDIV_S_W {
        FRQDIV_S_W { w: self }
    }
    #[doc = "Bits 4:5 - PWM4 and PWM5 Clock Source Select.(Medium Density Only)"]
    #[inline(always)]
    pub fn pwm45_s(&mut self) -> PWM45_S_W {
        PWM45_S_W { w: self }
    }
    #[doc = "Bits 6:7 - PWM6 and PWM7 Clock Source Select.(Medium Density Only)"]
    #[inline(always)]
    pub fn pwm67_s(&mut self) -> PWM67_S_W {
        PWM67_S_W { w: self }
    }
}
