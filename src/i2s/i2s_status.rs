#[doc = "Reader of register I2S_STATUS"]
pub type R = crate::R<u32, super::I2S_STATUS>;
#[doc = "Writer for register I2S_STATUS"]
pub type W = crate::W<u32, super::I2S_STATUS>;
#[doc = "Register I2S_STATUS `reset()`'s with value 0x0014_1000"]
impl crate::ResetValue for super::I2S_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0014_1000
    }
}
#[doc = "Reader of field `I2SINT`"]
pub type I2SINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2SRXINT`"]
pub type I2SRXINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2STXINT`"]
pub type I2STXINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RIGHT`"]
pub type RIGHT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUDF`"]
pub type RXUDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDF`"]
pub struct RXUDF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDF_W<'a> {
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
#[doc = "Reader of field `RXOVF`"]
pub type RXOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVF`"]
pub struct RXOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVF_W<'a> {
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
#[doc = "Reader of field `RXTHF`"]
pub type RXTHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXEMPTY`"]
pub type RXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUDF`"]
pub type TXUDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUDF`"]
pub struct TXUDF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXOVF`"]
pub type TXOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOVF`"]
pub struct TXOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TXTHF`"]
pub type TXTHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFULL`"]
pub type TXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUSY`"]
pub type TXBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RZCF`"]
pub type RZCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RZCF`"]
pub struct RZCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RZCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LZCF`"]
pub type LZCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LZCF`"]
pub struct LZCF_W<'a> {
    w: &'a mut W,
}
impl<'a> LZCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RX_LEVEL`"]
pub type RX_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_LEVEL`"]
pub type TX_LEVEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - I2S Interrupt flag 1 = I2S interrupt 0 = No I2S interrup It is wire-OR of I2STXINT and I2SRXINT bits. This bit is read only."]
    #[inline(always)]
    pub fn i2sint(&self) -> I2SINT_R {
        I2SINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2S receive interrupt 1 = Receive interrupt 0 = No receive interrupt This bit is read only"]
    #[inline(always)]
    pub fn i2srxint(&self) -> I2SRXINT_R {
        I2SRXINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2S transmit interrupt 1 = Transmit interrupt 0 = No transmit interrupt This bit is read only"]
    #[inline(always)]
    pub fn i2stxint(&self) -> I2STXINT_R {
        I2STXINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Right channel This bit indicate current transmit data is belong to right channel. 1 = Right channel 0 = Left channel This bit is read only."]
    #[inline(always)]
    pub fn right(&self) -> RIGHT_R {
        RIGHT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO underflow flag Read receive FIFO when it is empty, this bit set to 1 indicate underflow occur. 1 = Underflow occur 0 = No underflow occur Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxudf(&self) -> RXUDF_R {
        RXUDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO overflow flag When receive FIFO is full and receive hardware attempt write to data into receive FIFO then this bit is set to 1, data in 1st buffer is overwrote. 1 = Overflow occur 0 = No overflow occur Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxovf(&self) -> RXOVF_R {
        RXOVF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO threshold flag When data word(s) in receive FIFO is equal or higher than threshold value set in RXTH\\[2:0\\]
the RXTHF bit becomes to 1. It keeps at 1 till RXFIFO_LEVEL\\[3:0\\]
less than RXTH\\[1:0\\]
after software read RXFIFO register. 1 = Data word(s) in FIFO is equal or higher than threshold level 0 = Data word(s) in FIFO is lower than threshold level This bit is read only."]
    #[inline(always)]
    pub fn rxthf(&self) -> RXTHF_R {
        RXTHF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO full This bit reflect data words number in receive FIFO is 8. 1 = Full 0 = Not full This bit is read only."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive FIFO empty This bit reflects data words number in receive FIFO is zero. 1 = Empty 0 = Not empty This bit is read only."]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO underflow flag When transmit FIFO is empty and shift logic hardware read data from data FIFO causes this set to 1. 1 = Underflow 0 = No underflow Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn txudf(&self) -> TXUDF_R {
        TXUDF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit FIFO overflow flag Write data to transmit FIFO when it is full and this bit set to 1. 1 = Overflow 0 = No overflow Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn txovf(&self) -> TXOVF_R {
        TXOVF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO threshold flag When data word(s) in transmit FIFO is equal or lower than threshold value set in TXTH\\[2:0\\]
the TXTHF bit becomes to 1. It keeps at 1 till TXFIFO_LEVEL\\[3:0\\]
is higher than TXTH\\[1:0\\]
after software write TXFIFO register. 1 = Data word(s) in FIFO is equal or lower than threshold level 0 = Data word(s) in FIFO is higher than threshold level This bit is read only."]
    #[inline(always)]
    pub fn txthf(&self) -> TXTHF_R {
        TXTHF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO full This bit reflect data word number in transmit FIFO is 8. 1 = Full 0 = Not full This bit is read only."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmit FIFO empty This bit reflect data word number in transmit FIFO is zero. 1 = Empty 0 = Not empty This bit is read only."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Busy This bit is clear to 0 when all data in transmit FIFO and shift buffer is shifted out. And set to 1 when 1st data is load to shift buffer. 1 = Transmit shift buffer is busy 0 = Transmit shift buffer is empty This bit is read only."]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Right channel zero cross flag It indicates right channel next sample data sign bit is changed or all data bits are zero. 1 = Right channel zero cross is detected 0 = No zero cross Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rzcf(&self) -> RZCF_R {
        RZCF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Left channel zero cross flag It indicates left channel next sample data sign bit is changed or all data bits are zero. 1 = Left channel zero cross is detected 0 = No zero cross Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn lzcf(&self) -> LZCF_R {
        LZCF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Receive FIFO level These bits indicate word number in receive FIFO. 0000 = No data 0001 = 1 word in receive FIFO 0010 = 2 word in receive FIFO 0011 = 3 word in receive FIFO 0100 = 4 word in receive FIFO 0101 = 5 word in receive FIFO 0110 = 6 word in receive FIFO 0111 = 7 word in receive FIFO 1000 = 8 word in receive FIFO"]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Transmit FIFO level These bits indicate word number in Transmit FIFO. 0000 = No data 0001 = 1 word in Transmit FIFO 0010 = 2 word in Transmit FIFO 0011 = 3 word in Transmit FIFO 0100 = 4 word in Transmit FIFO 0101 = 5 word in Transmit FIFO 0110 = 6 word in Transmit FIFO 0111 = 7 word in Transmit FIFO 1000 = 8 word in Transmit FIFO"]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Receive FIFO underflow flag Read receive FIFO when it is empty, this bit set to 1 indicate underflow occur. 1 = Underflow occur 0 = No underflow occur Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxudf(&mut self) -> RXUDF_W {
        RXUDF_W { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO overflow flag When receive FIFO is full and receive hardware attempt write to data into receive FIFO then this bit is set to 1, data in 1st buffer is overwrote. 1 = Overflow occur 0 = No overflow occur Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxovf(&mut self) -> RXOVF_W {
        RXOVF_W { w: self }
    }
    #[doc = "Bit 16 - Transmit FIFO underflow flag When transmit FIFO is empty and shift logic hardware read data from data FIFO causes this set to 1. 1 = Underflow 0 = No underflow Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn txudf(&mut self) -> TXUDF_W {
        TXUDF_W { w: self }
    }
    #[doc = "Bit 17 - Transmit FIFO overflow flag Write data to transmit FIFO when it is full and this bit set to 1. 1 = Overflow 0 = No overflow Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn txovf(&mut self) -> TXOVF_W {
        TXOVF_W { w: self }
    }
    #[doc = "Bit 22 - Right channel zero cross flag It indicates right channel next sample data sign bit is changed or all data bits are zero. 1 = Right channel zero cross is detected 0 = No zero cross Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rzcf(&mut self) -> RZCF_W {
        RZCF_W { w: self }
    }
    #[doc = "Bit 23 - Left channel zero cross flag It indicates left channel next sample data sign bit is changed or all data bits are zero. 1 = Left channel zero cross is detected 0 = No zero cross Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn lzcf(&mut self) -> LZCF_W {
        LZCF_W { w: self }
    }
}
