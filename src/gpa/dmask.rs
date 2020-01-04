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
#[doc = "Reader of field `DMASK0`"]
pub type DMASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK0`"]
pub struct DMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK0_W<'a> {
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
#[doc = "Reader of field `DMASK1`"]
pub type DMASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK1`"]
pub struct DMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK1_W<'a> {
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
#[doc = "Reader of field `DMASK2`"]
pub type DMASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK2`"]
pub struct DMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK2_W<'a> {
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
#[doc = "Reader of field `DMASK3`"]
pub type DMASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK3`"]
pub struct DMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK3_W<'a> {
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
#[doc = "Reader of field `DMASK4`"]
pub type DMASK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK4`"]
pub struct DMASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK4_W<'a> {
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
#[doc = "Reader of field `DMASK5`"]
pub type DMASK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK5`"]
pub struct DMASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK5_W<'a> {
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
#[doc = "Reader of field `DMASK6`"]
pub type DMASK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK6`"]
pub struct DMASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK6_W<'a> {
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
#[doc = "Reader of field `DMASK7`"]
pub type DMASK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK7`"]
pub struct DMASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK7_W<'a> {
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
#[doc = "Reader of field `DMASK8`"]
pub type DMASK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK8`"]
pub struct DMASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK8_W<'a> {
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
#[doc = "Reader of field `DMASK9`"]
pub type DMASK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK9`"]
pub struct DMASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK9_W<'a> {
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
#[doc = "Reader of field `DMASK10`"]
pub type DMASK10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK10`"]
pub struct DMASK10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK10_W<'a> {
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
#[doc = "Reader of field `DMASK11`"]
pub type DMASK11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK11`"]
pub struct DMASK11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK11_W<'a> {
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
#[doc = "Reader of field `DMASK12`"]
pub type DMASK12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK12`"]
pub struct DMASK12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK12_W<'a> {
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
#[doc = "Reader of field `DMASK13`"]
pub type DMASK13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK13`"]
pub struct DMASK13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK13_W<'a> {
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
#[doc = "Reader of field `DMASK14`"]
pub type DMASK14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK14`"]
pub struct DMASK14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK14_W<'a> {
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
#[doc = "Reader of field `DMASK15`"]
pub type DMASK15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASK15`"]
pub struct DMASK15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK15_W<'a> {
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
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask0(&self) -> DMASK0_R {
        DMASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask1(&self) -> DMASK1_R {
        DMASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask2(&self) -> DMASK2_R {
        DMASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask3(&self) -> DMASK3_R {
        DMASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask4(&self) -> DMASK4_R {
        DMASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask5(&self) -> DMASK5_R {
        DMASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask6(&self) -> DMASK6_R {
        DMASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask7(&self) -> DMASK7_R {
        DMASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask8(&self) -> DMASK8_R {
        DMASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask9(&self) -> DMASK9_R {
        DMASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask10(&self) -> DMASK10_R {
        DMASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask11(&self) -> DMASK11_R {
        DMASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask12(&self) -> DMASK12_R {
        DMASK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask13(&self) -> DMASK13_R {
        DMASK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask14(&self) -> DMASK14_R {
        DMASK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask15(&self) -> DMASK15_R {
        DMASK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask0(&mut self) -> DMASK0_W {
        DMASK0_W { w: self }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask1(&mut self) -> DMASK1_W {
        DMASK1_W { w: self }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask2(&mut self) -> DMASK2_W {
        DMASK2_W { w: self }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask3(&mut self) -> DMASK3_W {
        DMASK3_W { w: self }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask4(&mut self) -> DMASK4_W {
        DMASK4_W { w: self }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask5(&mut self) -> DMASK5_W {
        DMASK5_W { w: self }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask6(&mut self) -> DMASK6_W {
        DMASK6_W { w: self }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask7(&mut self) -> DMASK7_W {
        DMASK7_W { w: self }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask8(&mut self) -> DMASK8_W {
        DMASK8_W { w: self }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask9(&mut self) -> DMASK9_W {
        DMASK9_W { w: self }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask10(&mut self) -> DMASK10_W {
        DMASK10_W { w: self }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask11(&mut self) -> DMASK11_W {
        DMASK11_W { w: self }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask12(&mut self) -> DMASK12_W {
        DMASK12_W { w: self }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask13(&mut self) -> DMASK13_W {
        DMASK13_W { w: self }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask14(&mut self) -> DMASK14_W {
        DMASK14_W { w: self }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Data Output Write Mask These bits are used to protect the corresponding register of GPIOx_DOUT bit\\[n\\]. When set the DMASK bit\\[n\\]
to\"1\", the corresponding GPIOx_DOUTn bit is protected. The write signal is masked, write data to the protect bit is ignored 1 = The corresponding GPIOx_DOUT\\[n\\]
bit is protected 0 = The corresponding GPIOx_DOUT\\[n\\]
bit can be updated"]
    #[inline(always)]
    pub fn dmask15(&mut self) -> DMASK15_W {
        DMASK15_W { w: self }
    }
}
