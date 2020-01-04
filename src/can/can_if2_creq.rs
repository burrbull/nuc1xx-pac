#[doc = "Reader of register CAN_IF2_CREQ"]
pub type R = crate::R<u32, super::CAN_IF2_CREQ>;
#[doc = "Writer for register CAN_IF2_CREQ"]
pub type W = crate::W<u32, super::CAN_IF2_CREQ>;
#[doc = "Register CAN_IF2_CREQ `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CAN_IF2_CREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MessageNumber`"]
pub type MESSAGENUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MessageNumber`"]
pub struct MESSAGENUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> MESSAGENUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `Busy`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Busy`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
    #[doc = "Bits 0:5 - Message Number 0x01-0x20: Valid Message Number, the Message Object in the Message RAM is selected for data transfer. 0x00: Not a valid Message Number, interpreted as 0x20. 0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
    #[inline(always)]
    pub fn message_number(&self) -> MESSAGENUMBER_R {
        MESSAGENUMBER_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag 1 = Writing to the IF2 Command Request Register is in progress. This bit can only be read by the software. 0 = Read/write action has finished."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number 0x01-0x20: Valid Message Number, the Message Object in the Message RAM is selected for data transfer. 0x00: Not a valid Message Number, interpreted as 0x20. 0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
    #[inline(always)]
    pub fn message_number(&mut self) -> MESSAGENUMBER_W {
        MESSAGENUMBER_W { w: self }
    }
    #[doc = "Bit 15 - Busy Flag 1 = Writing to the IF2 Command Request Register is in progress. This bit can only be read by the software. 0 = Read/write action has finished."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
