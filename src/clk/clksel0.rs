#[doc = "Reader of register CLKSEL0"]
pub type R = crate::R<u32, super::CLKSEL0>;
#[doc = "Writer for register CLKSEL0"]
pub type W = crate::W<u32, super::CLKSEL0>;
#[doc = "Register CLKSEL0 `reset()`'s with value 0xffff_fff8"]
impl crate::ResetValue for super::CLKSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_fff8
    }
}
#[doc = "HCLK clock source select (write-protection bits) Note: 1. Before clock switching, the related clock sources (both pre-select and new-select) must be turn on 2. The 3-bit default value is reloaded from the value of CFOSC (Config0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b. 3. These bits are protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCLK_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from external 32.768 kHz crystal clock"]
    XTL32K = 1,
    #[doc = "2: Clock source from PLL clock"]
    PLL = 2,
    #[doc = "3: Clock source from internal 10 kHz oscillator clock"]
    OSC10K = 3,
    #[doc = "7: Clock source from internal 22.1184 MHz oscillator clock"]
    OSC22M = 7,
}
impl From<HCLK_S_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLK_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HCLK_S`"]
pub type HCLK_S_R = crate::R<u8, HCLK_S_A>;
impl HCLK_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HCLK_S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HCLK_S_A::XTL12M),
            1 => Val(HCLK_S_A::XTL32K),
            2 => Val(HCLK_S_A::PLL),
            3 => Val(HCLK_S_A::OSC10K),
            7 => Val(HCLK_S_A::OSC22M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == HCLK_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `XTL32K`"]
    #[inline(always)]
    pub fn is_xtl32k(&self) -> bool {
        *self == HCLK_S_A::XTL32K
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == HCLK_S_A::PLL
    }
    #[doc = "Checks if the value of the field is `OSC10K`"]
    #[inline(always)]
    pub fn is_osc10k(&self) -> bool {
        *self == HCLK_S_A::OSC10K
    }
    #[doc = "Checks if the value of the field is `OSC22M`"]
    #[inline(always)]
    pub fn is_osc22m(&self) -> bool {
        *self == HCLK_S_A::OSC22M
    }
}
#[doc = "Write proxy for field `HCLK_S`"]
pub struct HCLK_S_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLK_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(HCLK_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(HCLK_S_A::XTL32K)
    }
    #[doc = "Clock source from PLL clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(HCLK_S_A::PLL)
    }
    #[doc = "Clock source from internal 10 kHz oscillator clock"]
    #[inline(always)]
    pub fn osc10k(self) -> &'a mut W {
        self.variant(HCLK_S_A::OSC10K)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock"]
    #[inline(always)]
    pub fn osc22m(self) -> &'a mut W {
        self.variant(HCLK_S_A::OSC22M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Cortex_M0 SysTick clock source select (write-protection bits) If SYST_CSR\\[2\\]=0, SysTick uses listed clock source below These bits are protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STCLK_S_A {
    #[doc = "0: Clock source from external 4~24 MHz crystal clock"]
    XTL12M = 0,
    #[doc = "1: Clock source from external 32.768 kHz crystal clock"]
    XTL32K = 1,
    #[doc = "2: Clock source from external 4~24 MHz crystal clock / 2"]
    XTL12M_DIV2 = 2,
    #[doc = "3: Clock source from HCLK / 2"]
    HCLK_DIV2 = 3,
    #[doc = "7: Clock source from internal 22.1184 MHz oscillator clock / 2"]
    OSC22M_DIV2 = 7,
}
impl From<STCLK_S_A> for u8 {
    #[inline(always)]
    fn from(variant: STCLK_S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STCLK_S`"]
pub type STCLK_S_R = crate::R<u8, STCLK_S_A>;
impl STCLK_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STCLK_S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STCLK_S_A::XTL12M),
            1 => Val(STCLK_S_A::XTL32K),
            2 => Val(STCLK_S_A::XTL12M_DIV2),
            3 => Val(STCLK_S_A::HCLK_DIV2),
            7 => Val(STCLK_S_A::OSC22M_DIV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTL12M`"]
    #[inline(always)]
    pub fn is_xtl12m(&self) -> bool {
        *self == STCLK_S_A::XTL12M
    }
    #[doc = "Checks if the value of the field is `XTL32K`"]
    #[inline(always)]
    pub fn is_xtl32k(&self) -> bool {
        *self == STCLK_S_A::XTL32K
    }
    #[doc = "Checks if the value of the field is `XTL12M_DIV2`"]
    #[inline(always)]
    pub fn is_xtl12m_div2(&self) -> bool {
        *self == STCLK_S_A::XTL12M_DIV2
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV2`"]
    #[inline(always)]
    pub fn is_hclk_div2(&self) -> bool {
        *self == STCLK_S_A::HCLK_DIV2
    }
    #[doc = "Checks if the value of the field is `OSC22M_DIV2`"]
    #[inline(always)]
    pub fn is_osc22m_div2(&self) -> bool {
        *self == STCLK_S_A::OSC22M_DIV2
    }
}
#[doc = "Write proxy for field `STCLK_S`"]
pub struct STCLK_S_W<'a> {
    w: &'a mut W,
}
impl<'a> STCLK_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STCLK_S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock"]
    #[inline(always)]
    pub fn xtl12m(self) -> &'a mut W {
        self.variant(STCLK_S_A::XTL12M)
    }
    #[doc = "Clock source from external 32.768 kHz crystal clock"]
    #[inline(always)]
    pub fn xtl32k(self) -> &'a mut W {
        self.variant(STCLK_S_A::XTL32K)
    }
    #[doc = "Clock source from external 4~24 MHz crystal clock / 2"]
    #[inline(always)]
    pub fn xtl12m_div2(self) -> &'a mut W {
        self.variant(STCLK_S_A::XTL12M_DIV2)
    }
    #[doc = "Clock source from HCLK / 2"]
    #[inline(always)]
    pub fn hclk_div2(self) -> &'a mut W {
        self.variant(STCLK_S_A::HCLK_DIV2)
    }
    #[doc = "Clock source from internal 22.1184 MHz oscillator clock / 2"]
    #[inline(always)]
    pub fn osc22m_div2(self) -> &'a mut W {
        self.variant(STCLK_S_A::OSC22M_DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HCLK clock source select (write-protection bits) Note: 1. Before clock switching, the related clock sources (both pre-select and new-select) must be turn on 2. The 3-bit default value is reloaded from the value of CFOSC (Config0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b. 3. These bits are protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn hclk_s(&self) -> HCLK_S_R {
        HCLK_S_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Cortex_M0 SysTick clock source select (write-protection bits) If SYST_CSR\\[2\\]=0, SysTick uses listed clock source below These bits are protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn stclk_s(&self) -> STCLK_S_R {
        STCLK_S_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HCLK clock source select (write-protection bits) Note: 1. Before clock switching, the related clock sources (both pre-select and new-select) must be turn on 2. The 3-bit default value is reloaded from the value of CFOSC (Config0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b. 3. These bits are protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn hclk_s(&mut self) -> HCLK_S_W {
        HCLK_S_W { w: self }
    }
    #[doc = "Bits 3:5 - Cortex_M0 SysTick clock source select (write-protection bits) If SYST_CSR\\[2\\]=0, SysTick uses listed clock source below These bits are protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn stclk_s(&mut self) -> STCLK_S_W {
        STCLK_S_W { w: self }
    }
}
