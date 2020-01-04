#[doc = "Reader of register UA_ISR"]
pub type R = crate::R<u32, super::UA_ISR>;
#[doc = "Writer for register UA_ISR"]
pub type W = crate::W<u32, super::UA_ISR>;
#[doc = "Register UA_ISR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::UA_ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RDA_IF`"]
pub type RDA_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `THRE_IF`"]
pub type THRE_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLS_IF`"]
pub type RLS_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODEM_IF`"]
pub type MODEM_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUT_IF`"]
pub type TOUT_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUF_ERR_IF`"]
pub type BUF_ERR_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LIN_RX_BREAK_IF`"]
pub type LIN_RX_BREAK_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDA_INT`"]
pub type RDA_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `THRE_INT`"]
pub type THRE_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLS_INT`"]
pub type RLS_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODEM_INT`"]
pub type MODEM_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUT_INT`"]
pub type TOUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUF_ERR_INT`"]
pub type BUF_ERR_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `LIN_RX_BREAK_INT`"]
pub type LIN_RX_BREAK_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIN_RX_BREAK_INT`"]
pub struct LIN_RX_BREAK_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_RX_BREAK_INT_W<'a> {
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
#[doc = "Reader of field `HW_RLS_IF`"]
pub type HW_RLS_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_RLS_IF`"]
pub struct HW_RLS_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_RLS_IF_W<'a> {
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
#[doc = "Reader of field `HW_MODEM_IF`"]
pub type HW_MODEM_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_MODEM_IF`"]
pub struct HW_MODEM_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MODEM_IF_W<'a> {
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
#[doc = "Reader of field `HW_TOUT_IF`"]
pub type HW_TOUT_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_TOUT_IF`"]
pub struct HW_TOUT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_TOUT_IF_W<'a> {
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
#[doc = "Reader of field `HW_BUF_ERR_IF`"]
pub type HW_BUF_ERR_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_BUF_ERR_IF`"]
pub struct HW_BUF_ERR_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_BUF_ERR_IF_W<'a> {
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
#[doc = "Reader of field `HW_LIN_RX_BREAK_IF`"]
pub type HW_LIN_RX_BREAK_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_LIN_RX_BREAK_IF`"]
pub struct HW_LIN_RX_BREAK_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LIN_RX_BREAK_IF_W<'a> {
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
#[doc = "Reader of field `HW_RLS_INT`"]
pub type HW_RLS_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_RLS_INT`"]
pub struct HW_RLS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_RLS_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `HW_MODEM_INT`"]
pub type HW_MODEM_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_MODEM_INT`"]
pub struct HW_MODEM_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MODEM_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `HW_TOUT_INT`"]
pub type HW_TOUT_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_TOUT_INT`"]
pub struct HW_TOUT_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_TOUT_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `HW_BUF_ERR_INT`"]
pub type HW_BUF_ERR_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_BUF_ERR_INT`"]
pub struct HW_BUF_ERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_BUF_ERR_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `HW_LIN_RX_BREAK_INT`"]
pub type HW_LIN_RX_BREAK_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_LIN_RX_BREAK_INT`"]
pub struct HW_LIN_RX_BREAK_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LIN_RX_BREAK_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Available Interrupt Flag (Read Only). When the number of bytes in the Rx FIFO equals the RFITL then the RDA_IF will be set. If IER\\[RDA_IEN\\]
is enabled, the RDA interrupt will be generated. NOTE: This bit is read only and it will be cleared when the number of unread bytes of Rx FIFO drops below the threshold level (RFITL)."]
    #[inline(always)]
    pub fn rda_if(&self) -> RDA_IF_R {
        RDA_IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Flag (Read Only). This bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If UA_IER\\[THRE_IEN\\]
is enabled, the THRE interrupt will be generated. NOTE: This bit is read only and it will be cleared when writing data into THR (TX FIFO not empty)."]
    #[inline(always)]
    pub fn thre_if(&self) -> THRE_IF_R {
        THRE_IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Line Interrupt Flag (Read Only). This bit is set when the Rx receive data have parity error, framing error or break error (at least one of 3 bits, BIF, FEF and PEF, is set). If IER\\[RLS_IEN\\]
is enabled, the RLS interrupt will be generated. NOTE: This bit is read only and reset to 0 when all bits of BIF, FEF and PEF are cleared. NOTE: When in RS-485 function mode, this field include \"receiver detect any address byte received address byte character (bit9 = 1') bit. \""]
    #[inline(always)]
    pub fn rls_if(&self) -> RLS_IF_R {
        RLS_IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODEM Interrupt Flag (Read Only) (not available in UART2 channel) This bit is set when the CTS pin has state change(DCTSF=1). if UA_IER\\[Modem_IEN\\]
is enabled, the Modem interrupt will be generated. NOTE: This bit is read only and reset to 0 when bit DCTSF is cleared by a write 1 on DCTSF."]
    #[inline(always)]
    pub fn modem_if(&self) -> MODEM_IF_R {
        MODEM_IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Out Interrupt Flag (Read Only) This bit is set when the RX FIFO is not empty and no activities occurres in the RX FIFO and the time out counter equal to TOIC. If IER\\[Tout_IEN\\]
is enabled, the Tout interrupt will be generated. NOTE: This bit is read only and user can read UA_RBR (Rx is in active) to clear it."]
    #[inline(always)]
    pub fn tout_if(&self) -> TOUT_IF_R {
        TOUT_IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Flag (Read Only) This bit is set when the TX or RX FIFO overflows (TX_Over_IF or RX_Over_IF is set). When BUF_ERR_IF is set, the transfer maybe is not correct. If UA_IER\\[BUF_ERR_IEN\\]
is enabled, the buffer error interrupt will be generated. NOTE: This bit is cleared when both TX_OVER_IF and RX_OVER_IF are cleared."]
    #[inline(always)]
    pub fn buf_err_if(&self) -> BUF_ERR_IF_R {
        BUF_ERR_IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN Bus RX Break Field Detected Flag This bit is set when RX received LIN Break Field. If UA_IER\\[LIN_RX_BRK_IEN\\]
is enabled the LIN RX Break interrupt will be generated. NOTE: This bit is read only and user can write 1 to clear it."]
    #[inline(always)]
    pub fn lin_rx_break_if(&self) -> LIN_RX_BREAK_IF_R {
        LIN_RX_BREAK_IF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Data Available Interrupt Indicator (INT_RDA). This bit is set if RDA_IEN and RDA_IF are both set to 1. 1 = The RDA interrupt is generated. 0 = No RDA interrupt is generated ."]
    #[inline(always)]
    pub fn rda_int(&self) -> RDA_INT_R {
        RDA_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Holding Register Empty Interrupt Indicator (INT_THRE). This bit is set if THRE_IEN and THRE_IF are both set to 1. 1 = The THRE interrupt is generated. 0 = No THRE interrupt is generated."]
    #[inline(always)]
    pub fn thre_int(&self) -> THRE_INT_R {
        THRE_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Line Status Interrupt Indicator to (INT_RLS). This bit is set if RLS_IEN and RLS_IF are both set to 1. 1 = The RLS interrupt is generated. 0 = No RLS interrupt is generated."]
    #[inline(always)]
    pub fn rls_int(&self) -> RLS_INT_R {
        RLS_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MODEM Status Interrupt Indicator to (INT_MOS). This bit is set if MODEM_IEN and MODEM_IF are both set to 1.. 1 = The Modem interrupt is generated. 0 = No Modem interrupt is generated."]
    #[inline(always)]
    pub fn modem_int(&self) -> MODEM_INT_R {
        MODEM_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Time Out Interrupt Indicator (INT_Tout) This bit is set if TOUT_IEN and TOUT_IF are both set to 1. 1 = The Tout interrupt is generated. 0 = No Tout interrupt is generated."]
    #[inline(always)]
    pub fn tout_int(&self) -> TOUT_INT_R {
        TOUT_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Buffer Error Interrupt Indicator (INT_Buf_err) This bit is set if BUF_ERR_IEN and BUF_ERR_IF are both set to 1. 1 = The buffer error interrupt is generated. 0 = No buffer error interrupt is generated."]
    #[inline(always)]
    pub fn buf_err_int(&self) -> BUF_ERR_INT_R {
        BUF_ERR_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Bus Rx Break Field Detected Interrupt Indicator This bit is set if LIN_RX_BRK_IEN and LIN_RX_BREAK_IF are both set to 1. 1 = The LIN RX Break interrupt is generated. 0 = No LIN RX Break interrupt is generated."]
    #[inline(always)]
    pub fn lin_rx_break_int(&self) -> LIN_RX_BREAK_INT_R {
        LIN_RX_BREAK_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - In DMA mode, Receive Line Status Flag (Read Only) This bit is set when the Rx receive data have parity error, framing error or break error (at least one of 3 bits, BIF, FEF and PEF, is set). If IER\\[RLS_IEN\\]
is enabled, the RLS interrupt will be generated. NOTE: This bit is read only and reset to 0 when all bits of BIF, FEF and PEF are cleared."]
    #[inline(always)]
    pub fn hw_rls_if(&self) -> HW_RLS_IF_R {
        HW_RLS_IF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - In DMA mode, MODEM Interrupt Flag (Read Only) (not available in UART2 channel) This bit is set when the CTS pin has state change(DCTSF=1). if IER\\[Modem_IEN\\]
is enabled, the Modem interrupt will be generated. NOTE: This bit is read only and reset to 0 when bit DCTSF is cleared by a write 1 on DCTSF."]
    #[inline(always)]
    pub fn hw_modem_if(&self) -> HW_MODEM_IF_R {
        HW_MODEM_IF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - In DMA mode, Time out Interrupt Flag (Read Only) This bit is set when the Rx FIFO is not empty and no activities occurres in the Rx FIFO and the time out counter equal to TOIC. If IER\\[Tout_IEN\\]
is enabled, the Tout interrupt will be generated. NOTE: This bit is read only and user can read UA_RBR (Rx is in active) to clear it."]
    #[inline(always)]
    pub fn hw_tout_if(&self) -> HW_TOUT_IF_R {
        HW_TOUT_IF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - In DMA mode, Buffer Error Interrupt Flag (Read Only) This bit is set when the Tx or Rx FIFO overflows (Tx_Over_IF or Rx_Over_IF is set). When Buf_Err_IF is set, the transfer maybe is not correct. If IER\\[Buf_Err_IEN\\]
is enabled, the buffer error interrupt will be generated. NOTE: This bit is cleared when both Tx_Over_IF and Rx_Over_IF are cleared."]
    #[inline(always)]
    pub fn hw_buf_err_if(&self) -> HW_BUF_ERR_IF_R {
        HW_BUF_ERR_IF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - In DMA mode, LIN Bus Rx Break Field Detect Interrupt Flag This bit is set when Rx received LIN Break Field. If IER\\[LIN_RX_BRK_IEN\\]
is enabled the LIN RX Break interrupt will be generated. NOTE: This bit is read only and user can write 1 to clear it."]
    #[inline(always)]
    pub fn hw_lin_rx_break_if(&self) -> HW_LIN_RX_BREAK_IF_R {
        HW_LIN_RX_BREAK_IF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - In DMA mode, Receive Line Status Interrupt Indicator (INT_RLS). This bit is set if RLS_IEN and HW_RLS_IF are both set to 1. 1 = The RLS interrupt is generated in DMA mode. 0 = No RLS interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_rls_int(&self) -> HW_RLS_INT_R {
        HW_RLS_INT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - In DMA mode, MODEM Status Interrupt Indicator (INT_MOS)(not available in UART2 channel). This bit is set if MODEM_IEN and HW_MODEM_IF are both set to 1. 1 = The Modem interrupt is generated in DMA mode. 0 = No Modem interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_modem_int(&self) -> HW_MODEM_INT_R {
        HW_MODEM_INT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - In DMA mode, Time Out Interrupt Indicator (INT_Tout) This bit is set if TOUT_IEN and HW_TOUT_IF are both set to 1. 1 = The Tout interrupt is generated in DMA mode. 0 = No Tout interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_tout_int(&self) -> HW_TOUT_INT_R {
        HW_TOUT_INT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - In DMA mode, Buffer Error Interrupt Indicator(INT_Buf_err) This bit is set if BUF_ERR_IEN and HW_BUF_ERR_IF are both set to 1. 1 = The buffer error interrupt is generated in DMA mode. 0 = No buffer error interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_buf_err_int(&self) -> HW_BUF_ERR_INT_R {
        HW_BUF_ERR_INT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - In DMA mode, LIN Bus Rx Break Field Detected Interrupt Indicator This bit is set if LIN_RX_BRK_IEN and HW_LIN_RX_BREAK_IF are both set to 1. 1 = The LIN RX Break interrupt is generated in DMA mode. 0 = No LIN RX Break interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_lin_rx_break_int(&self) -> HW_LIN_RX_BREAK_INT_R {
        HW_LIN_RX_BREAK_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LIN Bus Rx Break Field Detected Interrupt Indicator This bit is set if LIN_RX_BRK_IEN and LIN_RX_BREAK_IF are both set to 1. 1 = The LIN RX Break interrupt is generated. 0 = No LIN RX Break interrupt is generated."]
    #[inline(always)]
    pub fn lin_rx_break_int(&mut self) -> LIN_RX_BREAK_INT_W {
        LIN_RX_BREAK_INT_W { w: self }
    }
    #[doc = "Bit 18 - In DMA mode, Receive Line Status Flag (Read Only) This bit is set when the Rx receive data have parity error, framing error or break error (at least one of 3 bits, BIF, FEF and PEF, is set). If IER\\[RLS_IEN\\]
is enabled, the RLS interrupt will be generated. NOTE: This bit is read only and reset to 0 when all bits of BIF, FEF and PEF are cleared."]
    #[inline(always)]
    pub fn hw_rls_if(&mut self) -> HW_RLS_IF_W {
        HW_RLS_IF_W { w: self }
    }
    #[doc = "Bit 19 - In DMA mode, MODEM Interrupt Flag (Read Only) (not available in UART2 channel) This bit is set when the CTS pin has state change(DCTSF=1). if IER\\[Modem_IEN\\]
is enabled, the Modem interrupt will be generated. NOTE: This bit is read only and reset to 0 when bit DCTSF is cleared by a write 1 on DCTSF."]
    #[inline(always)]
    pub fn hw_modem_if(&mut self) -> HW_MODEM_IF_W {
        HW_MODEM_IF_W { w: self }
    }
    #[doc = "Bit 20 - In DMA mode, Time out Interrupt Flag (Read Only) This bit is set when the Rx FIFO is not empty and no activities occurres in the Rx FIFO and the time out counter equal to TOIC. If IER\\[Tout_IEN\\]
is enabled, the Tout interrupt will be generated. NOTE: This bit is read only and user can read UA_RBR (Rx is in active) to clear it."]
    #[inline(always)]
    pub fn hw_tout_if(&mut self) -> HW_TOUT_IF_W {
        HW_TOUT_IF_W { w: self }
    }
    #[doc = "Bit 21 - In DMA mode, Buffer Error Interrupt Flag (Read Only) This bit is set when the Tx or Rx FIFO overflows (Tx_Over_IF or Rx_Over_IF is set). When Buf_Err_IF is set, the transfer maybe is not correct. If IER\\[Buf_Err_IEN\\]
is enabled, the buffer error interrupt will be generated. NOTE: This bit is cleared when both Tx_Over_IF and Rx_Over_IF are cleared."]
    #[inline(always)]
    pub fn hw_buf_err_if(&mut self) -> HW_BUF_ERR_IF_W {
        HW_BUF_ERR_IF_W { w: self }
    }
    #[doc = "Bit 23 - In DMA mode, LIN Bus Rx Break Field Detect Interrupt Flag This bit is set when Rx received LIN Break Field. If IER\\[LIN_RX_BRK_IEN\\]
is enabled the LIN RX Break interrupt will be generated. NOTE: This bit is read only and user can write 1 to clear it."]
    #[inline(always)]
    pub fn hw_lin_rx_break_if(&mut self) -> HW_LIN_RX_BREAK_IF_W {
        HW_LIN_RX_BREAK_IF_W { w: self }
    }
    #[doc = "Bit 26 - In DMA mode, Receive Line Status Interrupt Indicator (INT_RLS). This bit is set if RLS_IEN and HW_RLS_IF are both set to 1. 1 = The RLS interrupt is generated in DMA mode. 0 = No RLS interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_rls_int(&mut self) -> HW_RLS_INT_W {
        HW_RLS_INT_W { w: self }
    }
    #[doc = "Bit 27 - In DMA mode, MODEM Status Interrupt Indicator (INT_MOS)(not available in UART2 channel). This bit is set if MODEM_IEN and HW_MODEM_IF are both set to 1. 1 = The Modem interrupt is generated in DMA mode. 0 = No Modem interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_modem_int(&mut self) -> HW_MODEM_INT_W {
        HW_MODEM_INT_W { w: self }
    }
    #[doc = "Bit 28 - In DMA mode, Time Out Interrupt Indicator (INT_Tout) This bit is set if TOUT_IEN and HW_TOUT_IF are both set to 1. 1 = The Tout interrupt is generated in DMA mode. 0 = No Tout interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_tout_int(&mut self) -> HW_TOUT_INT_W {
        HW_TOUT_INT_W { w: self }
    }
    #[doc = "Bit 29 - In DMA mode, Buffer Error Interrupt Indicator(INT_Buf_err) This bit is set if BUF_ERR_IEN and HW_BUF_ERR_IF are both set to 1. 1 = The buffer error interrupt is generated in DMA mode. 0 = No buffer error interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_buf_err_int(&mut self) -> HW_BUF_ERR_INT_W {
        HW_BUF_ERR_INT_W { w: self }
    }
    #[doc = "Bit 31 - In DMA mode, LIN Bus Rx Break Field Detected Interrupt Indicator This bit is set if LIN_RX_BRK_IEN and HW_LIN_RX_BREAK_IF are both set to 1. 1 = The LIN RX Break interrupt is generated in DMA mode. 0 = No LIN RX Break interrupt is generated in DMA mode."]
    #[inline(always)]
    pub fn hw_lin_rx_break_int(&mut self) -> HW_LIN_RX_BREAK_INT_W {
        HW_LIN_RX_BREAK_INT_W { w: self }
    }
}
