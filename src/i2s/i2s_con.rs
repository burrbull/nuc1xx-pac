#[doc = "Reader of register I2S_CON"]
pub type R = crate::R<u32, super::I2S_CON>;
#[doc = "Writer for register I2S_CON"]
pub type W = crate::W<u32, super::I2S_CON>;
#[doc = "Register I2S_CON `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2SEN`"]
pub type I2SEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2SEN`"]
pub struct I2SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SEN_W<'a> {
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
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEN`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
#[doc = "Reader of field `MUTE`"]
pub type MUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUTE`"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
#[doc = "Reader of field `WORDWIDTH`"]
pub type WORDWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WORDWIDTH`"]
pub struct WORDWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
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
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE`"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
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
#[doc = "Reader of field `TXTH`"]
pub type TXTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTH`"]
pub struct TXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `RXTH`"]
pub type RXTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTH`"]
pub struct RXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCLKEN`"]
pub type MCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLKEN`"]
pub struct MCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKEN_W<'a> {
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
#[doc = "Reader of field `RCHZCEN`"]
pub type RCHZCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCHZCEN`"]
pub struct RCHZCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCHZCEN_W<'a> {
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
#[doc = "Reader of field `LCHZCEN`"]
pub type LCHZCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCHZCEN`"]
pub struct LCHZCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCHZCEN_W<'a> {
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
#[doc = "Reader of field `CLR_TXFIFO`"]
pub type CLR_TXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_TXFIFO`"]
pub struct CLR_TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_TXFIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLR_RXFIFO`"]
pub type CLR_RXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_RXFIFO`"]
pub struct CLR_RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXFIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TXDMA`"]
pub type TXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMA`"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RXDMA`"]
pub type RXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMA`"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable I2S Controller 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable 1 = Enable data transmit 0 = Disable data transmit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Enable 1 = Enable data receive 0 = Disable data receive"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Mute Enable 1 = Transmit channel zero 0 = Transmit data is shifted from buffer"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Word Width 00 = data is 8 bit 01 = data is 16 bit 10 = data is 24 bit 11 = data is 32 bit"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WORDWIDTH_R {
        WORDWIDTH_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Monaural Data 1 = Data is monaural format 0 = Data is stereo format"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Format 1 = MSB justified data format 0 = I2S data format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Mode I2S can operate as master or slave. For master mode, I2S_BCLK and I2S_LRCLK pins are output mode and send bit clock from NuMicro(TM) NUC100 series to Audio CODEC chip. In slave mode, I2S_BCLK and I2S_LRCLK pins are input mode and I2S_BCLK and I2S_LRCLK signals are received from outer Audio CODEC chip. 1 = Slave mode 0 = Master mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Transmit FIFO Threshold Level If remain data word (32 bits) in transmit FIFO is the same or less than threshold level then TXTHI flag is set. 000 = 0 word data in transmit FIFO 001 = 1 word data in transmit FIFO 010 = 2 words data in transmit FIFO 011 = 3 words data in transmit FIFO 100 = 4 word data in transmit FIFO 101 = 5 word data in transmit FIFO 110 = 6 word data in transmit FIFO 111 = 7 word data in transmit FIFO"]
    #[inline(always)]
    pub fn txth(&self) -> TXTH_R {
        TXTH_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Receive FIFO Threshold Level When received data word(s) in buffer is equal or higher than threshold level then RXTHI flag is set. 000 = 1 word data in receive FIFO 001 = 2 word data in receive FIFO 010 = 3 word data in receive FIFO 011 = 4 word data in receive FIFO 100 = 5 word data in receive FIFO 101 = 6 word data in receive FIFO 110 = 7 word data in receive FIFO 111 = 8 word data in receive FIFO"]
    #[inline(always)]
    pub fn rxth(&self) -> RXTH_R {
        RXTH_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Master Clock Enable If NuMicro(TM) NUC100 series external crystal clock is frequency 2*N*256fs then software can program MCLK_DIV\\[2:0\\]
in I2S_CLKDIV register to get 256fs clock to audio codec chip. 1 = Enable master clock 0 = Disable master clock"]
    #[inline(always)]
    pub fn mclken(&self) -> MCLKEN_R {
        MCLKEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Right channel zero cross detect enable If this bit is set to 1, when left channel data sign bit change or next shift data bits are all zero then RZCF flag in I2S_STATUS register is set to 1. 1 = Enable right channel zero cross detect 0 = Disable right channel zero cross detect"]
    #[inline(always)]
    pub fn rchzcen(&self) -> RCHZCEN_R {
        RCHZCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Left channel zero cross detect enable If this bit is set to 1, when left channel data sign bit change or next shift data bits are all zero then LZCF flag in I2S_STATUS register is set to 1. 1 = Enable left channel zero cross detect 0 = Disable left channel zero cross detect"]
    #[inline(always)]
    pub fn lchzcen(&self) -> LCHZCEN_R {
        LCHZCEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clear Transmit FIFO Write 1 to clear transmit FIFO, internal pointer is reset to FIFO start point, and TXFIFO_LEVEL\\[3:0\\]
returns to zero and transmit FIFO becomes empty but data in transmit FIFO is not changed. This bit is clear by hardware automatically, read it return zero."]
    #[inline(always)]
    pub fn clr_txfifo(&self) -> CLR_TXFIFO_R {
        CLR_TXFIFO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear Receive FIFO Write 1 to clear receive FIFO, internal pointer is reset to FIFO start point, and RXFIFO_LEVEL\\[3:0\\]
returns to zero and receive FIFO becomes empty. This bit is clear by hardware automatically, read it return zero."]
    #[inline(always)]
    pub fn clr_rxfifo(&self) -> CLR_RXFIFO_R {
        CLR_RXFIFO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable Transmit DMA When TX DMA is enabled, I2S requests DMA to transfer data from SRAM to transmit FIFO if FIFO is not full. 1 = Enable TX DMA 0 = Disable TX DMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable Receive DMA When RX DMA is enabled, I2S requests DMA to transfer data from receive FIFO to SRAM if FIFO is not empty. 1 = Enable RX DMA 0 = Disable RX DMA"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S Controller 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Enable 1 = Enable data transmit 0 = Disable data transmit"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 2 - Receive Enable 1 = Enable data receive 0 = Disable data receive"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Mute Enable 1 = Transmit channel zero 0 = Transmit data is shifted from buffer"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bits 4:5 - Word Width 00 = data is 8 bit 01 = data is 16 bit 10 = data is 24 bit 11 = data is 32 bit"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> WORDWIDTH_W {
        WORDWIDTH_W { w: self }
    }
    #[doc = "Bit 6 - Monaural Data 1 = Data is monaural format 0 = Data is stereo format"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 7 - Data Format 1 = MSB justified data format 0 = I2S data format"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bit 8 - Slave Mode I2S can operate as master or slave. For master mode, I2S_BCLK and I2S_LRCLK pins are output mode and send bit clock from NuMicro(TM) NUC100 series to Audio CODEC chip. In slave mode, I2S_BCLK and I2S_LRCLK pins are input mode and I2S_BCLK and I2S_LRCLK signals are received from outer Audio CODEC chip. 1 = Slave mode 0 = Master mode"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bits 9:11 - Transmit FIFO Threshold Level If remain data word (32 bits) in transmit FIFO is the same or less than threshold level then TXTHI flag is set. 000 = 0 word data in transmit FIFO 001 = 1 word data in transmit FIFO 010 = 2 words data in transmit FIFO 011 = 3 words data in transmit FIFO 100 = 4 word data in transmit FIFO 101 = 5 word data in transmit FIFO 110 = 6 word data in transmit FIFO 111 = 7 word data in transmit FIFO"]
    #[inline(always)]
    pub fn txth(&mut self) -> TXTH_W {
        TXTH_W { w: self }
    }
    #[doc = "Bits 12:14 - Receive FIFO Threshold Level When received data word(s) in buffer is equal or higher than threshold level then RXTHI flag is set. 000 = 1 word data in receive FIFO 001 = 2 word data in receive FIFO 010 = 3 word data in receive FIFO 011 = 4 word data in receive FIFO 100 = 5 word data in receive FIFO 101 = 6 word data in receive FIFO 110 = 7 word data in receive FIFO 111 = 8 word data in receive FIFO"]
    #[inline(always)]
    pub fn rxth(&mut self) -> RXTH_W {
        RXTH_W { w: self }
    }
    #[doc = "Bit 15 - Master Clock Enable If NuMicro(TM) NUC100 series external crystal clock is frequency 2*N*256fs then software can program MCLK_DIV\\[2:0\\]
in I2S_CLKDIV register to get 256fs clock to audio codec chip. 1 = Enable master clock 0 = Disable master clock"]
    #[inline(always)]
    pub fn mclken(&mut self) -> MCLKEN_W {
        MCLKEN_W { w: self }
    }
    #[doc = "Bit 16 - Right channel zero cross detect enable If this bit is set to 1, when left channel data sign bit change or next shift data bits are all zero then RZCF flag in I2S_STATUS register is set to 1. 1 = Enable right channel zero cross detect 0 = Disable right channel zero cross detect"]
    #[inline(always)]
    pub fn rchzcen(&mut self) -> RCHZCEN_W {
        RCHZCEN_W { w: self }
    }
    #[doc = "Bit 17 - Left channel zero cross detect enable If this bit is set to 1, when left channel data sign bit change or next shift data bits are all zero then LZCF flag in I2S_STATUS register is set to 1. 1 = Enable left channel zero cross detect 0 = Disable left channel zero cross detect"]
    #[inline(always)]
    pub fn lchzcen(&mut self) -> LCHZCEN_W {
        LCHZCEN_W { w: self }
    }
    #[doc = "Bit 18 - Clear Transmit FIFO Write 1 to clear transmit FIFO, internal pointer is reset to FIFO start point, and TXFIFO_LEVEL\\[3:0\\]
returns to zero and transmit FIFO becomes empty but data in transmit FIFO is not changed. This bit is clear by hardware automatically, read it return zero."]
    #[inline(always)]
    pub fn clr_txfifo(&mut self) -> CLR_TXFIFO_W {
        CLR_TXFIFO_W { w: self }
    }
    #[doc = "Bit 19 - Clear Receive FIFO Write 1 to clear receive FIFO, internal pointer is reset to FIFO start point, and RXFIFO_LEVEL\\[3:0\\]
returns to zero and receive FIFO becomes empty. This bit is clear by hardware automatically, read it return zero."]
    #[inline(always)]
    pub fn clr_rxfifo(&mut self) -> CLR_RXFIFO_W {
        CLR_RXFIFO_W { w: self }
    }
    #[doc = "Bit 20 - Enable Transmit DMA When TX DMA is enabled, I2S requests DMA to transfer data from SRAM to transmit FIFO if FIFO is not full. 1 = Enable TX DMA 0 = Disable TX DMA"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 21 - Enable Receive DMA When RX DMA is enabled, I2S requests DMA to transfer data from receive FIFO to SRAM if FIFO is not empty. 1 = Enable RX DMA 0 = Disable RX DMA"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
}
