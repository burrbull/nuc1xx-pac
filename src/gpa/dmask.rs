#[doc = "Reader of register DMASK"]
pub type R = crate::R<u32, super::DMASK>;
#[doc = "Writer for register DMASK"]
pub type W = crate::W<u32, super::DMASK>;
#[doc = "Register DMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASK_A {
    #[doc = "0: GPIOx_DOUT\\[n\\]
bit can be updated"]
    UPDATABLE = 0,
    #[doc = "1: GPIOx_DOUT\\[n\\]
bit is protected"]
    PROTECTED = 1,
}
impl From<DMASK_A> for bool {
    #[inline(always)]
    fn from(variant: DMASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMASK[%s]`"]
pub type DMASK_R = crate::R<bool, DMASK_A>;
impl DMASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASK_A {
        match self.bits {
            false => DMASK_A::UPDATABLE,
            true => DMASK_A::PROTECTED,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATABLE`"]
    #[inline(always)]
    pub fn is_updatable(&self) -> bool {
        *self == DMASK_A::UPDATABLE
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == DMASK_A::PROTECTED
    }
}
#[doc = "Write proxy for fields `DMASK(0-15)`"]
pub struct DMASK_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> DMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn updatable(self) -> &'a mut W {
        self.variant(DMASK_A::UPDATABLE)
    }
    #[doc = "GPIOx_DOUT\\[n\\]
bit is protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(DMASK_A::PROTECTED)
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
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub unsafe fn dmask(&self, n: usize) -> DMASK_R {
        DMASK_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask0(&self) -> DMASK_R {
        DMASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask1(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask2(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask3(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask4(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask5(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask6(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask7(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask8(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask9(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask10(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask11(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask12(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask13(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask14(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask15(&self) -> DMASK_R {
        DMASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub unsafe fn dmask(&mut self, n: usize) -> DMASK_W {
        DMASK_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask0(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask1(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask2(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask3(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask4(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask5(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask6(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask7(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask8(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask9(&mut self) -> DMASK_W {
        DMASK_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask10(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask11(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask12(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask13(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask14(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored"]
    #[inline(always)]
    pub fn dmask15(&mut self) -> DMASK_W {
        DMASK_W {
            w: self,
            offset: 15,
        }
    }
}
