#[doc = "Reader of register SPI_CNTRL"]
pub type R = crate::R<u32, super::SPI_CNTRL>;
#[doc = "Writer for register SPI_CNTRL"]
pub type W = crate::W<u32, super::SPI_CNTRL>;
#[doc = "Register SPI_CNTRL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::SPI_CNTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `GO_BUSY`"]
pub type GO_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GO_BUSY`"]
pub struct GO_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_BUSY_W<'a> {
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
#[doc = "Reader of field `RX_NEG`"]
pub type RX_NEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_NEG`"]
pub struct RX_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NEG_W<'a> {
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
#[doc = "Reader of field `TX_NEG`"]
pub type TX_NEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_NEG`"]
pub struct TX_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NEG_W<'a> {
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
#[doc = "Reader of field `TX_BIT_LEN`"]
pub type TX_BIT_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BIT_LEN`"]
pub struct TX_BIT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `TX_NUM`"]
pub type TX_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_NUM`"]
pub struct TX_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LSB`"]
pub type LSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSB`"]
pub struct LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> LSB_W<'a> {
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
#[doc = "Reader of field `CLKP`"]
pub type CLKP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKP`"]
pub struct CLKP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKP_W<'a> {
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
#[doc = "Reader of field `SP_CYCLE`"]
pub type SP_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SP_CYCLE`"]
pub struct SP_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IF`"]
pub type IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF`"]
pub struct IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_W<'a> {
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
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `REORDER`"]
pub type REORDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REORDER`"]
pub struct REORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REORDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `TWOB`"]
pub type TWOB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWOB`"]
pub struct TWOB_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOB_W<'a> {
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
#[doc = "Reader of field `VARCLK_EN`"]
pub type VARCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VARCLK_EN`"]
pub struct VARCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VARCLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Go and Busy Status 1 = In master mode, writing 1 to this bit to start the SPI data transfer; in slave mode, writing 1 to this bit indicates that the slave is ready to communicate with a master. 0 = Writing 0 to this bit to stop data transfer if SPI is transferring. During the data transfer, this bit keeps the value of 1. As the transfer is finished, this bit will be cleared automatically. NOTE: All registers should be set before writing 1 to this GO_BUSY bit. The transfer result will be unpredictable if software changes related settings when GO_BUSY bit is 1."]
    #[inline(always)]
    pub fn go_busy(&self) -> GO_BUSY_R {
        GO_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive At Negative Edge 1 = The received data input signal is latched at the falling edge of SPICLK. 0 = The received data input signal is latched at the rising edge of SPICLK."]
    #[inline(always)]
    pub fn rx_neg(&self) -> RX_NEG_R {
        RX_NEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit At Negative Edge 1 = The transmitted data output signal is changed at the falling edge of SPICLK. 0 = The transmitted data output signal is changed at the rising edge of SPICLK."]
    #[inline(always)]
    pub fn tx_neg(&self) -> TX_NEG_R {
        TX_NEG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Transmit Bit Length This field specifies how many bits are transmitted in one transaction. Up to 32 bits can be transmitted. TX_BIT_LEN = 0x01 ... 1 bit TX_BIT_LEN = 0x02 ... 2 bits ...... TX_BIT_LEN = 0x1f ... 31 bits TX_BIT_LEN = 0x00 .. 32 bits"]
    #[inline(always)]
    pub fn tx_bit_len(&self) -> TX_BIT_LEN_R {
        TX_BIT_LEN_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Numbers of Transmit/Receive Word This field specifies how many transmit/receive word numbers should be executed in one transfer. 00 = Only one transmit/receive word will be executed in one transfer. 01 = Two successive transmit/receive words will be executed in one transfer. (burst mode) 10 = Reserved. 11 = Reserved."]
    #[inline(always)]
    pub fn tx_num(&self) -> TX_NUM_R {
        TX_NUM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - LSB First 1 = The LSB is sent first on the line (bit 0 of SPI_TX0/1), and the first bit received from the line will be put in the LSB position of the RX register (bit 0 of SPI_RX0/1). 0 = The MSB is transmitted/received first (which bit in SPI_TX0/1 and SPI_RX0/1 register that is depends on the TX_BIT_LEN field)."]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clock Polarity 1 = SPICLK idle high. 0 = SPICLK idle low."]
    #[inline(always)]
    pub fn clkp(&self) -> CLKP_R {
        CLKP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Suspend Interval (master only) These four bits provide configurable suspend interval between two successive transmit/receive transactions in a transfer. The suspend interval is from the last falling clock edge of the current transaction to the first rising clock edge of the successive transaction if CLKP = 0. If CLKP = 1, the interval is from the rising clock edge to the falling clock edge. The default value is 0x0. When TX_NUM = 00b, setting this field has no effect on transfer. The desired suspend interval is obtained according to the following equation: (SP_CYCLE\\[3:0\\]
+ 2)*period of SPI clock SP_CYCLE = 0x0 ... 2 SPICLK clock cycle SP_CYCLE = 0x1 ... 3 SPICLK clock cycle ...... SP_CYCLE = 0xe ... 16 SPICLK clock cycle SP_CYCLE = 0xf ... 17 SPICLK clock cycle"]
    #[inline(always)]
    pub fn sp_cycle(&self) -> SP_CYCLE_R {
        SP_CYCLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Interrupt Flag 1 = It indicates that the transfer is done. The interrupt flag is set if it was enable. 0 = It indicates that the transfer does not finish yet. NOTE: This bit can be cleared by writing 1 to itself."]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt Enable 1 = Enable MICROWIRE/SPI Interrupt. 0 = Disable MICROWIRE/SPI Interrupt."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SLAVE Mode Indication 1 = Slave mode. 0 = Master mode."]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Reorder Mode Select 00 = Disable both byte reorder and byte suspend functions. 01 = Enable byte reorder function and insert a byte suspend interval (2~17 SPICLK cycles) among each byte. The setting of TX_BIT_LEN must be configured as 0x00. (32 bits/word). 10 = Enable byte reorder function, but disable byte suspend function. 11 = Disable byte reorder function, but insert a suspend interval (2~17 SPICLK cycles) among each byte. The setting of TX_BIT_LEN must be configured as 0x00. (32 bits/word). Byte reorder function is only available if TX_BIT_LEN is defined as 16, 24 and 32."]
    #[inline(always)]
    pub fn reorder(&self) -> REORDER_R {
        REORDER_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Two Bits Transfer Mode Active 1 = Enable two-bit transfer mode. 0 = disable two-bit transfer mode. Note that when enable TWOB, the serial transmitted 2-bit data output are from SPI_TX1/0, and the received 2-bit data input are put in SPI_RX1/0. Note that when enable TWOB, the setting of TX_NUM must be programmed as 0x00."]
    #[inline(always)]
    pub fn twob(&self) -> TWOB_R {
        TWOB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Variable Clock Enable (master only) 1 = The serial clock output frequency is variable. The output frequency is decided by the value of VARCLK, DIVIDER, and DIVIDER2. 0 = The serial clock output frequency is fixed and decided only by the value of DIVIDER. Note that when enable this VARCLK_EN bit, the setting of TX_BIT_LEN must be programmed as 0x10 (16 bits mode)"]
    #[inline(always)]
    pub fn varclk_en(&self) -> VARCLK_EN_R {
        VARCLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Go and Busy Status 1 = In master mode, writing 1 to this bit to start the SPI data transfer; in slave mode, writing 1 to this bit indicates that the slave is ready to communicate with a master. 0 = Writing 0 to this bit to stop data transfer if SPI is transferring. During the data transfer, this bit keeps the value of 1. As the transfer is finished, this bit will be cleared automatically. NOTE: All registers should be set before writing 1 to this GO_BUSY bit. The transfer result will be unpredictable if software changes related settings when GO_BUSY bit is 1."]
    #[inline(always)]
    pub fn go_busy(&mut self) -> GO_BUSY_W {
        GO_BUSY_W { w: self }
    }
    #[doc = "Bit 1 - Receive At Negative Edge 1 = The received data input signal is latched at the falling edge of SPICLK. 0 = The received data input signal is latched at the rising edge of SPICLK."]
    #[inline(always)]
    pub fn rx_neg(&mut self) -> RX_NEG_W {
        RX_NEG_W { w: self }
    }
    #[doc = "Bit 2 - Transmit At Negative Edge 1 = The transmitted data output signal is changed at the falling edge of SPICLK. 0 = The transmitted data output signal is changed at the rising edge of SPICLK."]
    #[inline(always)]
    pub fn tx_neg(&mut self) -> TX_NEG_W {
        TX_NEG_W { w: self }
    }
    #[doc = "Bits 3:7 - Transmit Bit Length This field specifies how many bits are transmitted in one transaction. Up to 32 bits can be transmitted. TX_BIT_LEN = 0x01 ... 1 bit TX_BIT_LEN = 0x02 ... 2 bits ...... TX_BIT_LEN = 0x1f ... 31 bits TX_BIT_LEN = 0x00 .. 32 bits"]
    #[inline(always)]
    pub fn tx_bit_len(&mut self) -> TX_BIT_LEN_W {
        TX_BIT_LEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Numbers of Transmit/Receive Word This field specifies how many transmit/receive word numbers should be executed in one transfer. 00 = Only one transmit/receive word will be executed in one transfer. 01 = Two successive transmit/receive words will be executed in one transfer. (burst mode) 10 = Reserved. 11 = Reserved."]
    #[inline(always)]
    pub fn tx_num(&mut self) -> TX_NUM_W {
        TX_NUM_W { w: self }
    }
    #[doc = "Bit 10 - LSB First 1 = The LSB is sent first on the line (bit 0 of SPI_TX0/1), and the first bit received from the line will be put in the LSB position of the RX register (bit 0 of SPI_RX0/1). 0 = The MSB is transmitted/received first (which bit in SPI_TX0/1 and SPI_RX0/1 register that is depends on the TX_BIT_LEN field)."]
    #[inline(always)]
    pub fn lsb(&mut self) -> LSB_W {
        LSB_W { w: self }
    }
    #[doc = "Bit 11 - Clock Polarity 1 = SPICLK idle high. 0 = SPICLK idle low."]
    #[inline(always)]
    pub fn clkp(&mut self) -> CLKP_W {
        CLKP_W { w: self }
    }
    #[doc = "Bits 12:15 - Suspend Interval (master only) These four bits provide configurable suspend interval between two successive transmit/receive transactions in a transfer. The suspend interval is from the last falling clock edge of the current transaction to the first rising clock edge of the successive transaction if CLKP = 0. If CLKP = 1, the interval is from the rising clock edge to the falling clock edge. The default value is 0x0. When TX_NUM = 00b, setting this field has no effect on transfer. The desired suspend interval is obtained according to the following equation: (SP_CYCLE\\[3:0\\]
+ 2)*period of SPI clock SP_CYCLE = 0x0 ... 2 SPICLK clock cycle SP_CYCLE = 0x1 ... 3 SPICLK clock cycle ...... SP_CYCLE = 0xe ... 16 SPICLK clock cycle SP_CYCLE = 0xf ... 17 SPICLK clock cycle"]
    #[inline(always)]
    pub fn sp_cycle(&mut self) -> SP_CYCLE_W {
        SP_CYCLE_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Flag 1 = It indicates that the transfer is done. The interrupt flag is set if it was enable. 0 = It indicates that the transfer does not finish yet. NOTE: This bit can be cleared by writing 1 to itself."]
    #[inline(always)]
    pub fn if_(&mut self) -> IF_W {
        IF_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Enable 1 = Enable MICROWIRE/SPI Interrupt. 0 = Disable MICROWIRE/SPI Interrupt."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 18 - SLAVE Mode Indication 1 = Slave mode. 0 = Master mode."]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bits 19:20 - Reorder Mode Select 00 = Disable both byte reorder and byte suspend functions. 01 = Enable byte reorder function and insert a byte suspend interval (2~17 SPICLK cycles) among each byte. The setting of TX_BIT_LEN must be configured as 0x00. (32 bits/word). 10 = Enable byte reorder function, but disable byte suspend function. 11 = Disable byte reorder function, but insert a suspend interval (2~17 SPICLK cycles) among each byte. The setting of TX_BIT_LEN must be configured as 0x00. (32 bits/word). Byte reorder function is only available if TX_BIT_LEN is defined as 16, 24 and 32."]
    #[inline(always)]
    pub fn reorder(&mut self) -> REORDER_W {
        REORDER_W { w: self }
    }
    #[doc = "Bit 22 - Two Bits Transfer Mode Active 1 = Enable two-bit transfer mode. 0 = disable two-bit transfer mode. Note that when enable TWOB, the serial transmitted 2-bit data output are from SPI_TX1/0, and the received 2-bit data input are put in SPI_RX1/0. Note that when enable TWOB, the setting of TX_NUM must be programmed as 0x00."]
    #[inline(always)]
    pub fn twob(&mut self) -> TWOB_W {
        TWOB_W { w: self }
    }
    #[doc = "Bit 23 - Variable Clock Enable (master only) 1 = The serial clock output frequency is variable. The output frequency is decided by the value of VARCLK, DIVIDER, and DIVIDER2. 0 = The serial clock output frequency is fixed and decided only by the value of DIVIDER. Note that when enable this VARCLK_EN bit, the setting of TX_BIT_LEN must be programmed as 0x10 (16 bits mode)"]
    #[inline(always)]
    pub fn varclk_en(&mut self) -> VARCLK_EN_W {
        VARCLK_EN_W { w: self }
    }
}
