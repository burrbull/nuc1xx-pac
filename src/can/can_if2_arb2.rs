#[doc = "Reader of register CAN_IF2_ARB2"]
pub type R = crate::R<u32, super::CAN_IF2_ARB2>;
#[doc = "Writer for register CAN_IF2_ARB2"]
pub type W = crate::W<u32, super::CAN_IF2_ARB2>;
#[doc = "Register CAN_IF2_ARB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF2_ARB2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_16_28`"]
pub type ID_16_28_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ID_16_28`"]
pub struct ID_16_28_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_16_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `Dir`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Dir`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `Xtd`"]
pub type XTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Xtd`"]
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
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
#[doc = "Reader of field `MsgVal`"]
pub type MSGVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MsgVal`"]
pub struct MSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGVAL_W<'a> {
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
    #[doc = "Bits 0:12 - Message Identifier 28-16 ID28 - ID0, 29-bit Identifier (\"Extended Frame\"). ID28 - ID18, 11-bit Identifier (\"Standard Frame\")"]
    #[inline(always)]
    pub fn id_16_28(&self) -> ID_16_28_R {
        ID_16_28_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Message Direction 1 = Direction is transmit On TxRqst, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TxRqst bit of this Message Object is set (if RmtEn = one). 0 = Direction is receive On TxRqst, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Extended Identifier 1 = The 29-bit (\"extended\") Identifier will be used for this Message Object. 0 = The 11-bit (\"standard\") Identifier will be used for this Message Object."]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Message Valid 1 = The Message Object is configured and should be considered by the Message Handler. 0 = The Message Object is ignored by the Message Handler. Note: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init in the CAN Control Register. This bit must also be reset before the identifier Id28-0, the control bits Xtd, Dir, or the Data Length Code DLC3-0 are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msg_val(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Message Identifier 28-16 ID28 - ID0, 29-bit Identifier (\"Extended Frame\"). ID28 - ID18, 11-bit Identifier (\"Standard Frame\")"]
    #[inline(always)]
    pub fn id_16_28(&mut self) -> ID_16_28_W {
        ID_16_28_W { w: self }
    }
    #[doc = "Bit 13 - Message Direction 1 = Direction is transmit On TxRqst, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TxRqst bit of this Message Object is set (if RmtEn = one). 0 = Direction is receive On TxRqst, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 14 - Extended Identifier 1 = The 29-bit (\"extended\") Identifier will be used for this Message Object. 0 = The 11-bit (\"standard\") Identifier will be used for this Message Object."]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    #[doc = "Bit 15 - Message Valid 1 = The Message Object is configured and should be considered by the Message Handler. 0 = The Message Object is ignored by the Message Handler. Note: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init in the CAN Control Register. This bit must also be reset before the identifier Id28-0, the control bits Xtd, Dir, or the Data Length Code DLC3-0 are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msg_val(&mut self) -> MSGVAL_W {
        MSGVAL_W { w: self }
    }
}
