#[doc = "Reader of register SPI_DIVIDER"]
pub type R = crate::R<u32, super::SPI_DIVIDER>;
#[doc = "Writer for register SPI_DIVIDER"]
pub type W = crate::W<u32, super::SPI_DIVIDER>;
#[doc = "Register SPI_DIVIDER `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DIVIDER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVIDER`"]
pub type DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVIDER`"]
pub struct DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DIVIDER2`"]
pub type DIVIDER2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVIDER2`"]
pub struct DIVIDER2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divider Register (master only) The value in this field is the frequency divider of the system clock, PCLK, to generate the serial clock on the output SPICLK. The desired frequency is obtained according to the following equation: fsclk = fpclk / ((DIVIDER+1)*2) In slave mode, the period of SPI clock driven by a master shall equal or over 5 times the period of PCLK. In other words, the maximum frequency of SPI clock is the fifth of the frequency of slave's PCLK."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock Divider 2 Register (master only) The value in this field is the 2nd frequency divider of the system clock, PCLK, to generate the serial clock on the output SPICLK. The desired frequency is obtained according to the following equation: fsclk = fpclk / ((DIVIDER2+1)*2)"]
    #[inline(always)]
    pub fn divider2(&self) -> DIVIDER2_R {
        DIVIDER2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider Register (master only) The value in this field is the frequency divider of the system clock, PCLK, to generate the serial clock on the output SPICLK. The desired frequency is obtained according to the following equation: fsclk = fpclk / ((DIVIDER+1)*2) In slave mode, the period of SPI clock driven by a master shall equal or over 5 times the period of PCLK. In other words, the maximum frequency of SPI clock is the fifth of the frequency of slave's PCLK."]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W {
        DIVIDER_W { w: self }
    }
    #[doc = "Bits 16:31 - Clock Divider 2 Register (master only) The value in this field is the 2nd frequency divider of the system clock, PCLK, to generate the serial clock on the output SPICLK. The desired frequency is obtained according to the following equation: fsclk = fpclk / ((DIVIDER2+1)*2)"]
    #[inline(always)]
    pub fn divider2(&mut self) -> DIVIDER2_W {
        DIVIDER2_W { w: self }
    }
}
