#[doc = "Reader of register IMD"]
pub type R = crate::R<u32, super::IMD>;
#[doc = "Writer for register IMD"]
pub type W = crate::W<u32, super::IMD>;
#[doc = "Register IMD `reset()`'s with value 0"]
impl crate::ResetValue for super::IMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMD0`"]
pub type IMD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD0`"]
pub struct IMD0_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD0_W<'a> {
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
#[doc = "Reader of field `IMD1`"]
pub type IMD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD1`"]
pub struct IMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD1_W<'a> {
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
#[doc = "Reader of field `IMD2`"]
pub type IMD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD2`"]
pub struct IMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD2_W<'a> {
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
#[doc = "Reader of field `IMD3`"]
pub type IMD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD3`"]
pub struct IMD3_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD3_W<'a> {
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
#[doc = "Reader of field `IMD4`"]
pub type IMD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD4`"]
pub struct IMD4_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD4_W<'a> {
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
#[doc = "Reader of field `IMD5`"]
pub type IMD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD5`"]
pub struct IMD5_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD5_W<'a> {
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
#[doc = "Reader of field `IMD6`"]
pub type IMD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD6`"]
pub struct IMD6_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD6_W<'a> {
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
#[doc = "Reader of field `IMD7`"]
pub type IMD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD7`"]
pub struct IMD7_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD7_W<'a> {
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
#[doc = "Reader of field `IMD8`"]
pub type IMD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD8`"]
pub struct IMD8_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD8_W<'a> {
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
#[doc = "Reader of field `IMD9`"]
pub type IMD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD9`"]
pub struct IMD9_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD9_W<'a> {
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
#[doc = "Reader of field `IMD10`"]
pub type IMD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD10`"]
pub struct IMD10_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD10_W<'a> {
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
#[doc = "Reader of field `IMD11`"]
pub type IMD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD11`"]
pub struct IMD11_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD11_W<'a> {
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
#[doc = "Reader of field `IMD12`"]
pub type IMD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD12`"]
pub struct IMD12_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD12_W<'a> {
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
#[doc = "Reader of field `IMD13`"]
pub type IMD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD13`"]
pub struct IMD13_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD13_W<'a> {
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
#[doc = "Reader of field `IMD14`"]
pub type IMD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD14`"]
pub struct IMD14_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD14_W<'a> {
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
#[doc = "Reader of field `IMD15`"]
pub type IMD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMD15`"]
pub struct IMD15_W<'a> {
    w: &'a mut W,
}
impl<'a> IMD15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd0(&self) -> IMD0_R {
        IMD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd1(&self) -> IMD1_R {
        IMD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd2(&self) -> IMD2_R {
        IMD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd3(&self) -> IMD3_R {
        IMD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd4(&self) -> IMD4_R {
        IMD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd5(&self) -> IMD5_R {
        IMD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd6(&self) -> IMD6_R {
        IMD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd7(&self) -> IMD7_R {
        IMD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd8(&self) -> IMD8_R {
        IMD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd9(&self) -> IMD9_R {
        IMD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd10(&self) -> IMD10_R {
        IMD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd11(&self) -> IMD11_R {
        IMD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd12(&self) -> IMD12_R {
        IMD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd13(&self) -> IMD13_R {
        IMD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd14(&self) -> IMD14_R {
        IMD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd15(&self) -> IMD15_R {
        IMD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd0(&mut self) -> IMD0_W {
        IMD0_W { w: self }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd1(&mut self) -> IMD1_W {
        IMD1_W { w: self }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd2(&mut self) -> IMD2_W {
        IMD2_W { w: self }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd3(&mut self) -> IMD3_W {
        IMD3_W { w: self }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd4(&mut self) -> IMD4_W {
        IMD4_W { w: self }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd5(&mut self) -> IMD5_W {
        IMD5_W { w: self }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd6(&mut self) -> IMD6_W {
        IMD6_W { w: self }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd7(&mut self) -> IMD7_W {
        IMD7_W { w: self }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd8(&mut self) -> IMD8_W {
        IMD8_W { w: self }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd9(&mut self) -> IMD9_W {
        IMD9_W { w: self }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd10(&mut self) -> IMD10_W {
        IMD10_W { w: self }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd11(&mut self) -> IMD11_W {
        IMD11_W { w: self }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd12(&mut self) -> IMD12_W {
        IMD12_W { w: self }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd13(&mut self) -> IMD13_W {
        IMD13_W { w: self }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd14(&mut self) -> IMD14_W {
        IMD14_W { w: self }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. 1 = Level trigger interrupt 0 = Edge trigger interrupt If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd15(&mut self) -> IMD15_W {
        IMD15_W { w: self }
    }
}
