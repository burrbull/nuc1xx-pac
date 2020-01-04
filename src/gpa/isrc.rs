#[doc = "Reader of register ISRC"]
pub type R = crate::R<u32, super::ISRC>;
#[doc = "Writer for register ISRC"]
pub type W = crate::W<u32, super::ISRC>;
#[doc = "Register ISRC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISRC0`"]
pub type ISRC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC0`"]
pub struct ISRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC0_W<'a> {
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
#[doc = "Reader of field `ISRC1`"]
pub type ISRC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC1`"]
pub struct ISRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC1_W<'a> {
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
#[doc = "Reader of field `ISRC2`"]
pub type ISRC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC2`"]
pub struct ISRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC2_W<'a> {
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
#[doc = "Reader of field `ISRC3`"]
pub type ISRC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC3`"]
pub struct ISRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC3_W<'a> {
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
#[doc = "Reader of field `ISRC4`"]
pub type ISRC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC4`"]
pub struct ISRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC4_W<'a> {
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
#[doc = "Reader of field `ISRC5`"]
pub type ISRC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC5`"]
pub struct ISRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC5_W<'a> {
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
#[doc = "Reader of field `ISRC6`"]
pub type ISRC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC6`"]
pub struct ISRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC6_W<'a> {
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
#[doc = "Reader of field `ISRC7`"]
pub type ISRC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC7`"]
pub struct ISRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC7_W<'a> {
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
#[doc = "Reader of field `ISRC8`"]
pub type ISRC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC8`"]
pub struct ISRC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC8_W<'a> {
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
#[doc = "Reader of field `ISRC9`"]
pub type ISRC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC9`"]
pub struct ISRC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC9_W<'a> {
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
#[doc = "Reader of field `ISRC10`"]
pub type ISRC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC10`"]
pub struct ISRC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC10_W<'a> {
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
#[doc = "Reader of field `ISRC11`"]
pub type ISRC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC11`"]
pub struct ISRC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC11_W<'a> {
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
#[doc = "Reader of field `ISRC12`"]
pub type ISRC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC12`"]
pub struct ISRC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC12_W<'a> {
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
#[doc = "Reader of field `ISRC13`"]
pub type ISRC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC13`"]
pub struct ISRC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC13_W<'a> {
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
#[doc = "Reader of field `ISRC14`"]
pub type ISRC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC14`"]
pub struct ISRC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC14_W<'a> {
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
#[doc = "Reader of field `ISRC15`"]
pub type ISRC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRC15`"]
pub struct ISRC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRC15_W<'a> {
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
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc0(&self) -> ISRC0_R {
        ISRC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc1(&self) -> ISRC1_R {
        ISRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc2(&self) -> ISRC2_R {
        ISRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc3(&self) -> ISRC3_R {
        ISRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc4(&self) -> ISRC4_R {
        ISRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc5(&self) -> ISRC5_R {
        ISRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc6(&self) -> ISRC6_R {
        ISRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc7(&self) -> ISRC7_R {
        ISRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc8(&self) -> ISRC8_R {
        ISRC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc9(&self) -> ISRC9_R {
        ISRC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc10(&self) -> ISRC10_R {
        ISRC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc11(&self) -> ISRC11_R {
        ISRC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc12(&self) -> ISRC12_R {
        ISRC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc13(&self) -> ISRC13_R {
        ISRC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc14(&self) -> ISRC14_R {
        ISRC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc15(&self) -> ISRC15_R {
        ISRC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc0(&mut self) -> ISRC0_W {
        ISRC0_W { w: self }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc1(&mut self) -> ISRC1_W {
        ISRC1_W { w: self }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc2(&mut self) -> ISRC2_W {
        ISRC2_W { w: self }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc3(&mut self) -> ISRC3_W {
        ISRC3_W { w: self }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc4(&mut self) -> ISRC4_W {
        ISRC4_W { w: self }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc5(&mut self) -> ISRC5_W {
        ISRC5_W { w: self }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc6(&mut self) -> ISRC6_W {
        ISRC6_W { w: self }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc7(&mut self) -> ISRC7_W {
        ISRC7_W { w: self }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc8(&mut self) -> ISRC8_W {
        ISRC8_W { w: self }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc9(&mut self) -> ISRC9_W {
        ISRC9_W { w: self }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc10(&mut self) -> ISRC10_W {
        ISRC10_W { w: self }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc11(&mut self) -> ISRC11_W {
        ISRC11_W { w: self }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc12(&mut self) -> ISRC12_W {
        ISRC12_W { w: self }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc13(&mut self) -> ISRC13_W {
        ISRC13_W { w: self }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc14(&mut self) -> ISRC14_W {
        ISRC14_W { w: self }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator Read : 1 = Indicates GPIOx\\[n\\]
generate an interrupt 0 = No interrupt at GPIOx\\[n\\]
Write : 1= Clear the correspond pending interrupt 0= No action"]
    #[inline(always)]
    pub fn isrc15(&mut self) -> ISRC15_W {
        ISRC15_W { w: self }
    }
}
