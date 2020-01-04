#[doc = "Reader of register SPI_DMA"]
pub type R = crate::R<u32, super::SPI_DMA>;
#[doc = "Writer for register SPI_DMA"]
pub type W = crate::W<u32, super::SPI_DMA>;
#[doc = "Register SPI_DMA `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_DMA_GO`"]
pub type TX_DMA_GO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DMA_GO`"]
pub struct TX_DMA_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_GO_W<'a> {
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
#[doc = "Reader of field `RX_DMA_GO`"]
pub type RX_DMA_GO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DMA_GO`"]
pub struct RX_DMA_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_GO_W<'a> {
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
    #[doc = "Bit 0 - Transmit DMA start Set this bit to 1 will start the transmit DMA process. SPI module will issue request to DMA module automatically. If using DMA mode to transfer data, remember not to set GO_BUSY bit of SPI_CNTRL register. The DMA controller inside SPI module will set it automatically whenever necessary. Hardware will clear this bit automatically after DMA transfer done."]
    #[inline(always)]
    pub fn tx_dma_go(&self) -> TX_DMA_GO_R {
        TX_DMA_GO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive DMA start Set this bit to 1 will start the receive DMA process. SPI module will issue request to DMA module automatically. Hardware will clear this bit automatically after DMA transfer done."]
    #[inline(always)]
    pub fn rx_dma_go(&self) -> RX_DMA_GO_R {
        RX_DMA_GO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit DMA start Set this bit to 1 will start the transmit DMA process. SPI module will issue request to DMA module automatically. If using DMA mode to transfer data, remember not to set GO_BUSY bit of SPI_CNTRL register. The DMA controller inside SPI module will set it automatically whenever necessary. Hardware will clear this bit automatically after DMA transfer done."]
    #[inline(always)]
    pub fn tx_dma_go(&mut self) -> TX_DMA_GO_W {
        TX_DMA_GO_W { w: self }
    }
    #[doc = "Bit 1 - Receive DMA start Set this bit to 1 will start the receive DMA process. SPI module will issue request to DMA module automatically. Hardware will clear this bit automatically after DMA transfer done."]
    #[inline(always)]
    pub fn rx_dma_go(&mut self) -> RX_DMA_GO_W {
        RX_DMA_GO_W { w: self }
    }
}
