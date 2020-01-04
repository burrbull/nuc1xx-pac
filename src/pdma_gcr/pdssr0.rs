#[doc = "Reader of register PDSSR0"]
pub type R = crate::R<u32, super::PDSSR0>;
#[doc = "Writer for register PDSSR0"]
pub type W = crate::W<u32, super::PDSSR0>;
#[doc = "Register PDSSR0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PDSSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SPI0_RXSEL`"]
pub type SPI0_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_RXSEL`"]
pub struct SPI0_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SPI0_TXSEL`"]
pub type SPI0_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_TXSEL`"]
pub struct SPI0_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI1_RXSEL`"]
pub type SPI1_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_RXSEL`"]
pub struct SPI1_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI1_TXSEL`"]
pub type SPI1_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_TXSEL`"]
pub struct SPI1_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI2_RXSEL`"]
pub type SPI2_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI2_RXSEL`"]
pub struct SPI2_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI2_TXSEL`"]
pub type SPI2_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI2_TXSEL`"]
pub struct SPI2_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI3_RXSEL`"]
pub type SPI3_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI3_RXSEL`"]
pub struct SPI3_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI3_TXSEL`"]
pub type SPI3_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI3_TXSEL`"]
pub struct SPI3_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PDMA SPI0 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI0 RX. Software can change the channel RX setting by SPI0_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : SPI0_RXSEL = 4'b0110, that means SPI0_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn spi0_rxsel(&self) -> SPI0_RXSEL_R {
        SPI0_RXSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDMA SPI0 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI0 TX. Software can configure the TX channel setting by SPI0_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi0_txsel(&self) -> SPI0_TXSEL_R {
        SPI0_TXSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDMA SPI1 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI1 RX. Software can configure the RX channel setting by SPI1_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi1_rxsel(&self) -> SPI1_RXSEL_R {
        SPI1_RXSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDMA SPI1 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI1 TX. Software can configure the TX channel setting by SPI1_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi1_txsel(&self) -> SPI1_TXSEL_R {
        SPI1_TXSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDMA SPI2 RX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI2 RX. Software can configure the RX channel setting by SPI2_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi2_rxsel(&self) -> SPI2_RXSEL_R {
        SPI2_RXSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PDMA SPI2 TX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI2 TX. Software can configure the TX channel setting by SPI2_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi2_txsel(&self) -> SPI2_TXSEL_R {
        SPI2_TXSEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDMA SPI3 RX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI3 RX. Software can configure the RX channel setting by SPI3_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi3_rxsel(&self) -> SPI3_RXSEL_R {
        SPI3_RXSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PDMA SPI3 TX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI3 TX. Software can configure the TX channel setting by SPI3_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi3_txsel(&self) -> SPI3_TXSEL_R {
        SPI3_TXSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDMA SPI0 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI0 RX. Software can change the channel RX setting by SPI0_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : SPI0_RXSEL = 4'b0110, that means SPI0_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn spi0_rxsel(&mut self) -> SPI0_RXSEL_W {
        SPI0_RXSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - PDMA SPI0 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI0 TX. Software can configure the TX channel setting by SPI0_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi0_txsel(&mut self) -> SPI0_TXSEL_W {
        SPI0_TXSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - PDMA SPI1 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI1 RX. Software can configure the RX channel setting by SPI1_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi1_rxsel(&mut self) -> SPI1_RXSEL_W {
        SPI1_RXSEL_W { w: self }
    }
    #[doc = "Bits 12:15 - PDMA SPI1 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral SPI1 TX. Software can configure the TX channel setting by SPI1_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi1_txsel(&mut self) -> SPI1_TXSEL_W {
        SPI1_TXSEL_W { w: self }
    }
    #[doc = "Bits 16:19 - PDMA SPI2 RX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI2 RX. Software can configure the RX channel setting by SPI2_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi2_rxsel(&mut self) -> SPI2_RXSEL_W {
        SPI2_RXSEL_W { w: self }
    }
    #[doc = "Bits 20:23 - PDMA SPI2 TX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI2 TX. Software can configure the TX channel setting by SPI2_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi2_txsel(&mut self) -> SPI2_TXSEL_W {
        SPI2_TXSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - PDMA SPI3 RX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI3 RX. Software can configure the RX channel setting by SPI3_RXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi3_rxsel(&mut self) -> SPI3_RXSEL_W {
        SPI3_RXSEL_W { w: self }
    }
    #[doc = "Bits 28:31 - PDMA SPI3 TX Selection (Medium Density Only) This filed defines which PDMA channel is connected to the on-chip peripheral SPI3 TX. Software can configure the TX channel setting by SPI3_TXSEL. The channel configuration is the same as SPI0_RXSEL field. Please refer to the explanation of SPI0_RXSEL."]
    #[inline(always)]
    pub fn spi3_txsel(&mut self) -> SPI3_TXSEL_W {
        SPI3_TXSEL_W { w: self }
    }
}
