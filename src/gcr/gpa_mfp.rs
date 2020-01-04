#[doc = "Reader of register GPA_MFP"]
pub type R = crate::R<u32, super::GPA_MFP>;
#[doc = "Writer for register GPA_MFP"]
pub type W = crate::W<u32, super::GPA_MFP>;
#[doc = "Register GPA_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPA_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPA_MFP0`"]
pub type GPA_MFP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP0`"]
pub struct GPA_MFP0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP0_W<'a> {
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
#[doc = "Reader of field `GPA_MFP1`"]
pub type GPA_MFP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP1`"]
pub struct GPA_MFP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP1_W<'a> {
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
#[doc = "Reader of field `GPA_MFP2`"]
pub type GPA_MFP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP2`"]
pub struct GPA_MFP2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP2_W<'a> {
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
#[doc = "Reader of field `GPA_MFP3`"]
pub type GPA_MFP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP3`"]
pub struct GPA_MFP3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP3_W<'a> {
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
#[doc = "Reader of field `GPA_MFP4`"]
pub type GPA_MFP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP4`"]
pub struct GPA_MFP4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP4_W<'a> {
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
#[doc = "Reader of field `GPA_MFP5`"]
pub type GPA_MFP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP5`"]
pub struct GPA_MFP5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP5_W<'a> {
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
#[doc = "Reader of field `GPA_MFP6`"]
pub type GPA_MFP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP6`"]
pub struct GPA_MFP6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP6_W<'a> {
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
#[doc = "Reader of field `GPA_MFP7`"]
pub type GPA_MFP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP7`"]
pub struct GPA_MFP7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP7_W<'a> {
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
#[doc = "Reader of field `GPA_MFP8`"]
pub type GPA_MFP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP8`"]
pub struct GPA_MFP8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP8_W<'a> {
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
#[doc = "Reader of field `GPA_MFP9`"]
pub type GPA_MFP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP9`"]
pub struct GPA_MFP9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP9_W<'a> {
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
#[doc = "Reader of field `GPA_MFP10`"]
pub type GPA_MFP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP10`"]
pub struct GPA_MFP10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP10_W<'a> {
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
#[doc = "Reader of field `GPA_MFP11`"]
pub type GPA_MFP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP11`"]
pub struct GPA_MFP11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP11_W<'a> {
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
#[doc = "Reader of field `GPA_MFP12`"]
pub type GPA_MFP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP12`"]
pub struct GPA_MFP12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP12_W<'a> {
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
#[doc = "Reader of field `GPA_MFP13`"]
pub type GPA_MFP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP13`"]
pub struct GPA_MFP13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP13_W<'a> {
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
#[doc = "Reader of field `GPA_MFP14`"]
pub type GPA_MFP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP14`"]
pub struct GPA_MFP14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP14_W<'a> {
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
#[doc = "Reader of field `GPA_MFP15`"]
pub type GPA_MFP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPA_MFP15`"]
pub struct GPA_MFP15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_MFP15_W<'a> {
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
#[doc = "Reader of field `GPA_TYPEn`"]
pub type GPA_TYPEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPA_TYPEn`"]
pub struct GPA_TYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPA_TYPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PA.0 Pin Function Selection 1 = The ADC0 (Analog-to-Digital converter channel 0) function is selected to the pin PA.0 0 = The GPIOA\\[0\\]
is selected to the pin PA.0"]
    #[inline(always)]
    pub fn gpa_mfp0(&self) -> GPA_MFP0_R {
        GPA_MFP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PA.1 Pin Function Selection The pin function depends on GPA_MFP1 and EBI_HB_EN\\[4\\]
(ALT_MFP\\[20\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[4\\]
EBI_EN GPA_MFP\\[1\\]
PA.1 function x x 0 GPIO x 0 1 ADC1 (ADC) 0 1 1 ADC1 (ADC) 1 1 1 AD12 (EBI AD bus bit 12)"]
    #[inline(always)]
    pub fn gpa_mfp1(&self) -> GPA_MFP1_R {
        GPA_MFP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PA.2 Pin Function Selection The pin function depends on GPA_MFP2 and EBI_HB_EN\\[3\\]
(ALT_MFP\\[19\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[3\\]
EBI_EN GPA_MFP\\[2\\]
PA.2 function x x 0 GPIO x 0 1 ADC2 (ADC) 0 1 1 ADC2 (ADC) 1 1 1 AD11 (EBI AD bus bit 11)"]
    #[inline(always)]
    pub fn gpa_mfp2(&self) -> GPA_MFP2_R {
        GPA_MFP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PA.3 Pin Function Selection The pin function depends on GPA_MFP3 and EBI_HB_EN\\[2\\]
(ALT_MFP\\[18\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[2\\]
EBI_EN GPA_MFP\\[3\\]
PA.3 function x x 0 GPIO x 0 1 ADC3 (ADC) 0 1 1 ADC3 (ADC) 1 1 1 AD10 (EBI AD bus bit 10)"]
    #[inline(always)]
    pub fn gpa_mfp3(&self) -> GPA_MFP3_R {
        GPA_MFP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PA.4 Pin Function Selection The pin function depends on GPA_MFP4 and EBI_HB_EN\\[1\\]
(ALT_MFP\\[17\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[1\\]
EBI_EN GPA_MFP\\[4\\]
PA.4 function x x 0 GPIO x 0 1 ADC4 (ADC) 0 1 1 ADC4 (ADC) 1 1 1 AD9 (EBI AD bus bit 9)"]
    #[inline(always)]
    pub fn gpa_mfp4(&self) -> GPA_MFP4_R {
        GPA_MFP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PA.5 Pin Function Selection The pin function depends on GPA_MFP5 and EBI_HB_EN\\[0\\]
(ALT_MFP\\[16\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[0\\]
EBI_EN GPA_MFP\\[5\\]
PA.5 function x x 0 GPIO x 0 1 ADC5 (ADC) 0 1 1 ADC5 (ADC) 1 1 1 AD8 (EBI AD bus bit 8)"]
    #[inline(always)]
    pub fn gpa_mfp5(&self) -> GPA_MFP5_R {
        GPA_MFP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PA.6 Pin Function Selection The pin function depends on GPA_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[6\\]
PA.6 function x 0 GPIO 0 1 ADC6 (ADC) 1 1 AD7 (EBI AD bus bit 7)"]
    #[inline(always)]
    pub fn gpa_mfp6(&self) -> GPA_MFP6_R {
        GPA_MFP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PA.7 Pin Function Selection The pin function depends on GPA_MFP7 and PA7_S21 (ALT_MFP\\[2\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_EN PA7_S21 GPA_MFP\\[7\\]
PA.7 function x x 0 GPIO 0 0 1 ADC7 (ADC) 0 1 1 SPISS21 (SPI2) 1 x 1 AD6 (EBI AD bus bit 6)"]
    #[inline(always)]
    pub fn gpa_mfp7(&self) -> GPA_MFP7_R {
        GPA_MFP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PA.8 Pin Function Selection 1 = The I2C0 SDA function is selected to the pin PA.8 0 = The GPIOA\\[8\\]
is selected to the pin PA.8"]
    #[inline(always)]
    pub fn gpa_mfp8(&self) -> GPA_MFP8_R {
        GPA_MFP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PA.9 Pin Function Selection 1 = The I2C0 SCL function is selected to the pin PA.9 0 = The GPIOA\\[9\\]
is selected to the pin PA.9"]
    #[inline(always)]
    pub fn gpa_mfp9(&self) -> GPA_MFP9_R {
        GPA_MFP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PA.10 Pin Function Selection The pin function depends on GPA_MFP10 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[10\\]
PA.10 function x 0 GPIO 0 1 SDA1 (I2C) 1 1 nWR (EBI)"]
    #[inline(always)]
    pub fn gpa_mfp10(&self) -> GPA_MFP10_R {
        GPA_MFP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PA.11 Pin Function Selection The pin function depends on GPA_MFP11 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[11\\]
PA.11 function x 0 GPIO 0 1 SCL1 (I2C) 1 1 nRD (EBI)"]
    #[inline(always)]
    pub fn gpa_mfp11(&self) -> GPA_MFP11_R {
        GPA_MFP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PA.12 Pin Function Selection The pin function depends on GPA_MFP12 and EBI_HB_EN\\[5\\]
(ALT_MFP\\[21\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[5\\]
EBI_EN GPA_MFP\\[12\\]
PA.12 function x x 0 GPIO x 0 1 PWM0 (PWM) 0 1 1 PWM0 (PWM) 1 1 1 AD13 (EBI AD bus bit 13)"]
    #[inline(always)]
    pub fn gpa_mfp12(&self) -> GPA_MFP12_R {
        GPA_MFP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PA.13 Pin Function Selection The pin function depends on GPA_MFP13 and EBI_HB_EN\\[6\\]
(ALT_MFP\\[22\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[6\\]
EBI_EN GPA_MFP\\[13\\]
PA.13 function x x 0 GPIO x 0 1 PWM1 (PWM) 0 1 1 PWM1 (PWM) 1 1 1 AD14 (EBI AD bus bit 14)"]
    #[inline(always)]
    pub fn gpa_mfp13(&self) -> GPA_MFP13_R {
        GPA_MFP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PA.14 Pin Function Selection The pin function depends on GPA_MFP14 and EBI_HB_EN\\[7\\]
(ALT_MFP\\[23\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[7\\]
EBI_EN GPA_MFP\\[14\\]
PA.14 function x x 0 GPIO x 0 1 PWM2 (PWM) 0 1 1 PWM2 (PWM) 1 1 1 AD15 (EBI AD bus bit 15)"]
    #[inline(always)]
    pub fn gpa_mfp14(&self) -> GPA_MFP14_R {
        GPA_MFP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PA.14 Pin Function Selection The pin function depends on GPA_MFP15 and PA15_I2SMCLK (ALT_MFP\\[9\\]). PA15_I2SMCLK GPA_MFP\\[15\\]
PA.15 function x 0 GPIO 0 1 PWM3 (PWM) 1 1 I2SMCLK (I2S)"]
    #[inline(always)]
    pub fn gpa_mfp15(&self) -> GPA_MFP15_R {
        GPA_MFP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOA\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOA\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpa_typen(&self) -> GPA_TYPEN_R {
        GPA_TYPEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PA.0 Pin Function Selection 1 = The ADC0 (Analog-to-Digital converter channel 0) function is selected to the pin PA.0 0 = The GPIOA\\[0\\]
is selected to the pin PA.0"]
    #[inline(always)]
    pub fn gpa_mfp0(&mut self) -> GPA_MFP0_W {
        GPA_MFP0_W { w: self }
    }
    #[doc = "Bit 1 - PA.1 Pin Function Selection The pin function depends on GPA_MFP1 and EBI_HB_EN\\[4\\]
(ALT_MFP\\[20\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[4\\]
EBI_EN GPA_MFP\\[1\\]
PA.1 function x x 0 GPIO x 0 1 ADC1 (ADC) 0 1 1 ADC1 (ADC) 1 1 1 AD12 (EBI AD bus bit 12)"]
    #[inline(always)]
    pub fn gpa_mfp1(&mut self) -> GPA_MFP1_W {
        GPA_MFP1_W { w: self }
    }
    #[doc = "Bit 2 - PA.2 Pin Function Selection The pin function depends on GPA_MFP2 and EBI_HB_EN\\[3\\]
(ALT_MFP\\[19\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[3\\]
EBI_EN GPA_MFP\\[2\\]
PA.2 function x x 0 GPIO x 0 1 ADC2 (ADC) 0 1 1 ADC2 (ADC) 1 1 1 AD11 (EBI AD bus bit 11)"]
    #[inline(always)]
    pub fn gpa_mfp2(&mut self) -> GPA_MFP2_W {
        GPA_MFP2_W { w: self }
    }
    #[doc = "Bit 3 - PA.3 Pin Function Selection The pin function depends on GPA_MFP3 and EBI_HB_EN\\[2\\]
(ALT_MFP\\[18\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[2\\]
EBI_EN GPA_MFP\\[3\\]
PA.3 function x x 0 GPIO x 0 1 ADC3 (ADC) 0 1 1 ADC3 (ADC) 1 1 1 AD10 (EBI AD bus bit 10)"]
    #[inline(always)]
    pub fn gpa_mfp3(&mut self) -> GPA_MFP3_W {
        GPA_MFP3_W { w: self }
    }
    #[doc = "Bit 4 - PA.4 Pin Function Selection The pin function depends on GPA_MFP4 and EBI_HB_EN\\[1\\]
(ALT_MFP\\[17\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[1\\]
EBI_EN GPA_MFP\\[4\\]
PA.4 function x x 0 GPIO x 0 1 ADC4 (ADC) 0 1 1 ADC4 (ADC) 1 1 1 AD9 (EBI AD bus bit 9)"]
    #[inline(always)]
    pub fn gpa_mfp4(&mut self) -> GPA_MFP4_W {
        GPA_MFP4_W { w: self }
    }
    #[doc = "Bit 5 - PA.5 Pin Function Selection The pin function depends on GPA_MFP5 and EBI_HB_EN\\[0\\]
(ALT_MFP\\[16\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[0\\]
EBI_EN GPA_MFP\\[5\\]
PA.5 function x x 0 GPIO x 0 1 ADC5 (ADC) 0 1 1 ADC5 (ADC) 1 1 1 AD8 (EBI AD bus bit 8)"]
    #[inline(always)]
    pub fn gpa_mfp5(&mut self) -> GPA_MFP5_W {
        GPA_MFP5_W { w: self }
    }
    #[doc = "Bit 6 - PA.6 Pin Function Selection The pin function depends on GPA_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[6\\]
PA.6 function x 0 GPIO 0 1 ADC6 (ADC) 1 1 AD7 (EBI AD bus bit 7)"]
    #[inline(always)]
    pub fn gpa_mfp6(&mut self) -> GPA_MFP6_W {
        GPA_MFP6_W { w: self }
    }
    #[doc = "Bit 7 - PA.7 Pin Function Selection The pin function depends on GPA_MFP7 and PA7_S21 (ALT_MFP\\[2\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_EN PA7_S21 GPA_MFP\\[7\\]
PA.7 function x x 0 GPIO 0 0 1 ADC7 (ADC) 0 1 1 SPISS21 (SPI2) 1 x 1 AD6 (EBI AD bus bit 6)"]
    #[inline(always)]
    pub fn gpa_mfp7(&mut self) -> GPA_MFP7_W {
        GPA_MFP7_W { w: self }
    }
    #[doc = "Bit 8 - PA.8 Pin Function Selection 1 = The I2C0 SDA function is selected to the pin PA.8 0 = The GPIOA\\[8\\]
is selected to the pin PA.8"]
    #[inline(always)]
    pub fn gpa_mfp8(&mut self) -> GPA_MFP8_W {
        GPA_MFP8_W { w: self }
    }
    #[doc = "Bit 9 - PA.9 Pin Function Selection 1 = The I2C0 SCL function is selected to the pin PA.9 0 = The GPIOA\\[9\\]
is selected to the pin PA.9"]
    #[inline(always)]
    pub fn gpa_mfp9(&mut self) -> GPA_MFP9_W {
        GPA_MFP9_W { w: self }
    }
    #[doc = "Bit 10 - PA.10 Pin Function Selection The pin function depends on GPA_MFP10 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[10\\]
PA.10 function x 0 GPIO 0 1 SDA1 (I2C) 1 1 nWR (EBI)"]
    #[inline(always)]
    pub fn gpa_mfp10(&mut self) -> GPA_MFP10_W {
        GPA_MFP10_W { w: self }
    }
    #[doc = "Bit 11 - PA.11 Pin Function Selection The pin function depends on GPA_MFP11 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPA_MFP\\[11\\]
PA.11 function x 0 GPIO 0 1 SCL1 (I2C) 1 1 nRD (EBI)"]
    #[inline(always)]
    pub fn gpa_mfp11(&mut self) -> GPA_MFP11_W {
        GPA_MFP11_W { w: self }
    }
    #[doc = "Bit 12 - PA.12 Pin Function Selection The pin function depends on GPA_MFP12 and EBI_HB_EN\\[5\\]
(ALT_MFP\\[21\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[5\\]
EBI_EN GPA_MFP\\[12\\]
PA.12 function x x 0 GPIO x 0 1 PWM0 (PWM) 0 1 1 PWM0 (PWM) 1 1 1 AD13 (EBI AD bus bit 13)"]
    #[inline(always)]
    pub fn gpa_mfp12(&mut self) -> GPA_MFP12_W {
        GPA_MFP12_W { w: self }
    }
    #[doc = "Bit 13 - PA.13 Pin Function Selection The pin function depends on GPA_MFP13 and EBI_HB_EN\\[6\\]
(ALT_MFP\\[22\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[6\\]
EBI_EN GPA_MFP\\[13\\]
PA.13 function x x 0 GPIO x 0 1 PWM1 (PWM) 0 1 1 PWM1 (PWM) 1 1 1 AD14 (EBI AD bus bit 14)"]
    #[inline(always)]
    pub fn gpa_mfp13(&mut self) -> GPA_MFP13_W {
        GPA_MFP13_W { w: self }
    }
    #[doc = "Bit 14 - PA.14 Pin Function Selection The pin function depends on GPA_MFP14 and EBI_HB_EN\\[7\\]
(ALT_MFP\\[23\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_HB_EN\\[7\\]
EBI_EN GPA_MFP\\[14\\]
PA.14 function x x 0 GPIO x 0 1 PWM2 (PWM) 0 1 1 PWM2 (PWM) 1 1 1 AD15 (EBI AD bus bit 15)"]
    #[inline(always)]
    pub fn gpa_mfp14(&mut self) -> GPA_MFP14_W {
        GPA_MFP14_W { w: self }
    }
    #[doc = "Bit 15 - PA.14 Pin Function Selection The pin function depends on GPA_MFP15 and PA15_I2SMCLK (ALT_MFP\\[9\\]). PA15_I2SMCLK GPA_MFP\\[15\\]
PA.15 function x 0 GPIO 0 1 PWM3 (PWM) 1 1 I2SMCLK (I2S)"]
    #[inline(always)]
    pub fn gpa_mfp15(&mut self) -> GPA_MFP15_W {
        GPA_MFP15_W { w: self }
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOA\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOA\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpa_typen(&mut self) -> GPA_TYPEN_W {
        GPA_TYPEN_W { w: self }
    }
}
