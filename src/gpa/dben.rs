#[doc = "Reader of register DBEN"]
pub type R = crate::R<u32, super::DBEN>;
#[doc = "Writer for register DBEN"]
pub type W = crate::W<u32, super::DBEN>;
#[doc = "Register DBEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DBEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN_A {
    #[doc = "0: The bit\\[n\\]
de-bounce function is disabled"]
    DISABLED = 0,
    #[doc = "1: The bit\\[n\\]
de-bounce function is enabled"]
    ENABLED = 1,
}
impl From<DBEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBEN[%s]`"]
pub type DBEN_R = crate::R<bool, DBEN_A>;
impl DBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN_A {
        match self.bits {
            false => DBEN_A::DISABLED,
            true => DBEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBEN_A::ENABLED
    }
}
#[doc = "Write proxy for fields `DBEN(0-15)`"]
pub struct DBEN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> DBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The bit\\[n\\]
de-bounce function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBEN_A::DISABLED)
    }
    #[doc = "The bit\\[n\\]
de-bounce function is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBEN_A::ENABLED)
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
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub unsafe fn dben(&self, n: usize) -> DBEN_R {
        DBEN_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben0(&self) -> DBEN_R {
        DBEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben1(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben2(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben3(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben4(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben5(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben6(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben7(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben8(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben9(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben10(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben11(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben12(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben13(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben14(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben15(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub unsafe fn dben(&mut self, n: usize) -> DBEN_W {
        DBEN_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben0(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben1(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben2(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben3(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben4(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben5(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben6(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben7(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben8(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben9(&mut self) -> DBEN_W {
        DBEN_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben10(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben11(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben12(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben13(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben14(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben15(&mut self) -> DBEN_W {
        DBEN_W {
            w: self,
            offset: 15,
        }
    }
}
