#[doc = "Reader of register PS2INTID"]
pub type R = crate::R<u32, super::PS2INTID>;
#[doc = "Writer for register PS2INTID"]
pub type W = crate::W<u32, super::PS2INTID>;
#[doc = "Register PS2INTID `reset()`'s with value 0"]
impl crate::ResetValue for super::PS2INTID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXINT`"]
pub type RXINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINT`"]
pub struct RXINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINT_W<'a> {
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
#[doc = "Reader of field `TXINT`"]
pub type TXINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINT`"]
pub struct TXINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Receive Interrupt This bit is set to 1 when acknowledge bit is sent for Host to device communication. Interrupt occurs if RXINTEN bit is set to 1. 1 = Receive interrupt occurs 0 = No interrupt Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn rxint(&self) -> RXINT_R {
        RXINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt This bit is set to 1 after STOP bit is transmitted. Interrupt occur if TXINTEN bit is set to 1. 1 = Transmit interrupt occurs 0 = No interrupt Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn txint(&self) -> TXINT_R {
        TXINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Interrupt This bit is set to 1 when acknowledge bit is sent for Host to device communication. Interrupt occurs if RXINTEN bit is set to 1. 1 = Receive interrupt occurs 0 = No interrupt Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RXINT_W {
        RXINT_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Interrupt This bit is set to 1 after STOP bit is transmitted. Interrupt occur if TXINTEN bit is set to 1. 1 = Transmit interrupt occurs 0 = No interrupt Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn txint(&mut self) -> TXINT_W {
        TXINT_W { w: self }
    }
}
