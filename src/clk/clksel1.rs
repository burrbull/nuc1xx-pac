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
#[doc = "Watchdog Timer clock source select (write-protection bits) These bits are protected bit, program this need to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "2: Clock source from HCLK/2048 clock"]
    HCLK_DIV2048 = 2,
    #[doc = "3: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 3,
}
impl From<WDT_S_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDT_S`"]
pub type WDT_S_R = crate::R<u8, WDT_S_A>;
impl WDT_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WDT_S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WDT_S_A::XTL12M),
            2 => Val(WDT_S_A::HCLK_DIV2048),
            3 => Val(WDT_S_A::OSC22M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == WDT_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV2048`"]
    #[inline(always)]
    pub fn is_hclk_div2048(&self) -> bool {
        *self == WDT_S_A::HCLK_DIV2048
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == WDT_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `WDT_S`"]
pub struct WDT_S_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(WDT_S_A::XTL12M)
    }
    #[doc = "Clock source from HCLK/2048 clock"]
    #[inline(always)]
    pub fn hclk_div2048(self) -> &'a mut W {
        self.variant(WDT_S_A::HCLK_DIV2048)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(WDT_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ADC clock source select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from PLL clock"]
    PLL = 1,
    #[doc = "2: Clock source from HCLK"]
    HCLK = 2,
    #[doc = "3: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 3,
}
impl From<ADC_S_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_S`"]
pub type ADC_S_R = crate::R<u8, ADC_S_A>;
impl ADC_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_S_A {
        match self.bits {
            0 => ADC_S_A::XTL12M,
            1 => ADC_S_A::PLL,
            2 => ADC_S_A::HCLK,
            3 => ADC_S_A::OSC22M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == ADC_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == ADC_S_A::PLL
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ADC_S_A::HCLK
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == ADC_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `ADC_S`"]
pub struct ADC_S_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(ADC_S_A::XTL12M)
    }
    #[doc = "Clock source from PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(ADC_S_A::PLL)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(ADC_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(ADC_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "TIMER0 clock source select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR0_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from external 32.768 kHz crystal clock"]
    XTL32K = 1,
    #[doc = "2: Clock source from HCLK"]
    HCLK = 2,
    #[doc = "3: Clock source from external trigger"]
    EXTI = 3,
    #[doc = "7: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 7,
}
impl From<TMR0_S_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR0_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR0_S`"]
pub type TMR0_S_R = crate::R<u8, TMR0_S_A>;
impl TMR0_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMR0_S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMR0_S_A::XTL12M),
            1 => Val(TMR0_S_A::XTL32K),
            2 => Val(TMR0_S_A::HCLK),
            3 => Val(TMR0_S_A::EXTI),
            7 => Val(TMR0_S_A::OSC22M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == TMR0_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `XTL32K`"]
    #[inline(always)]
    pub fn is_xtl32k(&self) -> bool {
        *self == TMR0_S_A::XTL32K
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == TMR0_S_A::HCLK
    }
    #[doc = "Checks if the value of the field is `EXTI`"]
    #[inline(always)]
    pub fn is_exti(&self) -> bool {
        *self == TMR0_S_A::EXTI
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == TMR0_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `TMR0_S`"]
pub struct TMR0_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(TMR0_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(TMR0_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMR0_S_A::HCLK)
    }
    #[doc = "Clock source from external trigger"]
    #[inline(always)]
    pub fn exti(self) -> &'a mut W {
        self.variant(TMR0_S_A::EXTI)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(TMR0_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "TIMER1 clock source select"]
pub type TMR1_S_A = TMR0_S_A;
#[doc = "Reader of field `TMR1_S`"]
pub type TMR1_S_R = crate::R<u8, TMR1_S_A>;
#[doc = "Write proxy for field `TMR1_S`"]
pub struct TMR1_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(TMR1_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(TMR1_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMR1_S_A::HCLK)
    }
    #[doc = "Clock source from external trigger"]
    #[inline(always)]
    pub fn exti(self) -> &'a mut W {
        self.variant(TMR1_S_A::EXTI)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(TMR1_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "TIMER2 clock source select"]
pub type TMR2_S_A = TMR0_S_A;
#[doc = "Reader of field `TMR2_S`"]
pub type TMR2_S_R = crate::R<u8, TMR2_S_A>;
#[doc = "Write proxy for field `TMR2_S`"]
pub struct TMR2_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(TMR2_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(TMR2_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMR2_S_A::HCLK)
    }
    #[doc = "Clock source from external trigger"]
    #[inline(always)]
    pub fn exti(self) -> &'a mut W {
        self.variant(TMR2_S_A::EXTI)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(TMR2_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "TIMER3 clock source select"]
pub type TMR3_S_A = TMR0_S_A;
#[doc = "Reader of field `TMR3_S`"]
pub type TMR3_S_R = crate::R<u8, TMR3_S_A>;
#[doc = "Write proxy for field `TMR3_S`"]
pub struct TMR3_S_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(TMR3_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(TMR3_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(TMR3_S_A::HCLK)
    }
    #[doc = "Clock source from external trigger"]
    #[inline(always)]
    pub fn exti(self) -> &'a mut W {
        self.variant(TMR3_S_A::EXTI)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(TMR3_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "UART clock source select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from PLL clock"]
    PLL = 1,
    #[doc = "3: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 3,
}
impl From<UART_S_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART_S`"]
pub type UART_S_R = crate::R<u8, UART_S_A>;
impl UART_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART_S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART_S_A::XTL12M),
            1 => Val(UART_S_A::PLL),
            3 => Val(UART_S_A::OSC22M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == UART_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == UART_S_A::PLL
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == UART_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `UART_S`"]
pub struct UART_S_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(UART_S_A::XTL12M)
    }
    #[doc = "Clock source from PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(UART_S_A::PLL)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(UART_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "CAN clock source select"]
pub type CAN_S_A = UART_S_A;
#[doc = "Reader of field `CAN_S`"]
pub type CAN_S_R = crate::R<u8, CAN_S_A>;
#[doc = "Write proxy for field `CAN_S`"]
pub struct CAN_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(CAN_S_A::XTL12M)
    }
    #[doc = "Clock source from PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CAN_S_A::PLL)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(CAN_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "PWM0 and PWM1 clock source select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM01_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from external 32.768 kHz low speed crystal clock"]
    XTL32K = 1,
    #[doc = "2: Clock source from HCLK"]
    HCLK = 2,
    #[doc = "3: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 3,
}
impl From<PWM01_S_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM01_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM01_S`"]
pub type PWM01_S_R = crate::R<u8, PWM01_S_A>;
impl PWM01_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM01_S_A {
        match self.bits {
            0 => PWM01_S_A::XTL12M,
            1 => PWM01_S_A::XTL32K,
            2 => PWM01_S_A::HCLK,
            3 => PWM01_S_A::OSC22M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == PWM01_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `XTL32K`"]
    #[inline(always)]
    pub fn is_xtl32k(&self) -> bool {
        *self == PWM01_S_A::XTL32K
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == PWM01_S_A::HCLK
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == PWM01_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `PWM01_S`"]
pub struct PWM01_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM01_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM01_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(PWM01_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz low speed crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(PWM01_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(PWM01_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(PWM01_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "PWM2 and PWM3 clock source select"]
pub type PWM23_S_A = PWM01_S_A;
#[doc = "Reader of field `PWM23_S`"]
pub type PWM23_S_R = crate::R<u8, PWM23_S_A>;
#[doc = "Write proxy for field `PWM23_S`"]
pub struct PWM23_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM23_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM23_S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(PWM23_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz low speed crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(PWM23_S_A::XTL32K)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(PWM23_S_A::HCLK)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(PWM23_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Watchdog Timer clock source select (write-protection bits) These bits are protected bit, program this need to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn wdt_s(&self) -> WDT_S_R {
        WDT_S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ADC clock source select"]
    #[inline(always)]
    pub fn adc_s(&self) -> ADC_S_R {
        ADC_S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - TIMER0 clock source select"]
    #[inline(always)]
    pub fn tmr0_s(&self) -> TMR0_S_R {
        TMR0_S_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - TIMER1 clock source select"]
    #[inline(always)]
    pub fn tmr1_s(&self) -> TMR1_S_R {
        TMR1_S_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - TIMER2 clock source select"]
    #[inline(always)]
    pub fn tmr2_s(&self) -> TMR2_S_R {
        TMR2_S_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - TIMER3 clock source select"]
    #[inline(always)]
    pub fn tmr3_s(&self) -> TMR3_S_R {
        TMR3_S_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - UART clock source select"]
    #[inline(always)]
    pub fn uart_s(&self) -> UART_S_R {
        UART_S_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - CAN clock source select"]
    #[inline(always)]
    pub fn can_s(&self) -> CAN_S_R {
        CAN_S_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PWM0 and PWM1 clock source select"]
    #[inline(always)]
    pub fn pwm01_s(&self) -> PWM01_S_R {
        PWM01_S_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PWM2 and PWM3 clock source select"]
    #[inline(always)]
    pub fn pwm23_s(&self) -> PWM23_S_R {
        PWM23_S_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Timer clock source select (write-protection bits) These bits are protected bit, program this need to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn wdt_s(&mut self) -> WDT_S_W {
        WDT_S_W { w: self }
    }
    #[doc = "Bits 2:3 - ADC clock source select"]
    #[inline(always)]
    pub fn adc_s(&mut self) -> ADC_S_W {
        ADC_S_W { w: self }
    }
    #[doc = "Bits 8:10 - TIMER0 clock source select"]
    #[inline(always)]
    pub fn tmr0_s(&mut self) -> TMR0_S_W {
        TMR0_S_W { w: self }
    }
    #[doc = "Bits 12:14 - TIMER1 clock source select"]
    #[inline(always)]
    pub fn tmr1_s(&mut self) -> TMR1_S_W {
        TMR1_S_W { w: self }
    }
    #[doc = "Bits 16:18 - TIMER2 clock source select"]
    #[inline(always)]
    pub fn tmr2_s(&mut self) -> TMR2_S_W {
        TMR2_S_W { w: self }
    }
    #[doc = "Bits 20:22 - TIMER3 clock source select"]
    #[inline(always)]
    pub fn tmr3_s(&mut self) -> TMR3_S_W {
        TMR3_S_W { w: self }
    }
    #[doc = "Bits 24:25 - UART clock source select"]
    #[inline(always)]
    pub fn uart_s(&mut self) -> UART_S_W {
        UART_S_W { w: self }
    }
    #[doc = "Bits 26:27 - CAN clock source select"]
    #[inline(always)]
    pub fn can_s(&mut self) -> CAN_S_W {
        CAN_S_W { w: self }
    }
    #[doc = "Bits 28:29 - PWM0 and PWM1 clock source select"]
    #[inline(always)]
    pub fn pwm01_s(&mut self) -> PWM01_S_W {
        PWM01_S_W { w: self }
    }
    #[doc = "Bits 30:31 - PWM2 and PWM3 clock source select"]
    #[inline(always)]
    pub fn pwm23_s(&mut self) -> PWM23_S_W {
        PWM23_S_W { w: self }
    }
}
