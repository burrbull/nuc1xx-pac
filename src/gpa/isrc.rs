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
#[doc = "Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRC_A {
    #[doc = "0: No interrupt at GPIOx\\[n\\]"]
    NO_INTERRUPT = 0,
    #[doc = "1: Indicates GPIOx\\[n\\]
generate an interrupt"]
    INTERRUPT = 1,
}
impl From<ISRC_A> for bool {
    #[inline(always)]
    fn from(variant: ISRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISRC[%s]`"]
pub type ISRC_R = crate::R<bool, ISRC_A>;
impl ISRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRC_A {
        match self.bits {
            false => ISRC_A::NO_INTERRUPT,
            true => ISRC_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == ISRC_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ISRC_A::INTERRUPT
    }
}
#[doc = "Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRC_AW {
    #[doc = "1: Clear the correspond pending interrupt"]
    CLEAR = 1,
}
impl From<ISRC_AW> for bool {
    #[inline(always)]
    fn from(variant: ISRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for fields `ISRC(0-15)`"]
pub struct ISRC_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> ISRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISRC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the correspond pending interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISRC_AW::CLEAR)
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
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub unsafe fn isrc(&self, n: usize) -> ISRC_R {
        ISRC_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc0(&self) -> ISRC_R {
        ISRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc1(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc2(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc3(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc4(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc5(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc6(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc7(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc8(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc9(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc10(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc11(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc12(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc13(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc14(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc15(&self) -> ISRC_R {
        ISRC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub unsafe fn isrc(&mut self, n: usize) -> ISRC_W {
        ISRC_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc0(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc1(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc2(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc3(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc4(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc5(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc6(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc7(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc8(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc9(&mut self) -> ISRC_W {
        ISRC_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc10(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc11(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc12(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc13(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc14(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Trigger Source Indicator"]
    #[inline(always)]
    pub fn isrc15(&mut self) -> ISRC_W {
        ISRC_W {
            w: self,
            offset: 15,
        }
    }
}
