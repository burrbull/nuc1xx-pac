#[doc = "Reader of register I2S_CLKDIV"]
pub type R = crate::R<u32, super::I2S_CLKDIV>;
#[doc = "Writer for register I2S_CLKDIV"]
pub type W = crate::W<u32, super::I2S_CLKDIV>;
#[doc = "Register I2S_CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCLK_DIV`"]
pub type MCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCLK_DIV`"]
pub struct MCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BCLK_DIV`"]
pub type BCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCLK_DIV`"]
pub struct BCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Divider If chip external crystal frequency is (2xMCLK_DIV)*256fs then software can program these bits to generate 256fs clock frequency to audio codec chip. If MCLK_DIV is set to 0, MCLK is the same as external clock input. For example, sampling rate is 24 kHz and chip external crystal clock is 12.288 MHz, set MCLK_DIV=1. F_MCLK = F_I2SCLK/(2x(MCLK_DIV)) (When MCLK_DIV is >= 1 ) F_MCLK = F_I2SCLK (When MCLK_DIV is set to 0 )"]
    #[inline(always)]
    pub fn mclk_div(&self) -> MCLK_DIV_R {
        MCLK_DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Bit Clock Divider If I2S operates in master mode, bit clock is provided by NuMicro(TM) NUC100 series. Software can program these bits to generate sampling rate clock frequency. F_BCLK = F_I2SCLK /(2x(BCLK_DIV + 1))"]
    #[inline(always)]
    pub fn bclk_div(&self) -> BCLK_DIV_R {
        BCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Divider If chip external crystal frequency is (2xMCLK_DIV)*256fs then software can program these bits to generate 256fs clock frequency to audio codec chip. If MCLK_DIV is set to 0, MCLK is the same as external clock input. For example, sampling rate is 24 kHz and chip external crystal clock is 12.288 MHz, set MCLK_DIV=1. F_MCLK = F_I2SCLK/(2x(MCLK_DIV)) (When MCLK_DIV is >= 1 ) F_MCLK = F_I2SCLK (When MCLK_DIV is set to 0 )"]
    #[inline(always)]
    pub fn mclk_div(&mut self) -> MCLK_DIV_W {
        MCLK_DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Bit Clock Divider If I2S operates in master mode, bit clock is provided by NuMicro(TM) NUC100 series. Software can program these bits to generate sampling rate clock frequency. F_BCLK = F_I2SCLK /(2x(BCLK_DIV + 1))"]
    #[inline(always)]
    pub fn bclk_div(&mut self) -> BCLK_DIV_W {
        BCLK_DIV_W { w: self }
    }
}
