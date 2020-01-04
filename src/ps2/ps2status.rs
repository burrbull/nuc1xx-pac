#[doc = "Reader of register PS2STATUS"]
pub type R = crate::R<u32, super::PS2STATUS>;
#[doc = "Writer for register PS2STATUS"]
pub type W = crate::W<u32, super::PS2STATUS>;
#[doc = "Register PS2STATUS `reset()`'s with value 0x83"]
impl crate::ResetValue for super::PS2STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x83
    }
}
#[doc = "Reader of field `PS2CLK`"]
pub type PS2CLK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PS2DATA`"]
pub type PS2DATA_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAMERR`"]
pub type FRAMERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMERR`"]
pub struct FRAMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMERR_W<'a> {
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
#[doc = "Reader of field `RXPARITY`"]
pub type RXPARITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUSY`"]
pub type RXBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUSY`"]
pub type TXBUSY_R = crate::R<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BYTEIDX`"]
pub type BYTEIDX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - CLK Pin State This bit reflects the status of the PS2CLK line after synchronizing."]
    #[inline(always)]
    pub fn ps2clk(&self) -> PS2CLK_R {
        PS2CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DATA Pin State This bit reflects the status of the PS2DATA line after synchronizing and sampling."]
    #[inline(always)]
    pub fn ps2data(&self) -> PS2DATA_R {
        PS2DATA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Frame Error For host to device communication, if STOP bit (logic 1) is not received it is a frame error. If frame error occurs, DATA line may keep at low state after 12th clock. At this moment, S/w overrides PS2CLK to send clock till PS2DATA release to high state. After that, device sends a \"Resend\" command to host. 1 = Frame error occur 0 = No frame error Write 1 to clear this bit."]
    #[inline(always)]
    pub fn framerr(&self) -> FRAMERR_R {
        FRAMERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Received Parity This bit reflects the parity bit for the last received data byte (odd parity). Read only bit."]
    #[inline(always)]
    pub fn rxparity(&self) -> RXPARITY_R {
        RXPARITY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Busy This bit indicates that the PS2 device is currently receiving data. 0 = Idle. 1 = Currently receiving data. Read only bit."]
    #[inline(always)]
    pub fn rxbusy(&self) -> RXBUSY_R {
        RXBUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Busy This bit indicates that the PS2 device is currently sending data. 0 = Idle. 1 = Currently sending data. Read only bit."]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Buffer Overwrite 1 = Data in PS2RXDATA register is overwritten by new coming data. 0 = No overwrite Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxovf(&self) -> RXOVF_R {
        RXOVF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Empty When S/W writes any data to PS2TXDATA0-3 the TXEMPTY bit is cleared to 0 immediately if PS2EN is enabled. When transmitted data byte number is equal to FIFODEPTH then TXEMPTY bit is clear to 1. 1 = FIFO is empty 0 = There is data to be transmitted Read only bit."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Byte Index It indicates which data byte in transmit data shift register. When all data in FIFO is transmitted and it will be cleared to 0. It is a read only bit. BYTEIDX DATA Transmit BYTEIDX DATA Transmit 0000 TXDATA0\\[7:0\\]
1000 TXDATA2\\[7:0\\]
0001 TXDATA0\\[15:8\\]
1001 TXDATA2\\[15:8\\]
0010 TXDATA0\\[23:16\\]
1010 TXDATA2\\[23:16\\]
0011 TXDATA0\\[31:24\\]
1011 TXDATA2\\[31:24\\]
0100 TXDATA1\\[7:0\\]
1100 TXDATA3\\[7:0\\]
0101 TXDATA1\\[15:8\\]
1101 TXDATA3\\[15:8\\]
0110 TXDATA1\\[23:16\\]
1110 TXDATA3\\[23:16\\]
0111 TXDATA1\\[31:24\\]
1111 TXDATA3\\[31:24\\]"]
    #[inline(always)]
    pub fn byteidx(&self) -> BYTEIDX_R {
        BYTEIDX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Frame Error For host to device communication, if STOP bit (logic 1) is not received it is a frame error. If frame error occurs, DATA line may keep at low state after 12th clock. At this moment, S/w overrides PS2CLK to send clock till PS2DATA release to high state. After that, device sends a \"Resend\" command to host. 1 = Frame error occur 0 = No frame error Write 1 to clear this bit."]
    #[inline(always)]
    pub fn framerr(&mut self) -> FRAMERR_W {
        FRAMERR_W { w: self }
    }
    #[doc = "Bit 6 - RX Buffer Overwrite 1 = Data in PS2RXDATA register is overwritten by new coming data. 0 = No overwrite Write 1 to clear this bit."]
    #[inline(always)]
    pub fn rxovf(&mut self) -> RXOVF_W {
        RXOVF_W { w: self }
    }
}
