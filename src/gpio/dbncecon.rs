#[doc = "Reader of register DBNCECON"]
pub type R = crate::R<u32, super::DBNCECON>;
#[doc = "Writer for register DBNCECON"]
pub type W = crate::W<u32, super::DBNCECON>;
#[doc = "Register DBNCECON `reset()`'s with value 0x20"]
impl crate::ResetValue for super::DBNCECON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `DBCLKSEL`"]
pub type DBCLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBCLKSEL`"]
pub struct DBCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DBCLKSRC`"]
pub type DBCLKSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBCLKSRC`"]
pub struct DBCLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCLKSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ICLK_ON`"]
pub type ICLK_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICLK_ON`"]
pub struct ICLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ICLK_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - De-bounce sampling cycle selection DBCLKSEL Description 0 Sample interrupt input once per 1 clocks 1 Sample interrupt input once per 2 clocks 2 Sample interrupt input once per 4 clocks 3 Sample interrupt input once per 8 clocks 4 Sample interrupt input once per 16 clocks 5 Sample interrupt input once per 32 clocks 6 Sample interrupt input once per 64 clocks 7 Sample interrupt input once per 128 clocks 8 Sample interrupt input once per 256 clocks 9 Sample interrupt input once per 2*256 clocks 10 Sample interrupt input once per 4*256clocks 11 Sample interrupt input once per 8*256 clocks 12 Sample interrupt input once per 16*256 clocks 13 Sample interrupt input once per 32*256 clocks 14 Sample interrupt input once per 64*256 clocks 15 Sample interrupt input once per 128*256 clocks"]
    #[inline(always)]
    pub fn dbclksel(&self) -> DBCLKSEL_R {
        DBCLKSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - De-bounce counter clock source select 1 = De-bounce counter clock source is the internal 10 KHz clock 0 = De-bounce counter clock source is the HCLK"]
    #[inline(always)]
    pub fn dbclksrc(&self) -> DBCLKSRC_R {
        DBCLKSRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt clock On mode Set this bit to 0 will disable the interrupt generate circuit clock, if the pin\\[n\\]
interrupt is disabled 1 = Interrupt generated circuit clock always enable 0 = Disable the clock if the GPIOA/B/C/D/E\\[n\\]
interrupt is disabled"]
    #[inline(always)]
    pub fn iclk_on(&self) -> ICLK_ON_R {
        ICLK_ON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - De-bounce sampling cycle selection DBCLKSEL Description 0 Sample interrupt input once per 1 clocks 1 Sample interrupt input once per 2 clocks 2 Sample interrupt input once per 4 clocks 3 Sample interrupt input once per 8 clocks 4 Sample interrupt input once per 16 clocks 5 Sample interrupt input once per 32 clocks 6 Sample interrupt input once per 64 clocks 7 Sample interrupt input once per 128 clocks 8 Sample interrupt input once per 256 clocks 9 Sample interrupt input once per 2*256 clocks 10 Sample interrupt input once per 4*256clocks 11 Sample interrupt input once per 8*256 clocks 12 Sample interrupt input once per 16*256 clocks 13 Sample interrupt input once per 32*256 clocks 14 Sample interrupt input once per 64*256 clocks 15 Sample interrupt input once per 128*256 clocks"]
    #[inline(always)]
    pub fn dbclksel(&mut self) -> DBCLKSEL_W {
        DBCLKSEL_W { w: self }
    }
    #[doc = "Bit 4 - De-bounce counter clock source select 1 = De-bounce counter clock source is the internal 10 KHz clock 0 = De-bounce counter clock source is the HCLK"]
    #[inline(always)]
    pub fn dbclksrc(&mut self) -> DBCLKSRC_W {
        DBCLKSRC_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt clock On mode Set this bit to 0 will disable the interrupt generate circuit clock, if the pin\\[n\\]
interrupt is disabled 1 = Interrupt generated circuit clock always enable 0 = Disable the clock if the GPIOA/B/C/D/E\\[n\\]
interrupt is disabled"]
    #[inline(always)]
    pub fn iclk_on(&mut self) -> ICLK_ON_W {
        ICLK_ON_W { w: self }
    }
}
