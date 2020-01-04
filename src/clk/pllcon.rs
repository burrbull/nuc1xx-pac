#[doc = "Reader of register PLLCON"]
pub type R = crate::R<u32, super::PLLCON>;
#[doc = "Writer for register PLLCON"]
pub type W = crate::W<u32, super::PLLCON>;
#[doc = "Register PLLCON `reset()`'s with value 0x0005_c22e"]
impl crate::ResetValue for super::PLLCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0005_c22e
    }
}
#[doc = "Reader of field `FB_DV`"]
pub type FB_DV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FB_DV`"]
pub struct FB_DV_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_DV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `IN_DV`"]
pub type IN_DV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN_DV`"]
pub struct IN_DV_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
#[doc = "Reader of field `OUT_DV`"]
pub type OUT_DV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_DV`"]
pub struct OUT_DV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BP`"]
pub type BP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BP`"]
pub struct BP_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE`"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PLL_SRC`"]
pub type PLL_SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_SRC`"]
pub struct PLL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PLL Feedback Divider Control Pins Refer to the formulas below the table. FOUT = FIN x NF/NR x 1/NO Constrain: 1. 3.2MHz < FIN < 150MHz 2. 800KHz < FIN/(2xNR) < 8MHz 3. 100MHz < FCO = FINxNF/NR < 200MHz , 120M < FCO is preferred. Symbol Description FOUT Output Clock Frequency FIN Input (Reference) Clock Frequency NR Input Divider (IN_DV + 2) NF Feedback Divider (FB_DV + 2) NO OUT_DV = \"00\":NO = 1 OUT_DV = \"01\":NO = 2 OUT_DV = \"10\":NO = 2 OUT_DV = \"11\":NO = 4"]
    #[inline(always)]
    pub fn fb_dv(&self) -> FB_DV_R {
        FB_DV_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - PLL Input Divider Control Pins Refer to the formulas below the table. (Table is the same as FB_DV)."]
    #[inline(always)]
    pub fn in_dv(&self) -> IN_DV_R {
        IN_DV_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - PLL Output Divider Control Pins Refer to the formulas below the table. (Table is the same as FB_DV)."]
    #[inline(always)]
    pub fn out_dv(&self) -> OUT_DV_R {
        OUT_DV_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Power Down Mode. If set the IDLE bit \"1\" in PWRCON register, the PLL will enter power down mode too 0 = PLL is in normal mode 1 = PLL is in power-down mode(default)"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PLL Bypass Control 0 = PLL is in normal mode (default) 1 = PLL clock output is same as clock input (XTALin)"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLL OE (FOUT enable) pin Control 0 = PLL FOUT enable 1 = PLL FOUT is fixed low"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PLL Source Clock Select 1 = PLL source clock from 22.1184 MHz oscillator 0 = PLL source clock from 4~24 MHz crystal"]
    #[inline(always)]
    pub fn pll_src(&self) -> PLL_SRC_R {
        PLL_SRC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - PLL Feedback Divider Control Pins Refer to the formulas below the table. FOUT = FIN x NF/NR x 1/NO Constrain: 1. 3.2MHz < FIN < 150MHz 2. 800KHz < FIN/(2xNR) < 8MHz 3. 100MHz < FCO = FINxNF/NR < 200MHz , 120M < FCO is preferred. Symbol Description FOUT Output Clock Frequency FIN Input (Reference) Clock Frequency NR Input Divider (IN_DV + 2) NF Feedback Divider (FB_DV + 2) NO OUT_DV = \"00\":NO = 1 OUT_DV = \"01\":NO = 2 OUT_DV = \"10\":NO = 2 OUT_DV = \"11\":NO = 4"]
    #[inline(always)]
    pub fn fb_dv(&mut self) -> FB_DV_W {
        FB_DV_W { w: self }
    }
    #[doc = "Bits 9:13 - PLL Input Divider Control Pins Refer to the formulas below the table. (Table is the same as FB_DV)."]
    #[inline(always)]
    pub fn in_dv(&mut self) -> IN_DV_W {
        IN_DV_W { w: self }
    }
    #[doc = "Bits 14:15 - PLL Output Divider Control Pins Refer to the formulas below the table. (Table is the same as FB_DV)."]
    #[inline(always)]
    pub fn out_dv(&mut self) -> OUT_DV_W {
        OUT_DV_W { w: self }
    }
    #[doc = "Bit 16 - Power Down Mode. If set the IDLE bit \"1\" in PWRCON register, the PLL will enter power down mode too 0 = PLL is in normal mode 1 = PLL is in power-down mode(default)"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 17 - PLL Bypass Control 0 = PLL is in normal mode (default) 1 = PLL clock output is same as clock input (XTALin)"]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W {
        BP_W { w: self }
    }
    #[doc = "Bit 18 - PLL OE (FOUT enable) pin Control 0 = PLL FOUT enable 1 = PLL FOUT is fixed low"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 19 - PLL Source Clock Select 1 = PLL source clock from 22.1184 MHz oscillator 0 = PLL source clock from 4~24 MHz crystal"]
    #[inline(always)]
    pub fn pll_src(&mut self) -> PLL_SRC_W {
        PLL_SRC_W { w: self }
    }
}
