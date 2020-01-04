#[doc = "Reader of register GPD_MFP"]
pub type R = crate::R<u32, super::GPD_MFP>;
#[doc = "Writer for register GPD_MFP"]
pub type W = crate::W<u32, super::GPD_MFP>;
#[doc = "Register GPD_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPD_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPD_MFP0`"]
pub type GPD_MFP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP0`"]
pub struct GPD_MFP0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP0_W<'a> {
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
#[doc = "Reader of field `GPD_MFP1`"]
pub type GPD_MFP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP1`"]
pub struct GPD_MFP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP1_W<'a> {
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
#[doc = "Reader of field `GPD_MFP2`"]
pub type GPD_MFP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP2`"]
pub struct GPD_MFP2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP2_W<'a> {
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
#[doc = "Reader of field `GPD_MFP3`"]
pub type GPD_MFP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP3`"]
pub struct GPD_MFP3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP3_W<'a> {
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
#[doc = "Reader of field `GPD_MFP4`"]
pub type GPD_MFP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP4`"]
pub struct GPD_MFP4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP4_W<'a> {
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
#[doc = "Reader of field `GPD_MFP5`"]
pub type GPD_MFP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP5`"]
pub struct GPD_MFP5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP5_W<'a> {
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
#[doc = "Reader of field `GPD_MFP6`"]
pub type GPD_MFP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP6`"]
pub struct GPD_MFP6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP6_W<'a> {
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
#[doc = "Reader of field `GPD_MFP7`"]
pub type GPD_MFP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP7`"]
pub struct GPD_MFP7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP7_W<'a> {
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
#[doc = "Reader of field `GPD_MFP8`"]
pub type GPD_MFP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP8`"]
pub struct GPD_MFP8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP8_W<'a> {
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
#[doc = "Reader of field `GPD_MFP9`"]
pub type GPD_MFP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP9`"]
pub struct GPD_MFP9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP9_W<'a> {
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
#[doc = "Reader of field `GPD_MFP10`"]
pub type GPD_MFP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP10`"]
pub struct GPD_MFP10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP10_W<'a> {
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
#[doc = "Reader of field `GPD_MFP11`"]
pub type GPD_MFP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP11`"]
pub struct GPD_MFP11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP11_W<'a> {
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
#[doc = "Reader of field `GPD_MFP12`"]
pub type GPD_MFP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP12`"]
pub struct GPD_MFP12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP12_W<'a> {
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
#[doc = "Reader of field `GPD_MFP13`"]
pub type GPD_MFP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP13`"]
pub struct GPD_MFP13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP13_W<'a> {
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
#[doc = "Reader of field `GPD_MFP14`"]
pub type GPD_MFP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP14`"]
pub struct GPD_MFP14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP14_W<'a> {
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
#[doc = "Reader of field `GPD_MFP15`"]
pub type GPD_MFP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPD_MFP15`"]
pub struct GPD_MFP15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_MFP15_W<'a> {
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
#[doc = "Reader of field `GPD_TYPEn`"]
pub type GPD_TYPEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPD_TYPEn`"]
pub struct GPD_TYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPD_TYPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PD.0 Pin Function Selection (Medium Density Only) 1 = The SPI2 SS20 function is selected to the pin PD.0 0 = The GPIOD\\[0\\]
is selected to the pin PD.0"]
    #[inline(always)]
    pub fn gpd_mfp0(&self) -> GPD_MFP0_R {
        GPD_MFP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PD.1 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 SPICLK function is selected to the pin PD.1 0 = The GPIOD\\[1\\]
is selected to the pin PD.1 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 SS01 function is selected to the pin PD.1 0 = The GPIOD\\[1\\]
is selected to the pin PD.1"]
    #[inline(always)]
    pub fn gpd_mfp1(&self) -> GPD_MFP1_R {
        GPD_MFP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PD.2 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 MISO0 (master input, slave output pin-0) function is selected to the pin PD.2 0 = The GPIOD\\[2\\]
is selected to the pin PD.2 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 MISO1 (master input, slave output pin-1) function is selected to the pin PD.2 0 = The GPIOD\\[2\\]
is selected to the pin PD.2"]
    #[inline(always)]
    pub fn gpd_mfp2(&self) -> GPD_MFP2_R {
        GPD_MFP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PD.3 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 MOSI0 (master output, slave input pin-0) function is selected to the pin GPD3 0 = The GPIOD\\[3\\]
is selected to the pin PD.3 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.3 0 = The GPIOD\\[3\\]
is selected to the pin PD.3"]
    #[inline(always)]
    pub fn gpd_mfp3(&self) -> GPD_MFP3_R {
        GPD_MFP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PD.4 Pin Function Selection (Medium Density Only) 1 = The SPI2 MISO1 (master input, slave output pin-1) function is selected to the pin PD.4 0 = The GPIOD\\[4\\]is selected to the pin PD.4"]
    #[inline(always)]
    pub fn gpd_mfp4(&self) -> GPD_MFP4_R {
        GPD_MFP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PD.5 Pin Function Selection (Medium Density Only) 1 = The SPI2 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.5 0 = The GPIOD\\[5\\]
is selected to the pin PD.5"]
    #[inline(always)]
    pub fn gpd_mfp5(&self) -> GPD_MFP5_R {
        GPD_MFP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PD.6 Pin Function Selection (Medium Density Only) 1 = The CAN0 RX function is selected to the pin PD.6 0 = The GPIOD\\[6\\]
is selected to the pin PD.6"]
    #[inline(always)]
    pub fn gpd_mfp6(&self) -> GPD_MFP6_R {
        GPD_MFP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PD.7 Pin Function Selection (Medium Density Only) 1 = The CAN0 TX function is selected to the pin PD.7 0 = The GPIOD\\[7\\]
is selected to the pin PD.7"]
    #[inline(always)]
    pub fn gpd_mfp7(&self) -> GPD_MFP7_R {
        GPD_MFP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PD.8 Pin Function Selection (Medium Density Only) 1 = The SPI3 SS30 function is selected to the pin PD8 0 = The GPIOD\\[8\\]
is selected to the pin PD8"]
    #[inline(always)]
    pub fn gpd_mfp8(&self) -> GPD_MFP8_R {
        GPD_MFP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PD.9 Pin Function Selection (Medium Density Only) 1 = The SPI3 SPICLK function is selected to the pin PD.9 0 = The GPIOD-9 is selected to the pin PD.9"]
    #[inline(always)]
    pub fn gpd_mfp9(&self) -> GPD_MFP9_R {
        GPD_MFP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PD.10 Pin Function Selection (Medium Density Only) 1 = The SPI3 MISO0 (master input, slave output pin-0) function is selected to the pin PD.10 0 = The GPIOD\\[10\\]
is selected to the pin PD.10"]
    #[inline(always)]
    pub fn gpd_mfp10(&self) -> GPD_MFP10_R {
        GPD_MFP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PD.11 Pin Function Selection (Medium Density Only) 1 = The SPI3 MOSI0 (master output, slave input pin-0) function is selected to the pin PD.11 0 = The GPIOD\\[11\\]
is selected to the pin PD.11"]
    #[inline(always)]
    pub fn gpd_mfp11(&self) -> GPD_MFP11_R {
        GPD_MFP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PD.12 Pin Function Selection (Medium Density Only) 1 = The SPI3 MISO1 (master input, slave output pin-1) function is selected to the pin PD.12 0 = The GPIOD\\[12\\]
is selected to the pin PD.12"]
    #[inline(always)]
    pub fn gpd_mfp12(&self) -> GPD_MFP12_R {
        GPD_MFP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PD.13 Pin Function Selection (Medium Density Only) 1 = The SPI3 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.13 0 = The GPIOD\\[13\\]
is selected to the pin PD.13"]
    #[inline(always)]
    pub fn gpd_mfp13(&self) -> GPD_MFP13_R {
        GPD_MFP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PD.14 Pin Function Selection (Medium Density Only) 1 = The UART2 RXD function is selected to the pin PD.14 0 = The GPIOD\\[14\\]
selected to the pin PD.14"]
    #[inline(always)]
    pub fn gpd_mfp14(&self) -> GPD_MFP14_R {
        GPD_MFP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PD.15 Pin Function Selection (Medium Density Only) 1 = The UART2 TXD function is selected to the pin PD.15 0 = The GPIOD\\[15\\]
selected to the pin PD.15"]
    #[inline(always)]
    pub fn gpd_mfp15(&self) -> GPD_MFP15_R {
        GPD_MFP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOD\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOD\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpd_typen(&self) -> GPD_TYPEN_R {
        GPD_TYPEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PD.0 Pin Function Selection (Medium Density Only) 1 = The SPI2 SS20 function is selected to the pin PD.0 0 = The GPIOD\\[0\\]
is selected to the pin PD.0"]
    #[inline(always)]
    pub fn gpd_mfp0(&mut self) -> GPD_MFP0_W {
        GPD_MFP0_W { w: self }
    }
    #[doc = "Bit 1 - PD.1 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 SPICLK function is selected to the pin PD.1 0 = The GPIOD\\[1\\]
is selected to the pin PD.1 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 SS01 function is selected to the pin PD.1 0 = The GPIOD\\[1\\]
is selected to the pin PD.1"]
    #[inline(always)]
    pub fn gpd_mfp1(&mut self) -> GPD_MFP1_W {
        GPD_MFP1_W { w: self }
    }
    #[doc = "Bit 2 - PD.2 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 MISO0 (master input, slave output pin-0) function is selected to the pin PD.2 0 = The GPIOD\\[2\\]
is selected to the pin PD.2 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 MISO1 (master input, slave output pin-1) function is selected to the pin PD.2 0 = The GPIOD\\[2\\]
is selected to the pin PD.2"]
    #[inline(always)]
    pub fn gpd_mfp2(&mut self) -> GPD_MFP2_W {
        GPD_MFP2_W { w: self }
    }
    #[doc = "Bit 3 - PD.3 Pin Function Selection For NUC100/NUC120/NUC130/NUC140 Medium Density 1 = The SPI2 MOSI0 (master output, slave input pin-0) function is selected to the pin GPD3 0 = The GPIOD\\[3\\]
is selected to the pin PD.3 For NUC100/NUC120/NUC130/NUC140 Low Density and NUC101 LQFP48 package Reserved For NUC101 QFN36 package 1 = The SPI0 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.3 0 = The GPIOD\\[3\\]
is selected to the pin PD.3"]
    #[inline(always)]
    pub fn gpd_mfp3(&mut self) -> GPD_MFP3_W {
        GPD_MFP3_W { w: self }
    }
    #[doc = "Bit 4 - PD.4 Pin Function Selection (Medium Density Only) 1 = The SPI2 MISO1 (master input, slave output pin-1) function is selected to the pin PD.4 0 = The GPIOD\\[4\\]is selected to the pin PD.4"]
    #[inline(always)]
    pub fn gpd_mfp4(&mut self) -> GPD_MFP4_W {
        GPD_MFP4_W { w: self }
    }
    #[doc = "Bit 5 - PD.5 Pin Function Selection (Medium Density Only) 1 = The SPI2 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.5 0 = The GPIOD\\[5\\]
is selected to the pin PD.5"]
    #[inline(always)]
    pub fn gpd_mfp5(&mut self) -> GPD_MFP5_W {
        GPD_MFP5_W { w: self }
    }
    #[doc = "Bit 6 - PD.6 Pin Function Selection (Medium Density Only) 1 = The CAN0 RX function is selected to the pin PD.6 0 = The GPIOD\\[6\\]
is selected to the pin PD.6"]
    #[inline(always)]
    pub fn gpd_mfp6(&mut self) -> GPD_MFP6_W {
        GPD_MFP6_W { w: self }
    }
    #[doc = "Bit 7 - PD.7 Pin Function Selection (Medium Density Only) 1 = The CAN0 TX function is selected to the pin PD.7 0 = The GPIOD\\[7\\]
is selected to the pin PD.7"]
    #[inline(always)]
    pub fn gpd_mfp7(&mut self) -> GPD_MFP7_W {
        GPD_MFP7_W { w: self }
    }
    #[doc = "Bit 8 - PD.8 Pin Function Selection (Medium Density Only) 1 = The SPI3 SS30 function is selected to the pin PD8 0 = The GPIOD\\[8\\]
is selected to the pin PD8"]
    #[inline(always)]
    pub fn gpd_mfp8(&mut self) -> GPD_MFP8_W {
        GPD_MFP8_W { w: self }
    }
    #[doc = "Bit 9 - PD.9 Pin Function Selection (Medium Density Only) 1 = The SPI3 SPICLK function is selected to the pin PD.9 0 = The GPIOD-9 is selected to the pin PD.9"]
    #[inline(always)]
    pub fn gpd_mfp9(&mut self) -> GPD_MFP9_W {
        GPD_MFP9_W { w: self }
    }
    #[doc = "Bit 10 - PD.10 Pin Function Selection (Medium Density Only) 1 = The SPI3 MISO0 (master input, slave output pin-0) function is selected to the pin PD.10 0 = The GPIOD\\[10\\]
is selected to the pin PD.10"]
    #[inline(always)]
    pub fn gpd_mfp10(&mut self) -> GPD_MFP10_W {
        GPD_MFP10_W { w: self }
    }
    #[doc = "Bit 11 - PD.11 Pin Function Selection (Medium Density Only) 1 = The SPI3 MOSI0 (master output, slave input pin-0) function is selected to the pin PD.11 0 = The GPIOD\\[11\\]
is selected to the pin PD.11"]
    #[inline(always)]
    pub fn gpd_mfp11(&mut self) -> GPD_MFP11_W {
        GPD_MFP11_W { w: self }
    }
    #[doc = "Bit 12 - PD.12 Pin Function Selection (Medium Density Only) 1 = The SPI3 MISO1 (master input, slave output pin-1) function is selected to the pin PD.12 0 = The GPIOD\\[12\\]
is selected to the pin PD.12"]
    #[inline(always)]
    pub fn gpd_mfp12(&mut self) -> GPD_MFP12_W {
        GPD_MFP12_W { w: self }
    }
    #[doc = "Bit 13 - PD.13 Pin Function Selection (Medium Density Only) 1 = The SPI3 MOSI1 (master output, slave input pin-1) function is selected to the pin PD.13 0 = The GPIOD\\[13\\]
is selected to the pin PD.13"]
    #[inline(always)]
    pub fn gpd_mfp13(&mut self) -> GPD_MFP13_W {
        GPD_MFP13_W { w: self }
    }
    #[doc = "Bit 14 - PD.14 Pin Function Selection (Medium Density Only) 1 = The UART2 RXD function is selected to the pin PD.14 0 = The GPIOD\\[14\\]
selected to the pin PD.14"]
    #[inline(always)]
    pub fn gpd_mfp14(&mut self) -> GPD_MFP14_W {
        GPD_MFP14_W { w: self }
    }
    #[doc = "Bit 15 - PD.15 Pin Function Selection (Medium Density Only) 1 = The UART2 TXD function is selected to the pin PD.15 0 = The GPIOD\\[15\\]
selected to the pin PD.15"]
    #[inline(always)]
    pub fn gpd_mfp15(&mut self) -> GPD_MFP15_W {
        GPD_MFP15_W { w: self }
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOD\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOD\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpd_typen(&mut self) -> GPD_TYPEN_W {
        GPD_TYPEN_W { w: self }
    }
}
