#[doc = "Reader of register UA_IER"]
pub type R = crate::R<u32, super::UA_IER>;
#[doc = "Writer for register UA_IER"]
pub type W = crate::W<u32, super::UA_IER>;
#[doc = "Register UA_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDA_IEN`"]
pub type RDA_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDA_IEN`"]
pub struct RDA_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDA_IEN_W<'a> {
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
#[doc = "Reader of field `THRE_IEN`"]
pub type THRE_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THRE_IEN`"]
pub struct THRE_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRE_IEN_W<'a> {
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
#[doc = "Reader of field `RLS_IEN`"]
pub type RLS_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RLS_IEN`"]
pub struct RLS_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLS_IEN_W<'a> {
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
#[doc = "Reader of field `MODEM_IEN`"]
pub type MODEM_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEM_IEN`"]
pub struct MODEM_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEM_IEN_W<'a> {
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
#[doc = "Reader of field `RTO_IEN`"]
pub type RTO_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTO_IEN`"]
pub struct RTO_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BUF_ERR_IEN`"]
pub type BUF_ERR_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_ERR_IEN`"]
pub struct BUF_ERR_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_ERR_IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WAKE_EN`"]
pub type WAKE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKE_EN`"]
pub struct WAKE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_EN_W<'a> {
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
#[doc = "Reader of field `LIN_RX_BRK_IEN`"]
pub type LIN_RX_BRK_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIN_RX_BRK_IEN`"]
pub struct LIN_RX_BRK_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_RX_BRK_IEN_W<'a> {
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
#[doc = "Reader of field `TIME_OUT_EN`"]
pub type TIME_OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_OUT_EN`"]
pub struct TIME_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_EN_W<'a> {
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
#[doc = "Reader of field `AUTO_RTS_EN`"]
pub type AUTO_RTS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RTS_EN`"]
pub struct AUTO_RTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RTS_EN_W<'a> {
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
#[doc = "Reader of field `AUTO_CTS_EN`"]
pub type AUTO_CTS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_CTS_EN`"]
pub struct AUTO_CTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CTS_EN_W<'a> {
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
#[doc = "Reader of field `DMA_TX_EN`"]
pub type DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TX_EN`"]
pub struct DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_EN_W<'a> {
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
#[doc = "Reader of field `DMA_RX_EN`"]
pub type DMA_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_RX_EN`"]
pub struct DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_EN_W<'a> {
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
    #[doc = "Bit 0 - Receive Data Available Interrupt Enable. 0 = Mask off INT_RDA 1 = Enable INT_RDA"]
    #[inline(always)]
    pub fn rda_ien(&self) -> RDA_IEN_R {
        RDA_IEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Enable 0 = Mask off INT_THRE 1 = Enable INT_THRE"]
    #[inline(always)]
    pub fn thre_ien(&self) -> THRE_IEN_R {
        THRE_IEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable 0 = Mask off INT_RLS 1 = Enable INT_RLS"]
    #[inline(always)]
    pub fn rls_ien(&self) -> RLS_IEN_R {
        RLS_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable 0 = Mask off INT_MODEM 1 = Enable INT_MODEM"]
    #[inline(always)]
    pub fn modem_ien(&self) -> MODEM_IEN_R {
        MODEM_IEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx Time out Interrupt Enable 0 = Mask off INT_TOUT 1 = Enable INT_TOUT."]
    #[inline(always)]
    pub fn rto_ien(&self) -> RTO_IEN_R {
        RTO_IEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Enable 0 = Mask off INT_Buf_ERR 1 = Enable INT_Buf_ERR"]
    #[inline(always)]
    pub fn buf_err_ien(&self) -> BUF_ERR_IEN_R {
        BUF_ERR_IEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake up CPU function enable 0 = Disable UART wake up CPU function 1 = Enable wake up function, when the system is in deep sleep mode, an external /CTS change will wake up CPU from deep sleep mode."]
    #[inline(always)]
    pub fn wake_en(&self) -> WAKE_EN_R {
        WAKE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN RX Break Field Detected Interrupt Enable 0 = Mask off Lin bus Rx break filed interrupt. 1 = Enable Lin bus Rx break filed interrupt. Note: This field is used for LIN function mode."]
    #[inline(always)]
    pub fn lin_rx_brk_ien(&self) -> LIN_RX_BRK_IEN_R {
        LIN_RX_BRK_IEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time-Out Counter Enable 1 = Enable Time-out counter. 0 = Disable Time-out counter."]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTS Auto Flow Control Enable 1 = Enable RTS auto flow control. 0 = Disable RTS auto flow control. When RTS auto-flow is enabled, if the number of bytes in the Rx FIFO equals the UA_FCR\\[RTS_Tri_Lev\\], the UART will de-assert RTS signal."]
    #[inline(always)]
    pub fn auto_rts_en(&self) -> AUTO_RTS_EN_R {
        AUTO_RTS_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CTS Auto Flow Control Enable 1 = Enable CTS auto flow control. 0 = Disable CTS auto flow control. When CTS auto-flow is enabled, the UART will send data to external device when CTS input assert (UART will not send data to device until CTS is asserted)."]
    #[inline(always)]
    pub fn auto_cts_en(&self) -> AUTO_CTS_EN_R {
        AUTO_CTS_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TX DMA Enable 1 = Enable TX DMA. 0 = Disable TX DMA."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Time-Out Counter Enable 1 = Enable RX DMA. 0 = Disable RX DMA."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Available Interrupt Enable. 0 = Mask off INT_RDA 1 = Enable INT_RDA"]
    #[inline(always)]
    pub fn rda_ien(&mut self) -> RDA_IEN_W {
        RDA_IEN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Enable 0 = Mask off INT_THRE 1 = Enable INT_THRE"]
    #[inline(always)]
    pub fn thre_ien(&mut self) -> THRE_IEN_W {
        THRE_IEN_W { w: self }
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable 0 = Mask off INT_RLS 1 = Enable INT_RLS"]
    #[inline(always)]
    pub fn rls_ien(&mut self) -> RLS_IEN_W {
        RLS_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable 0 = Mask off INT_MODEM 1 = Enable INT_MODEM"]
    #[inline(always)]
    pub fn modem_ien(&mut self) -> MODEM_IEN_W {
        MODEM_IEN_W { w: self }
    }
    #[doc = "Bit 4 - Rx Time out Interrupt Enable 0 = Mask off INT_TOUT 1 = Enable INT_TOUT."]
    #[inline(always)]
    pub fn rto_ien(&mut self) -> RTO_IEN_W {
        RTO_IEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Enable 0 = Mask off INT_Buf_ERR 1 = Enable INT_Buf_ERR"]
    #[inline(always)]
    pub fn buf_err_ien(&mut self) -> BUF_ERR_IEN_W {
        BUF_ERR_IEN_W { w: self }
    }
    #[doc = "Bit 6 - Wake up CPU function enable 0 = Disable UART wake up CPU function 1 = Enable wake up function, when the system is in deep sleep mode, an external /CTS change will wake up CPU from deep sleep mode."]
    #[inline(always)]
    pub fn wake_en(&mut self) -> WAKE_EN_W {
        WAKE_EN_W { w: self }
    }
    #[doc = "Bit 8 - LIN RX Break Field Detected Interrupt Enable 0 = Mask off Lin bus Rx break filed interrupt. 1 = Enable Lin bus Rx break filed interrupt. Note: This field is used for LIN function mode."]
    #[inline(always)]
    pub fn lin_rx_brk_ien(&mut self) -> LIN_RX_BRK_IEN_W {
        LIN_RX_BRK_IEN_W { w: self }
    }
    #[doc = "Bit 11 - Time-Out Counter Enable 1 = Enable Time-out counter. 0 = Disable Time-out counter."]
    #[inline(always)]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W {
        TIME_OUT_EN_W { w: self }
    }
    #[doc = "Bit 12 - RTS Auto Flow Control Enable 1 = Enable RTS auto flow control. 0 = Disable RTS auto flow control. When RTS auto-flow is enabled, if the number of bytes in the Rx FIFO equals the UA_FCR\\[RTS_Tri_Lev\\], the UART will de-assert RTS signal."]
    #[inline(always)]
    pub fn auto_rts_en(&mut self) -> AUTO_RTS_EN_W {
        AUTO_RTS_EN_W { w: self }
    }
    #[doc = "Bit 13 - CTS Auto Flow Control Enable 1 = Enable CTS auto flow control. 0 = Disable CTS auto flow control. When CTS auto-flow is enabled, the UART will send data to external device when CTS input assert (UART will not send data to device until CTS is asserted)."]
    #[inline(always)]
    pub fn auto_cts_en(&mut self) -> AUTO_CTS_EN_W {
        AUTO_CTS_EN_W { w: self }
    }
    #[doc = "Bit 14 - TX DMA Enable 1 = Enable TX DMA. 0 = Disable TX DMA."]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W {
        DMA_TX_EN_W { w: self }
    }
    #[doc = "Bit 15 - Time-Out Counter Enable 1 = Enable RX DMA. 0 = Disable RX DMA."]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W {
        DMA_RX_EN_W { w: self }
    }
}
