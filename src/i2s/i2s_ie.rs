#[doc = "Reader of register I2S_IE"]
pub type R = crate::R<u32, super::I2S_IE>;
#[doc = "Writer for register I2S_IE"]
pub type W = crate::W<u32, super::I2S_IE>;
#[doc = "Register I2S_IE `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXUDFIE`"]
pub type RXUDFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDFIE`"]
pub struct RXUDFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDFIE_W<'a> {
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
#[doc = "Reader of field `RXOVFIE`"]
pub type RXOVFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVFIE`"]
pub struct RXOVFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVFIE_W<'a> {
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
#[doc = "Reader of field `RXTHIE`"]
pub type RXTHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTHIE`"]
pub struct RXTHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHIE_W<'a> {
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
#[doc = "Reader of field `TXUDFIE`"]
pub type TXUDFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUDFIE`"]
pub struct TXUDFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDFIE_W<'a> {
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
#[doc = "Reader of field `TXOVFIE`"]
pub type TXOVFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOVFIE`"]
pub struct TXOVFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVFIE_W<'a> {
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
#[doc = "Reader of field `TXTHIE`"]
pub type TXTHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTHIE`"]
pub struct TXTHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHIE_W<'a> {
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
#[doc = "Reader of field `RZCIE`"]
pub type RZCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RZCIE`"]
pub struct RZCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RZCIE_W<'a> {
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
#[doc = "Reader of field `LZCIE`"]
pub type LZCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LZCIE`"]
pub struct LZCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LZCIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Receive FIFO underflow interrupt enable If software read receive FIFO when it is empty then RXUDF flag in I2SSTATUS register is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxudfie(&self) -> RXUDFIE_R {
        RXUDFIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO overflow interrupt enable 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxovfie(&self) -> RXOVFIE_R {
        RXOVFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO threshold level interrupt When data word in receive FIFO is equal or higher then RXTH\\[2:0\\]
and the RXTHI bit is set to 1. If RXTHIE bit is enabled, interrupt occur. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxthie(&self) -> RXTHIE_R {
        RXTHIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO underflow interrupt enable Interrupt occurs if this bit is set to 1 and transmit FIFO underflow flag is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txudfie(&self) -> TXUDFIE_R {
        TXUDFIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO overflow interrupt enable Interrupt occurs if this bit is set to 1 and transmit FIFO overflow flag is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txovfie(&self) -> TXOVFIE_R {
        TXOVFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO threshold level interrupt enable Interrupt occurs if this bit is set to 1 and data words in transmit FIFO is less than TXTH\\[2:0\\]. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txthie(&self) -> TXTHIE_R {
        TXTHIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Right channel zero cross interrupt enable Interrupt occurs if this bit is set to 1 and right channel zero cross. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rzcie(&self) -> RZCIE_R {
        RZCIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Left channel zero cross interrupt enable Interrupt occurs if this bit is set to 1 and left channel zero cross. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn lzcie(&self) -> LZCIE_R {
        LZCIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO underflow interrupt enable If software read receive FIFO when it is empty then RXUDF flag in I2SSTATUS register is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxudfie(&mut self) -> RXUDFIE_W {
        RXUDFIE_W { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO overflow interrupt enable 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxovfie(&mut self) -> RXOVFIE_W {
        RXOVFIE_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO threshold level interrupt When data word in receive FIFO is equal or higher then RXTH\\[2:0\\]
and the RXTHI bit is set to 1. If RXTHIE bit is enabled, interrupt occur. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rxthie(&mut self) -> RXTHIE_W {
        RXTHIE_W { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO underflow interrupt enable Interrupt occurs if this bit is set to 1 and transmit FIFO underflow flag is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txudfie(&mut self) -> TXUDFIE_W {
        TXUDFIE_W { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO overflow interrupt enable Interrupt occurs if this bit is set to 1 and transmit FIFO overflow flag is set to 1. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txovfie(&mut self) -> TXOVFIE_W {
        TXOVFIE_W { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO threshold level interrupt enable Interrupt occurs if this bit is set to 1 and data words in transmit FIFO is less than TXTH\\[2:0\\]. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn txthie(&mut self) -> TXTHIE_W {
        TXTHIE_W { w: self }
    }
    #[doc = "Bit 11 - Right channel zero cross interrupt enable Interrupt occurs if this bit is set to 1 and right channel zero cross. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn rzcie(&mut self) -> RZCIE_W {
        RZCIE_W { w: self }
    }
    #[doc = "Bit 12 - Left channel zero cross interrupt enable Interrupt occurs if this bit is set to 1 and left channel zero cross. 1 = Enable interrupt 0 = Disable interrupt"]
    #[inline(always)]
    pub fn lzcie(&mut self) -> LZCIE_W {
        LZCIE_W { w: self }
    }
}
