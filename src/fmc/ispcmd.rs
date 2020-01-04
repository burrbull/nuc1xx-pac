#[doc = "Reader of register ISPCMD"]
pub type R = crate::R<u32, super::ISPCMD>;
#[doc = "Writer for register ISPCMD"]
pub type W = crate::W<u32, super::ISPCMD>;
#[doc = "Register ISPCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCTRL`"]
pub type FCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCTRL`"]
pub struct FCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FCEN`"]
pub type FCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCEN`"]
pub struct FCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEN_W<'a> {
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
#[doc = "Reader of field `FOEN`"]
pub type FOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOEN`"]
pub struct FOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FOEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn fctrl(&self) -> FCTRL_R {
        FCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn fcen(&self) -> FCEN_R {
        FCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn foen(&self) -> FOEN_R {
        FOEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn fctrl(&mut self) -> FCTRL_W {
        FCTRL_W { w: self }
    }
    #[doc = "Bit 4 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn fcen(&mut self) -> FCEN_W {
        FCEN_W { w: self }
    }
    #[doc = "Bit 5 - ISP Command ISP command table is showed below: Operation Mode FOEN FCEN FCTRL\\[3:0\\]
Read 0 0 0 0 0 0 Program 1 0 0 0 0 1 Page Erase 1 0 0 0 1 0"]
    #[inline(always)]
    pub fn foen(&mut self) -> FOEN_W {
        FOEN_W { w: self }
    }
}
