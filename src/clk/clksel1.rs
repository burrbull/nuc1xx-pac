#[doc = "Reader of register CLKSEL1"]
pub type R = crate::R<u32, super::CLKSEL1>;
#[doc = "Writer for register CLKSEL1"]
pub type W = crate::W<u32, super::CLKSEL1>;
#[doc = "Register CLKSEL1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CLKSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WDT_S`"]
pub type WDT_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDT_S`"]
pub struct WDT_S_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADC_S`"]
pub type ADC_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_S`"]
pub struct ADC_S_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TMR0_S`"]
pub type TMR0_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMR0_S`"]
pub struct TMR0_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `TMR1_S`"]
pub type TMR1_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMR1_S`"]
pub struct TMR1_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `TMR2_S`"]
pub type TMR2_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMR2_S`"]
pub struct TMR2_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TMR3_S`"]
pub type TMR3_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMR3_S`"]
pub struct TMR3_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_S`"]
pub type UART_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_S`"]
pub struct UART_S_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CAN_S`"]
pub type CAN_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAN_S`"]
pub struct CAN_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PWM01_S`"]
pub type PWM01_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM01_S`"]
pub struct PWM01_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM01_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `PWM23_S`"]
pub type PWM23_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM23_S`"]
pub struct PWM23_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM23_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Watchdog Timer clock source select (write-protection bits) These bits are protected bit, program this need to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Reserved 10 = Clock source from HCLK/2048 clock. 11 = Clock source from internal 10 kHz oscillator clock."]
    #[inline(always)]
    pub fn wdt_s(&self) -> WDT_S_R {
        WDT_S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ADC clock source select 00 = Clock source from external 4~24 MHz crystal clock. 01 = clock source from PLL clock 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn adc_s(&self) -> ADC_S_R {
        ADC_S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - TIMER0 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr0_s(&self) -> TMR0_S_R {
        TMR0_S_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - TIMER1 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr1_s(&self) -> TMR1_S_R {
        TMR1_S_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - TIMER2 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr2_s(&self) -> TMR2_S_R {
        TMR2_S_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - TIMER3 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr3_s(&self) -> TMR3_S_R {
        TMR3_S_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - UART clock source select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn uart_s(&self) -> UART_S_R {
        UART_S_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - CAN clock source select 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn can_s(&self) -> CAN_S_R {
        CAN_S_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PWM0 and PWM1 clock source select PWM0 and PWM1 uses the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm01_s(&self) -> PWM01_S_R {
        PWM01_S_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PWM2 and PWM3 clock source select PWM2 and PWM3 uses the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm23_s(&self) -> PWM23_S_R {
        PWM23_S_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Timer clock source select (write-protection bits) These bits are protected bit, program this need to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Reserved 10 = Clock source from HCLK/2048 clock. 11 = Clock source from internal 10 kHz oscillator clock."]
    #[inline(always)]
    pub fn wdt_s(&mut self) -> WDT_S_W {
        WDT_S_W { w: self }
    }
    #[doc = "Bits 2:3 - ADC clock source select 00 = Clock source from external 4~24 MHz crystal clock. 01 = clock source from PLL clock 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn adc_s(&mut self) -> ADC_S_W {
        ADC_S_W { w: self }
    }
    #[doc = "Bits 8:10 - TIMER0 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr0_s(&mut self) -> TMR0_S_W {
        TMR0_S_W { w: self }
    }
    #[doc = "Bits 12:14 - TIMER1 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr1_s(&mut self) -> TMR1_S_W {
        TMR1_S_W { w: self }
    }
    #[doc = "Bits 16:18 - TIMER2 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr2_s(&mut self) -> TMR2_S_W {
        TMR2_S_W { w: self }
    }
    #[doc = "Bits 20:22 - TIMER3 clock source select. 000 = Clock source from external 4~24 MHz crystal clock. 001 = Clock source from external 32.768 kHz crystal clock. 010 = Clock source from HCLK. 011 = Clock source from external trigger. 1xx = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn tmr3_s(&mut self) -> TMR3_S_W {
        TMR3_S_W { w: self }
    }
    #[doc = "Bits 24:25 - UART clock source select. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn uart_s(&mut self) -> UART_S_W {
        UART_S_W { w: self }
    }
    #[doc = "Bits 26:27 - CAN clock source select 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from PLL clock. 1x = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn can_s(&mut self) -> CAN_S_W {
        CAN_S_W { w: self }
    }
    #[doc = "Bits 28:29 - PWM0 and PWM1 clock source select PWM0 and PWM1 uses the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm01_s(&mut self) -> PWM01_S_W {
        PWM01_S_W { w: self }
    }
    #[doc = "Bits 30:31 - PWM2 and PWM3 clock source select PWM2 and PWM3 uses the same Engine clock source, both of them use the same prescaler. 00 = Clock source from external 4~24 MHz crystal clock. 01 = Clock source from external 32.768 kHz crystal clock. 10 = Clock source from HCLK. 11 = Clock source from internal 22.1184 MHz oscillator clock."]
    #[inline(always)]
    pub fn pwm23_s(&mut self) -> PWM23_S_W {
        PWM23_S_W { w: self }
    }
}
