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
#[doc = "Reader of field `DBEN0`"]
pub type DBEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN0`"]
pub struct DBEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN0_W<'a> {
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
#[doc = "Reader of field `DBEN1`"]
pub type DBEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN1`"]
pub struct DBEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN1_W<'a> {
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
#[doc = "Reader of field `DBEN2`"]
pub type DBEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN2`"]
pub struct DBEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN2_W<'a> {
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
#[doc = "Reader of field `DBEN3`"]
pub type DBEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN3`"]
pub struct DBEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN3_W<'a> {
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
#[doc = "Reader of field `DBEN4`"]
pub type DBEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN4`"]
pub struct DBEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN4_W<'a> {
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
#[doc = "Reader of field `DBEN5`"]
pub type DBEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN5`"]
pub struct DBEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN5_W<'a> {
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
#[doc = "Reader of field `DBEN6`"]
pub type DBEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN6`"]
pub struct DBEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN6_W<'a> {
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
#[doc = "Reader of field `DBEN7`"]
pub type DBEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN7`"]
pub struct DBEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN7_W<'a> {
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
#[doc = "Reader of field `DBEN8`"]
pub type DBEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN8`"]
pub struct DBEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN8_W<'a> {
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
#[doc = "Reader of field `DBEN9`"]
pub type DBEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN9`"]
pub struct DBEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN9_W<'a> {
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
#[doc = "Reader of field `DBEN10`"]
pub type DBEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN10`"]
pub struct DBEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN10_W<'a> {
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
#[doc = "Reader of field `DBEN11`"]
pub type DBEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN11`"]
pub struct DBEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN11_W<'a> {
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
#[doc = "Reader of field `DBEN12`"]
pub type DBEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN12`"]
pub struct DBEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN12_W<'a> {
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
#[doc = "Reader of field `DBEN13`"]
pub type DBEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN13`"]
pub struct DBEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN13_W<'a> {
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
#[doc = "Reader of field `DBEN14`"]
pub type DBEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN14`"]
pub struct DBEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN14_W<'a> {
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
#[doc = "Reader of field `DBEN15`"]
pub type DBEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN15`"]
pub struct DBEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN15_W<'a> {
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
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben0(&self) -> DBEN0_R {
        DBEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben1(&self) -> DBEN1_R {
        DBEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben2(&self) -> DBEN2_R {
        DBEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben3(&self) -> DBEN3_R {
        DBEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben4(&self) -> DBEN4_R {
        DBEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben5(&self) -> DBEN5_R {
        DBEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben6(&self) -> DBEN6_R {
        DBEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben7(&self) -> DBEN7_R {
        DBEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben8(&self) -> DBEN8_R {
        DBEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben9(&self) -> DBEN9_R {
        DBEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben10(&self) -> DBEN10_R {
        DBEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben11(&self) -> DBEN11_R {
        DBEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben12(&self) -> DBEN12_R {
        DBEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben13(&self) -> DBEN13_R {
        DBEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben14(&self) -> DBEN14_R {
        DBEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben15(&self) -> DBEN15_R {
        DBEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben0(&mut self) -> DBEN0_W {
        DBEN0_W { w: self }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben1(&mut self) -> DBEN1_W {
        DBEN1_W { w: self }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben2(&mut self) -> DBEN2_W {
        DBEN2_W { w: self }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben3(&mut self) -> DBEN3_W {
        DBEN3_W { w: self }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben4(&mut self) -> DBEN4_W {
        DBEN4_W { w: self }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben5(&mut self) -> DBEN5_W {
        DBEN5_W { w: self }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben6(&mut self) -> DBEN6_W {
        DBEN6_W { w: self }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben7(&mut self) -> DBEN7_W {
        DBEN7_W { w: self }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben8(&mut self) -> DBEN8_W {
        DBEN8_W { w: self }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben9(&mut self) -> DBEN9_W {
        DBEN9_W { w: self }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben10(&mut self) -> DBEN10_W {
        DBEN10_W { w: self }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben11(&mut self) -> DBEN11_W {
        DBEN11_W { w: self }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben12(&mut self) -> DBEN12_W {
        DBEN12_W { w: self }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben13(&mut self) -> DBEN13_W {
        DBEN13_W { w: self }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben14(&mut self) -> DBEN14_W {
        DBEN14_W { w: self }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Input Signal De-bounce Enable DBEN\\[n\\]used to enable the de-bounce function for each corresponding bit. If the input signal pulse width can't be sampled by continuous two de-bounce sample cycle The input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBNCECON\\[4\\], one de-bounce sample cycle is controlled by DBNCECON\\[3:0\\]
The DBEN\\[n\\]
is used for \"edge-trigger\" interrupt only, and ignored for \"level trigger\" interrupt 1 = The bit\\[n\\]
de-bounce function is enabled 0 = The bit\\[n\\]
de-bounce function is disabled The de-bounce function is valid for edge triggered interrupt. If the interrupt mode is level triggered, the de-bounce enable bit is ignored."]
    #[inline(always)]
    pub fn dben15(&mut self) -> DBEN15_W {
        DBEN15_W { w: self }
    }
}
