#[doc = "Reader of register CLKSTATUS"]
pub type R = crate::R<u32, super::CLKSTATUS>;
#[doc = "Writer for register CLKSTATUS"]
pub type W = crate::W<u32, super::CLKSTATUS>;
#[doc = "Register CLKSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTL12M_STB`"]
pub type XTL12M_STB_R = crate::R<bool, bool>;
#[doc = "Reader of field `XTL32K_STB`"]
pub type XTL32K_STB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL_STB`"]
pub type PLL_STB_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC10K_STB`"]
pub type OSC10K_STB_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC22M_STB`"]
pub type OSC22M_STB_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_SW_FAIL`"]
pub type CLK_SW_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_SW_FAIL`"]
pub struct CLK_SW_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SW_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTL12M clock source stable flag 1 = XTL12M clock is stable 0 = XTL12M clock is not stable or disabled This is read only bit"]
    #[inline(always)]
    pub fn xtl12m_stb(&self) -> XTL12M_STB_R {
        XTL12M_STB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XTL32K clock source stable flag 1 = XTL32K clock is stable 0 = XTL32K clock is not stable or disabled This is read only bit"]
    #[inline(always)]
    pub fn xtl32k_stb(&self) -> XTL32K_STB_R {
        XTL32K_STB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLL clock source stable flag 1 = PLL clock is stable 0 = PLL clock is not stable or disabled This is read only bit"]
    #[inline(always)]
    pub fn pll_stb(&self) -> PLL_STB_R {
        PLL_STB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OSC10K clock source stable flag 1 = OSC10K clock is stable 0 = OSC10K clock is not stable or disabled This is read only bit"]
    #[inline(always)]
    pub fn osc10k_stb(&self) -> OSC10K_STB_R {
        OSC10K_STB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC22M clock source stable flag 1 = OSC22M clock is stable 0 = OSC22M clock is not stable or disabled This is read only bit"]
    #[inline(always)]
    pub fn osc22m_stb(&self) -> OSC22M_STB_R {
        OSC22M_STB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock switching fail flag 1 = Clock switching failure 0 = Clock switching success This bit is updated when software switches system clock source. If switch target clock is stable, this bit will be set to 1'b0. If switch target clock is not stable, this bit will be set to 1'b1. Write 1 to clear the bit to zero"]
    #[inline(always)]
    pub fn clk_sw_fail(&self) -> CLK_SW_FAIL_R {
        CLK_SW_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Clock switching fail flag 1 = Clock switching failure 0 = Clock switching success This bit is updated when software switches system clock source. If switch target clock is stable, this bit will be set to 1'b0. If switch target clock is not stable, this bit will be set to 1'b1. Write 1 to clear the bit to zero"]
    #[inline(always)]
    pub fn clk_sw_fail(&mut self) -> CLK_SW_FAIL_W {
        CLK_SW_FAIL_W { w: self }
    }
}
