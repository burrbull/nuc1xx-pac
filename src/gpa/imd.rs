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
#[doc = "Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMD_A {
    #[doc = "0: Edge trigger interrupt"]
    EDGE = 0,
    #[doc = "1: Level trigger interrupt"]
    LEVEL = 1,
}
impl From<IMD_A> for bool {
    #[inline(always)]
    fn from(variant: IMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMD[%s]`"]
pub type IMD_R = crate::R<bool, IMD_A>;
impl IMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMD_A {
        match self.bits {
            false => IMD_A::EDGE,
            true => IMD_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IMD_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == IMD_A::LEVEL
    }
}
#[doc = "Write proxy for fields `IMD(0-15)`"]
pub struct IMD_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> IMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IMD_A::EDGE)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(IMD_A::LEVEL)
    }
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | (((value as u32) & 0x01) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub unsafe fn imd(&self, n: usize) -> IMD_R {
        IMD_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd0(&self) -> IMD_R {
        IMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd1(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd2(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd3(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd4(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd5(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd6(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd7(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd8(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd9(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd10(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd11(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd12(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd13(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd14(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd15(&self) -> IMD_R {
        IMD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub unsafe fn imd(&mut self, n: usize) -> IMD_W {
        IMD_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd0(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd1(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd2(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd3(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd4(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd5(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd6(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd7(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd8(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd9(&mut self) -> IMD_W {
        IMD_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd10(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd11(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd12(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd13(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd14(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Edge or Level Detection Interrupt Control IMD\\[n\\]
is used to control the interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrup. If set pin as the level trigger interrupt, then only one level can be set on the registers GPIOx_IEN. If set both the level to trigger interrupt, the setting is ignored and no interrupt will occur The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn imd15(&mut self) -> IMD_W {
        IMD_W {
            w: self,
            offset: 15,
        }
    }
}
