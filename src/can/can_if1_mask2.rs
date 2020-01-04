#[doc = "Reader of register CAN_IF1_MASK2"]
pub type R = crate::R<u32, super::CAN_IF1_MASK2>;
#[doc = "Writer for register CAN_IF1_MASK2"]
pub type W = crate::W<u32, super::CAN_IF1_MASK2>;
#[doc = "Register CAN_IF1_MASK2 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CAN_IF1_MASK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `Msk_16_28`"]
pub type MSK_16_28_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Msk_16_28`"]
pub struct MSK_16_28_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_16_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `MDir`"]
pub type MDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDir`"]
pub struct MDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIR_W<'a> {
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
#[doc = "Reader of field `MXtd`"]
pub type MXTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MXtd`"]
pub struct MXTD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXTD_W<'a> {
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
    #[doc = "Bits 0:12 - Identifier Mask 28-16 1 = The corresponding identifier bit is used for acceptance filtering. 0 = The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering."]
    #[inline(always)]
    pub fn msk_16_28(&self) -> MSK_16_28_R {
        MSK_16_28_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask Message Direction 1 = The message direction bit (Dir) is used for acceptance filtering. 0 = The message direction bit (Dir) has no effect on the acceptance filtering."]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask Extended Identifier 1 = The extended identifier bit (IDE) is used for acceptance filtering. 0 = The extended identifier bit (IDE) has no effect on the acceptance filtering. Note: When 11-bit (\"standard\") Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18. For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 are considered."]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier Mask 28-16 1 = The corresponding identifier bit is used for acceptance filtering. 0 = The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering."]
    #[inline(always)]
    pub fn msk_16_28(&mut self) -> MSK_16_28_W {
        MSK_16_28_W { w: self }
    }
    #[doc = "Bit 14 - Mask Message Direction 1 = The message direction bit (Dir) is used for acceptance filtering. 0 = The message direction bit (Dir) has no effect on the acceptance filtering."]
    #[inline(always)]
    pub fn mdir(&mut self) -> MDIR_W {
        MDIR_W { w: self }
    }
    #[doc = "Bit 15 - Mask Extended Identifier 1 = The extended identifier bit (IDE) is used for acceptance filtering. 0 = The extended identifier bit (IDE) has no effect on the acceptance filtering. Note: When 11-bit (\"standard\") Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18. For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 are considered."]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MXTD_W {
        MXTD_W { w: self }
    }
}
