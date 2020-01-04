#[doc = "Reader of register GPC_MFP"]
pub type R = crate::R<u32, super::GPC_MFP>;
#[doc = "Writer for register GPC_MFP"]
pub type W = crate::W<u32, super::GPC_MFP>;
#[doc = "Register GPC_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPC_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI0_SS0_I2SLRCLK`"]
pub type SPI0_SS0_I2SLRCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_SS0_I2SLRCLK`"]
pub struct SPI0_SS0_I2SLRCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SS0_I2SLRCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SPI0_CLK_I2SBCLK`"]
pub type SPI0_CLK_I2SBCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_CLK_I2SBCLK`"]
pub struct SPI0_CLK_I2SBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CLK_I2SBCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SPI0_MISO0_I2SDI`"]
pub type SPI0_MISO0_I2SDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_MISO0_I2SDI`"]
pub struct SPI0_MISO0_I2SDI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MISO0_I2SDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI0_MOSI0_I2SDO`"]
pub type SPI0_MOSI0_I2SDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_MOSI0_I2SDO`"]
pub struct SPI0_MOSI0_I2SDO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MOSI0_I2SDO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPI0_MISO1`"]
pub type SPI0_MISO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_MISO1`"]
pub struct SPI0_MISO1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MISO1_W<'a> {
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
#[doc = "Reader of field `SPI0_MOSI1`"]
pub type SPI0_MOSI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_MOSI1`"]
pub struct SPI0_MOSI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MOSI1_W<'a> {
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
#[doc = "Reader of field `CPP0_AD4`"]
pub type CPP0_AD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPP0_AD4`"]
pub struct CPP0_AD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CPP0_AD4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CPN0_AD5`"]
pub type CPN0_AD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPN0_AD5`"]
pub struct CPN0_AD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CPN0_AD5_W<'a> {
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
#[doc = "Reader of field `SPI1_SS0_MCLK`"]
pub type SPI1_SS0_MCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_SS0_MCLK`"]
pub struct SPI1_SS0_MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SS0_MCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI1_CLK`"]
pub type SPI1_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_CLK`"]
pub struct SPI1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPI1_MISO0`"]
pub type SPI1_MISO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_MISO0`"]
pub struct SPI1_MISO0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MISO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI1_MOSI0`"]
pub type SPI1_MOSI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_MOSI0`"]
pub struct SPI1_MOSI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MOSI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI1_MISO1`"]
pub type SPI1_MISO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_MISO1`"]
pub struct SPI1_MISO1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MISO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI1_MOSI1`"]
pub type SPI1_MOSI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_MOSI1`"]
pub struct SPI1_MOSI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MOSI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CPP1_AD2`"]
pub type CPP1_AD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPP1_AD2`"]
pub struct CPP1_AD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPP1_AD2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CPP1_AD3`"]
pub type CPP1_AD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPP1_AD3`"]
pub struct CPP1_AD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CPP1_AD3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SCHMITT`"]
pub type SCHMITT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCHMITT`"]
pub struct SCHMITT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PC.0 Pin Function Selection Bits PC0_I2SLRCLK (ALT_MFP\\[5\\]) and GPC_MFP\\[0\\]
determine the PC.0 function. PC0_I2SLRCLK GPC_MFP\\[0\\]
PC.0 function x 0 GPIO 0 1 SPISS00(SPI0) 1 1 I2SLRCLK (I2S)"]
    #[inline(always)]
    pub fn spi0_ss0_i2slrclk(&self) -> SPI0_SS0_I2SLRCLK_R {
        SPI0_SS0_I2SLRCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PC.1 Pin Function Selection Bits PC1_I2SBCLK (ALT_MFP\\[6\\]) and GPC_MFP\\[1\\]
determine the PC.1 function. PC1_I2SBCLK GPC_MFP\\[1\\]
PC.1 function x 0 GPIO 0 1 SPICLK0 (SPI0) 1 1 I2SBLK (I2S)"]
    #[inline(always)]
    pub fn spi0_clk_i2sbclk(&self) -> SPI0_CLK_I2SBCLK_R {
        SPI0_CLK_I2SBCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PC.2 Pin Function Selection Bits PC2_I2SDI (ALT_MFP\\[7\\]) and GPC_MFP\\[2\\]
determine the PC.2 function. PC2_I2SDI GPC_MFP\\[2\\]
PC.2 function x 0 GPIO 0 1 MISO00 (SPI0) 1 1 I2SDI (I2S)"]
    #[inline(always)]
    pub fn spi0_miso0_i2sdi(&self) -> SPI0_MISO0_I2SDI_R {
        SPI0_MISO0_I2SDI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PC.3 Pin Function Selection Bits PC3_I2SDO (ALT_MFP\\[8\\]) and GPC_MFP\\[3\\]
determine the PC.3 function. PC3_I2SDO GPC_MFP\\[3\\]
PC.3 function x 0 GPIO 0 1 MOSI00 (SPI0) 1 1 I2SDO (I2S)"]
    #[inline(always)]
    pub fn spi0_mosi0_i2sdo(&self) -> SPI0_MOSI0_I2SDO_R {
        SPI0_MOSI0_I2SDO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PC.4 Pin Function Selection 1 = The SPI0 MISO1 (master input, slave output pin-1) function is selected to the pin PC.4 0 = The GPIOC\\[4\\]
is selected to the pin PC.4"]
    #[inline(always)]
    pub fn spi0_miso1(&self) -> SPI0_MISO1_R {
        SPI0_MISO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PC.5 Pin Function Selection 1 = The SPI0 MOSI1 (master output, slave input pin-1) function is selected to the pin PC.5 0 = The GPIOC\\[5\\]
is selected to the pin PC.5"]
    #[inline(always)]
    pub fn spi0_mosi1(&self) -> SPI0_MOSI1_R {
        SPI0_MOSI1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PC.6 Pin Function Selection The pin function depends on GPC_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[6\\]
PC.6 function x 0 GPIO 0 1 CPP0 (CMP) 1 1 AD4 (EBI AD bus bit 4)"]
    #[inline(always)]
    pub fn cpp0_ad4(&self) -> CPP0_AD4_R {
        CPP0_AD4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PC.7 Pin Function Selection The pin function depends on GPC_MFP7 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[7\\]
PC.7 function x 0 GPIO 0 1 CPN0 (CMP) 1 1 AD5 (EBI AD bus bit 5)"]
    #[inline(always)]
    pub fn cpn0_ad5(&self) -> CPN0_AD5_R {
        CPN0_AD5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PC.8 Pin Function Selection The pin function depends on GPC_MFP8 and EBI_MCLK_EN (ALT_MFP\\[12\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_MCLK_EN EBI_EN GPC_MFP\\[8\\]
PC.8 function x x 0 GPIO x 0 1 SPISS10 (SPI1) 0 1 1 SPISS10 (SPI1) 1 1 1 MCLK (EBI Clock output)"]
    #[inline(always)]
    pub fn spi1_ss0_mclk(&self) -> SPI1_SS0_MCLK_R {
        SPI1_SS0_MCLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PC.9 Pin Function Selection 1 = The SPI1 SPICLK function is selected to the pin PC.9 0 = The GPIOC\\[9\\]
is selected to the pin PC.9"]
    #[inline(always)]
    pub fn spi1_clk(&self) -> SPI1_CLK_R {
        SPI1_CLK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PC.10 Pin Function Selection 1 = The SPI1 MISO0 (master input, slave output pin-0) function is selected to the pin PC.10 0 = The GPIOC\\[10\\]
is selected to the pin PC.10"]
    #[inline(always)]
    pub fn spi1_miso0(&self) -> SPI1_MISO0_R {
        SPI1_MISO0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PC.11 Pin Function Selection 1 = The SPI1 MOSI0 (master output, slave input pin-0) function is selected to the pin PC.11 0 = The GPIOC\\[11\\]
is selected to the pin PC.11"]
    #[inline(always)]
    pub fn spi1_mosi0(&self) -> SPI1_MOSI0_R {
        SPI1_MOSI0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PC.12 Pin Function Selection 1 = The SPI1 MISO1 (master input, slave output pin-1) function is selected to the pin PC.12 0 = The GPIOC\\[12\\]
is selected to the pin PC.12"]
    #[inline(always)]
    pub fn spi1_miso1(&self) -> SPI1_MISO1_R {
        SPI1_MISO1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PC.13 Pin Function Selection 1 = The SPI1 MOSI1 (master output, slave input pin-1) function is selected to the pin PC.13 0 = The GPIOC\\[13\\]
is selected to the pin PC.13"]
    #[inline(always)]
    pub fn spi1_mosi1(&self) -> SPI1_MOSI1_R {
        SPI1_MOSI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PC.14 Pin Function Selection The pin function depends on GPC_MFP14 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[14\\]
PC.14 function x 0 GPIO 0 1 CPP1 (CMP) 1 1 AD2 (EBI AD bus bit 2)"]
    #[inline(always)]
    pub fn cpp1_ad2(&self) -> CPP1_AD2_R {
        CPP1_AD2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PC.15 Pin Function Selection The pin function depends on GPC_MFP15 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[15\\]
PC.15 function x 0 GPIO 0 1 CPN1 (CMP) 1 1 AD3 (EBI AD bus bit 3)"]
    #[inline(always)]
    pub fn cpp1_ad3(&self) -> CPP1_AD3_R {
        CPP1_AD3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOC\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOC\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PC.0 Pin Function Selection Bits PC0_I2SLRCLK (ALT_MFP\\[5\\]) and GPC_MFP\\[0\\]
determine the PC.0 function. PC0_I2SLRCLK GPC_MFP\\[0\\]
PC.0 function x 0 GPIO 0 1 SPISS00(SPI0) 1 1 I2SLRCLK (I2S)"]
    #[inline(always)]
    pub fn spi0_ss0_i2slrclk(&mut self) -> SPI0_SS0_I2SLRCLK_W {
        SPI0_SS0_I2SLRCLK_W { w: self }
    }
    #[doc = "Bit 1 - PC.1 Pin Function Selection Bits PC1_I2SBCLK (ALT_MFP\\[6\\]) and GPC_MFP\\[1\\]
determine the PC.1 function. PC1_I2SBCLK GPC_MFP\\[1\\]
PC.1 function x 0 GPIO 0 1 SPICLK0 (SPI0) 1 1 I2SBLK (I2S)"]
    #[inline(always)]
    pub fn spi0_clk_i2sbclk(&mut self) -> SPI0_CLK_I2SBCLK_W {
        SPI0_CLK_I2SBCLK_W { w: self }
    }
    #[doc = "Bit 2 - PC.2 Pin Function Selection Bits PC2_I2SDI (ALT_MFP\\[7\\]) and GPC_MFP\\[2\\]
determine the PC.2 function. PC2_I2SDI GPC_MFP\\[2\\]
PC.2 function x 0 GPIO 0 1 MISO00 (SPI0) 1 1 I2SDI (I2S)"]
    #[inline(always)]
    pub fn spi0_miso0_i2sdi(&mut self) -> SPI0_MISO0_I2SDI_W {
        SPI0_MISO0_I2SDI_W { w: self }
    }
    #[doc = "Bit 3 - PC.3 Pin Function Selection Bits PC3_I2SDO (ALT_MFP\\[8\\]) and GPC_MFP\\[3\\]
determine the PC.3 function. PC3_I2SDO GPC_MFP\\[3\\]
PC.3 function x 0 GPIO 0 1 MOSI00 (SPI0) 1 1 I2SDO (I2S)"]
    #[inline(always)]
    pub fn spi0_mosi0_i2sdo(&mut self) -> SPI0_MOSI0_I2SDO_W {
        SPI0_MOSI0_I2SDO_W { w: self }
    }
    #[doc = "Bit 4 - PC.4 Pin Function Selection 1 = The SPI0 MISO1 (master input, slave output pin-1) function is selected to the pin PC.4 0 = The GPIOC\\[4\\]
is selected to the pin PC.4"]
    #[inline(always)]
    pub fn spi0_miso1(&mut self) -> SPI0_MISO1_W {
        SPI0_MISO1_W { w: self }
    }
    #[doc = "Bit 5 - PC.5 Pin Function Selection 1 = The SPI0 MOSI1 (master output, slave input pin-1) function is selected to the pin PC.5 0 = The GPIOC\\[5\\]
is selected to the pin PC.5"]
    #[inline(always)]
    pub fn spi0_mosi1(&mut self) -> SPI0_MOSI1_W {
        SPI0_MOSI1_W { w: self }
    }
    #[doc = "Bit 6 - PC.6 Pin Function Selection The pin function depends on GPC_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[6\\]
PC.6 function x 0 GPIO 0 1 CPP0 (CMP) 1 1 AD4 (EBI AD bus bit 4)"]
    #[inline(always)]
    pub fn cpp0_ad4(&mut self) -> CPP0_AD4_W {
        CPP0_AD4_W { w: self }
    }
    #[doc = "Bit 7 - PC.7 Pin Function Selection The pin function depends on GPC_MFP7 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[7\\]
PC.7 function x 0 GPIO 0 1 CPN0 (CMP) 1 1 AD5 (EBI AD bus bit 5)"]
    #[inline(always)]
    pub fn cpn0_ad5(&mut self) -> CPN0_AD5_W {
        CPN0_AD5_W { w: self }
    }
    #[doc = "Bit 8 - PC.8 Pin Function Selection The pin function depends on GPC_MFP8 and EBI_MCLK_EN (ALT_MFP\\[12\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_MCLK_EN EBI_EN GPC_MFP\\[8\\]
PC.8 function x x 0 GPIO x 0 1 SPISS10 (SPI1) 0 1 1 SPISS10 (SPI1) 1 1 1 MCLK (EBI Clock output)"]
    #[inline(always)]
    pub fn spi1_ss0_mclk(&mut self) -> SPI1_SS0_MCLK_W {
        SPI1_SS0_MCLK_W { w: self }
    }
    #[doc = "Bit 9 - PC.9 Pin Function Selection 1 = The SPI1 SPICLK function is selected to the pin PC.9 0 = The GPIOC\\[9\\]
is selected to the pin PC.9"]
    #[inline(always)]
    pub fn spi1_clk(&mut self) -> SPI1_CLK_W {
        SPI1_CLK_W { w: self }
    }
    #[doc = "Bit 10 - PC.10 Pin Function Selection 1 = The SPI1 MISO0 (master input, slave output pin-0) function is selected to the pin PC.10 0 = The GPIOC\\[10\\]
is selected to the pin PC.10"]
    #[inline(always)]
    pub fn spi1_miso0(&mut self) -> SPI1_MISO0_W {
        SPI1_MISO0_W { w: self }
    }
    #[doc = "Bit 11 - PC.11 Pin Function Selection 1 = The SPI1 MOSI0 (master output, slave input pin-0) function is selected to the pin PC.11 0 = The GPIOC\\[11\\]
is selected to the pin PC.11"]
    #[inline(always)]
    pub fn spi1_mosi0(&mut self) -> SPI1_MOSI0_W {
        SPI1_MOSI0_W { w: self }
    }
    #[doc = "Bit 12 - PC.12 Pin Function Selection 1 = The SPI1 MISO1 (master input, slave output pin-1) function is selected to the pin PC.12 0 = The GPIOC\\[12\\]
is selected to the pin PC.12"]
    #[inline(always)]
    pub fn spi1_miso1(&mut self) -> SPI1_MISO1_W {
        SPI1_MISO1_W { w: self }
    }
    #[doc = "Bit 13 - PC.13 Pin Function Selection 1 = The SPI1 MOSI1 (master output, slave input pin-1) function is selected to the pin PC.13 0 = The GPIOC\\[13\\]
is selected to the pin PC.13"]
    #[inline(always)]
    pub fn spi1_mosi1(&mut self) -> SPI1_MOSI1_W {
        SPI1_MOSI1_W { w: self }
    }
    #[doc = "Bit 14 - PC.14 Pin Function Selection The pin function depends on GPC_MFP14 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[14\\]
PC.14 function x 0 GPIO 0 1 CPP1 (CMP) 1 1 AD2 (EBI AD bus bit 2)"]
    #[inline(always)]
    pub fn cpp1_ad2(&mut self) -> CPP1_AD2_W {
        CPP1_AD2_W { w: self }
    }
    #[doc = "Bit 15 - PC.15 Pin Function Selection The pin function depends on GPC_MFP15 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPC_MFP\\[15\\]
PC.15 function x 0 GPIO 0 1 CPN1 (CMP) 1 1 AD3 (EBI AD bus bit 3)"]
    #[inline(always)]
    pub fn cpp1_ad3(&mut self) -> CPP1_AD3_W {
        CPP1_AD3_W { w: self }
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOC\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOC\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn schmitt(&mut self) -> SCHMITT_W {
        SCHMITT_W { w: self }
    }
}
