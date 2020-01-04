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
#[doc = "Reader of field `HCLK_S`"]
pub type HCLK_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCLK_S`"]
pub struct HCLK_S_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `STCLK_S`"]
pub type STCLK_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STCLK_S`"]
pub struct STCLK_S_W<'a> {
    w: &'a mut W,
}
impl<'a> STCLK_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HCLK clock source select (write-protection bits) Note: 1. Before clock switching, the related clock sources (both pre-select and new-select) must be turn on 2. The 3-bit default value is reloaded from the value of CFOSC (Config0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b. 3. These bits are protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 000 = Clock source from external 4~24 MHz crystal clock 001 = Clock source from external 32.768 kHz crystal clock 010 = Clock source from PLL clock 011 = Clock source from internal 10 kHz oscillator clock 111 = Clock source from internal 22.1184 MHz oscillator clock Others = reserved"]
    #[inline(always)]
    pub fn hclk_s(&self) -> HCLK_S_R {
        HCLK_S_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Cortex_M0 SysTick clock source select (write-protection bits) If SYST_CSR\\[2\\]=0, SysTick uses listed clock source below These bits are protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 000 = clock source from 4~24 MHz crystal clock 001 = Clock source from external 32.768 kHz crystal clock 010 = clock source from 12MHz crystal clock / 2 011 = clock source from HCLK / 2 1xx = Clock source from internal 22.1184 MHz oscillator clock / 2"]
    #[inline(always)]
    pub fn stclk_s(&self) -> STCLK_S_R {
        STCLK_S_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HCLK clock source select (write-protection bits) Note: 1. Before clock switching, the related clock sources (both pre-select and new-select) must be turn on 2. The 3-bit default value is reloaded from the value of CFOSC (Config0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b. 3. These bits are protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 000 = Clock source from external 4~24 MHz crystal clock 001 = Clock source from external 32.768 kHz crystal clock 010 = Clock source from PLL clock 011 = Clock source from internal 10 kHz oscillator clock 111 = Clock source from internal 22.1184 MHz oscillator clock Others = reserved"]
    #[inline(always)]
    pub fn hclk_s(&mut self) -> HCLK_S_W {
        HCLK_S_W { w: self }
    }
    #[doc = "Bits 3:5 - Cortex_M0 SysTick clock source select (write-protection bits) If SYST_CSR\\[2\\]=0, SysTick uses listed clock source below These bits are protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 000 = clock source from 4~24 MHz crystal clock 001 = Clock source from external 32.768 kHz crystal clock 010 = clock source from 12MHz crystal clock / 2 011 = clock source from HCLK / 2 1xx = Clock source from internal 22.1184 MHz oscillator clock / 2"]
    #[inline(always)]
    pub fn stclk_s(&mut self) -> STCLK_S_W {
        STCLK_S_W { w: self }
    }
}
