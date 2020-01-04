#[doc = "Reader of register GPB_MFP"]
pub type R = crate::R<u32, super::GPB_MFP>;
#[doc = "Writer for register GPB_MFP"]
pub type W = crate::W<u32, super::GPB_MFP>;
#[doc = "Register GPB_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPB_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPB_MFP0`"]
pub type GPB_MFP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP0`"]
pub struct GPB_MFP0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP0_W<'a> {
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
#[doc = "Reader of field `GPB_MFP1`"]
pub type GPB_MFP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP1`"]
pub struct GPB_MFP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP1_W<'a> {
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
#[doc = "Reader of field `GPB_MFP2`"]
pub type GPB_MFP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP2`"]
pub struct GPB_MFP2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP2_W<'a> {
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
#[doc = "Reader of field `GPB_MFP3`"]
pub type GPB_MFP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP3`"]
pub struct GPB_MFP3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP3_W<'a> {
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
#[doc = "Reader of field `GPB_MFP4`"]
pub type GPB_MFP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP4`"]
pub struct GPB_MFP4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP4_W<'a> {
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
#[doc = "Reader of field `GPB_MFP5`"]
pub type GPB_MFP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP5`"]
pub struct GPB_MFP5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP5_W<'a> {
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
#[doc = "Reader of field `GPB_MFP6`"]
pub type GPB_MFP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP6`"]
pub struct GPB_MFP6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP6_W<'a> {
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
#[doc = "Reader of field `GPB_MFP7`"]
pub type GPB_MFP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP7`"]
pub struct GPB_MFP7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP7_W<'a> {
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
#[doc = "Reader of field `GPB_MFP8`"]
pub type GPB_MFP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP8`"]
pub struct GPB_MFP8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP8_W<'a> {
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
#[doc = "Reader of field `GPB_MFP9`"]
pub type GPB_MFP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP9`"]
pub struct GPB_MFP9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP9_W<'a> {
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
#[doc = "Reader of field `GPB_MFP10`"]
pub type GPB_MFP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP10`"]
pub struct GPB_MFP10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP10_W<'a> {
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
#[doc = "Reader of field `GPB_MFP11`"]
pub type GPB_MFP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP11`"]
pub struct GPB_MFP11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP11_W<'a> {
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
#[doc = "Reader of field `GPB_MFP12`"]
pub type GPB_MFP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP12`"]
pub struct GPB_MFP12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP12_W<'a> {
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
#[doc = "Reader of field `GPB_MFP13`"]
pub type GPB_MFP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP13`"]
pub struct GPB_MFP13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP13_W<'a> {
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
#[doc = "Reader of field `GPB_MFP14`"]
pub type GPB_MFP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP14`"]
pub struct GPB_MFP14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP14_W<'a> {
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
#[doc = "Reader of field `GPB_MFP15`"]
pub type GPB_MFP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPB_MFP15`"]
pub struct GPB_MFP15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_MFP15_W<'a> {
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
#[doc = "Reader of field `GPB_TYPEn`"]
pub type GPB_TYPEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPB_TYPEn`"]
pub struct GPB_TYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPB_TYPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PB.0 Pin Function Selection 1 = The UART0 RXD function is selected to the pin PB.0 0 = The GPIOB\\[0\\]
is selected to the pin PB.0"]
    #[inline(always)]
    pub fn gpb_mfp0(&self) -> GPB_MFP0_R {
        GPB_MFP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PB.1 Pin Function Selection 1 = The UART0 TXD function is selected to the pin PB.1 0 = The GPIOB\\[1\\]
is selected to the pin PB.1"]
    #[inline(always)]
    pub fn gpb_mfp1(&self) -> GPB_MFP1_R {
        GPB_MFP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PB.2 Pin Function Selection The pin function depends on GPB_MFP2 and EBI_nWRL_EN (ALT_MFP\\[13\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_nWRL_EN EBI_EN GPB_MFP\\[2\\]
PB.2 function x x 0 GPIO x 0 1 RTS0 (UART0) 0 1 1 RTS0 (UART0) 1 1 1 nWRL (EBI write low byte enable)"]
    #[inline(always)]
    pub fn gpb_mfp2(&self) -> GPB_MFP2_R {
        GPB_MFP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PB.3 Pin Function Selection The pin function depends on GPB_MFP3 and EBI_nWRH_EN (ALT_MFP\\[14\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_nWRH_EN EBI_EN GPB_MFP\\[3\\]
PB.3 function x x 0 GPIO x 0 1 CTS0 (UART0) 0 1 1 CTS0 (UART0) 1 1 1 nWRH (EBI write high byte enable)"]
    #[inline(always)]
    pub fn gpb_mfp3(&self) -> GPB_MFP3_R {
        GPB_MFP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PB.4 Pin Function Selection 1 = The UART1 RXD function is selected to the pin PB.4 0 = The GPIOB\\[4\\]
is selected to the pin PB.4"]
    #[inline(always)]
    pub fn gpb_mfp4(&self) -> GPB_MFP4_R {
        GPB_MFP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PB.5 Pin Function Selection 1 = The UART1 TXD function is selected to the pin PB.5 0 = The GPIOB\\[5\\]
is selected to the pin PB.5"]
    #[inline(always)]
    pub fn gpb_mfp5(&self) -> GPB_MFP5_R {
        GPB_MFP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB.6 Pin Function Selection The pin function depends on GPB_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[6\\]
PB.6 function x 0 GPIO 0 1 TRS1 (UART1) 1 1 ALE (EBI)"]
    #[inline(always)]
    pub fn gpb_mfp6(&self) -> GPB_MFP6_R {
        GPB_MFP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PB.7 Pin Function Selection The pin function depends on GPB_MFP7 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[7\\]
PB.7 function x 0 GPIO 0 1 CTS1 (UART1) 1 1 nCS (EBI)"]
    #[inline(always)]
    pub fn gpb_mfp7(&self) -> GPB_MFP7_R {
        GPB_MFP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PB.8 Pin Function Selection 1 = The TM0 (Timer/Counter external trigger clock input) function is selected to the pin PB.8 0 = The GPIOB\\[8\\]
is selected to the pin PB.8"]
    #[inline(always)]
    pub fn gpb_mfp8(&self) -> GPB_MFP8_R {
        GPB_MFP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PB.9 Pin Function Selection The pin function depends on GPB_MFP9 and PB9_S11 (ALT_MFP\\[1\\]). PB9_S11 GPB_MFP\\[9\\]
PB.9 function x 0 GPIO 0 1 TM1 1 1 SPISS11 (SPI1)"]
    #[inline(always)]
    pub fn gpb_mfp9(&self) -> GPB_MFP9_R {
        GPB_MFP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PB.10 Pin Function Selection The pin function depends on GPB_MFP10 and PB10_S01 (ALT_MFP\\[0\\]). PB10_S01 GPB_MFP\\[10\\]
PB.10 function x 0 GPIO 0 1 TM2 1 1 SPISS01 (SPI0)"]
    #[inline(always)]
    pub fn gpb_mfp10(&self) -> GPB_MFP10_R {
        GPB_MFP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PB.11 Pin Function Selection The pin function depends on GPB_MFP11 and PB11_PWM4 (ALT_MFP\\[4\\]). PB11_PWM4 GPB_MFP\\[11\\]
PB.11 function x 0 GPIO 0 1 TM3 1 1 PWM4 (PWM)"]
    #[inline(always)]
    pub fn gpb_mfp11(&self) -> GPB_MFP11_R {
        GPB_MFP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PB.12 Pin Function Selection The pin function depends on GPB_MFP12 and PB12_CLKO (ALT_MFP\\[10\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_EN PB12_CLKO GPB_MFP\\[12\\]
PB.12 function x x 0 GPIO 0 0 1 CPO0(CMP) 0 1 1 CLKO (Clock Driver output) 1 x 1 AD0(EBI AD bus bit 0)"]
    #[inline(always)]
    pub fn gpb_mfp12(&self) -> GPB_MFP12_R {
        GPB_MFP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PB.13 Pin Function Selection The pin function depends on GPB_MFP13 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[13\\]
PB.13 function x 0 GPIO 0 1 CPO1 (CMP) 1 1 AD1 (EBI AD bus bit 1)"]
    #[inline(always)]
    pub fn gpb_mfp13(&self) -> GPB_MFP13_R {
        GPB_MFP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PB.14 Pin Function Selection The pin function depends on GPB_MFP14 and PB14_S31 (ALT_MFP\\[3\\]). PB14_S31 GPB_MFP\\[14\\]
PB.14 function x 0 GPIO 0 1 /INT0 1 1 SPISS31 (SPI3)"]
    #[inline(always)]
    pub fn gpb_mfp14(&self) -> GPB_MFP14_R {
        GPB_MFP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PB.15 Pin Function Selection 1 = The External Interrupt INT1 function is selected to the pin PB.15 0 = The GPIOB\\[15\\]
is selected to the pin PB.15"]
    #[inline(always)]
    pub fn gpb_mfp15(&self) -> GPB_MFP15_R {
        GPB_MFP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOB\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOB\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpb_typen(&self) -> GPB_TYPEN_R {
        GPB_TYPEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PB.0 Pin Function Selection 1 = The UART0 RXD function is selected to the pin PB.0 0 = The GPIOB\\[0\\]
is selected to the pin PB.0"]
    #[inline(always)]
    pub fn gpb_mfp0(&mut self) -> GPB_MFP0_W {
        GPB_MFP0_W { w: self }
    }
    #[doc = "Bit 1 - PB.1 Pin Function Selection 1 = The UART0 TXD function is selected to the pin PB.1 0 = The GPIOB\\[1\\]
is selected to the pin PB.1"]
    #[inline(always)]
    pub fn gpb_mfp1(&mut self) -> GPB_MFP1_W {
        GPB_MFP1_W { w: self }
    }
    #[doc = "Bit 2 - PB.2 Pin Function Selection The pin function depends on GPB_MFP2 and EBI_nWRL_EN (ALT_MFP\\[13\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_nWRL_EN EBI_EN GPB_MFP\\[2\\]
PB.2 function x x 0 GPIO x 0 1 RTS0 (UART0) 0 1 1 RTS0 (UART0) 1 1 1 nWRL (EBI write low byte enable)"]
    #[inline(always)]
    pub fn gpb_mfp2(&mut self) -> GPB_MFP2_W {
        GPB_MFP2_W { w: self }
    }
    #[doc = "Bit 3 - PB.3 Pin Function Selection The pin function depends on GPB_MFP3 and EBI_nWRH_EN (ALT_MFP\\[14\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_nWRH_EN EBI_EN GPB_MFP\\[3\\]
PB.3 function x x 0 GPIO x 0 1 CTS0 (UART0) 0 1 1 CTS0 (UART0) 1 1 1 nWRH (EBI write high byte enable)"]
    #[inline(always)]
    pub fn gpb_mfp3(&mut self) -> GPB_MFP3_W {
        GPB_MFP3_W { w: self }
    }
    #[doc = "Bit 4 - PB.4 Pin Function Selection 1 = The UART1 RXD function is selected to the pin PB.4 0 = The GPIOB\\[4\\]
is selected to the pin PB.4"]
    #[inline(always)]
    pub fn gpb_mfp4(&mut self) -> GPB_MFP4_W {
        GPB_MFP4_W { w: self }
    }
    #[doc = "Bit 5 - PB.5 Pin Function Selection 1 = The UART1 TXD function is selected to the pin PB.5 0 = The GPIOB\\[5\\]
is selected to the pin PB.5"]
    #[inline(always)]
    pub fn gpb_mfp5(&mut self) -> GPB_MFP5_W {
        GPB_MFP5_W { w: self }
    }
    #[doc = "Bit 6 - PB.6 Pin Function Selection The pin function depends on GPB_MFP6 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[6\\]
PB.6 function x 0 GPIO 0 1 TRS1 (UART1) 1 1 ALE (EBI)"]
    #[inline(always)]
    pub fn gpb_mfp6(&mut self) -> GPB_MFP6_W {
        GPB_MFP6_W { w: self }
    }
    #[doc = "Bit 7 - PB.7 Pin Function Selection The pin function depends on GPB_MFP7 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[7\\]
PB.7 function x 0 GPIO 0 1 CTS1 (UART1) 1 1 nCS (EBI)"]
    #[inline(always)]
    pub fn gpb_mfp7(&mut self) -> GPB_MFP7_W {
        GPB_MFP7_W { w: self }
    }
    #[doc = "Bit 8 - PB.8 Pin Function Selection 1 = The TM0 (Timer/Counter external trigger clock input) function is selected to the pin PB.8 0 = The GPIOB\\[8\\]
is selected to the pin PB.8"]
    #[inline(always)]
    pub fn gpb_mfp8(&mut self) -> GPB_MFP8_W {
        GPB_MFP8_W { w: self }
    }
    #[doc = "Bit 9 - PB.9 Pin Function Selection The pin function depends on GPB_MFP9 and PB9_S11 (ALT_MFP\\[1\\]). PB9_S11 GPB_MFP\\[9\\]
PB.9 function x 0 GPIO 0 1 TM1 1 1 SPISS11 (SPI1)"]
    #[inline(always)]
    pub fn gpb_mfp9(&mut self) -> GPB_MFP9_W {
        GPB_MFP9_W { w: self }
    }
    #[doc = "Bit 10 - PB.10 Pin Function Selection The pin function depends on GPB_MFP10 and PB10_S01 (ALT_MFP\\[0\\]). PB10_S01 GPB_MFP\\[10\\]
PB.10 function x 0 GPIO 0 1 TM2 1 1 SPISS01 (SPI0)"]
    #[inline(always)]
    pub fn gpb_mfp10(&mut self) -> GPB_MFP10_W {
        GPB_MFP10_W { w: self }
    }
    #[doc = "Bit 11 - PB.11 Pin Function Selection The pin function depends on GPB_MFP11 and PB11_PWM4 (ALT_MFP\\[4\\]). PB11_PWM4 GPB_MFP\\[11\\]
PB.11 function x 0 GPIO 0 1 TM3 1 1 PWM4 (PWM)"]
    #[inline(always)]
    pub fn gpb_mfp11(&mut self) -> GPB_MFP11_W {
        GPB_MFP11_W { w: self }
    }
    #[doc = "Bit 12 - PB.12 Pin Function Selection The pin function depends on GPB_MFP12 and PB12_CLKO (ALT_MFP\\[10\\]) and EBI_EN (ALT_MFP\\[11\\]). EBI_EN PB12_CLKO GPB_MFP\\[12\\]
PB.12 function x x 0 GPIO 0 0 1 CPO0(CMP) 0 1 1 CLKO (Clock Driver output) 1 x 1 AD0(EBI AD bus bit 0)"]
    #[inline(always)]
    pub fn gpb_mfp12(&mut self) -> GPB_MFP12_W {
        GPB_MFP12_W { w: self }
    }
    #[doc = "Bit 13 - PB.13 Pin Function Selection The pin function depends on GPB_MFP13 and EBI_EN (ALT_MFP\\[11\\]). EBI_EN GPB_MFP\\[13\\]
PB.13 function x 0 GPIO 0 1 CPO1 (CMP) 1 1 AD1 (EBI AD bus bit 1)"]
    #[inline(always)]
    pub fn gpb_mfp13(&mut self) -> GPB_MFP13_W {
        GPB_MFP13_W { w: self }
    }
    #[doc = "Bit 14 - PB.14 Pin Function Selection The pin function depends on GPB_MFP14 and PB14_S31 (ALT_MFP\\[3\\]). PB14_S31 GPB_MFP\\[14\\]
PB.14 function x 0 GPIO 0 1 /INT0 1 1 SPISS31 (SPI3)"]
    #[inline(always)]
    pub fn gpb_mfp14(&mut self) -> GPB_MFP14_W {
        GPB_MFP14_W { w: self }
    }
    #[doc = "Bit 15 - PB.15 Pin Function Selection 1 = The External Interrupt INT1 function is selected to the pin PB.15 0 = The GPIOB\\[15\\]
is selected to the pin PB.15"]
    #[inline(always)]
    pub fn gpb_mfp15(&mut self) -> GPB_MFP15_W {
        GPB_MFP15_W { w: self }
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOB\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOB\\[15:0\\]
I/O input Schmitt Trigger function"]
    #[inline(always)]
    pub fn gpb_typen(&mut self) -> GPB_TYPEN_W {
        GPB_TYPEN_W { w: self }
    }
}
