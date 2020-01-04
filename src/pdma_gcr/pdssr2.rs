#[doc = "Reader of register PDSSR2"]
pub type R = crate::R<u32, super::PDSSR2>;
#[doc = "Writer for register PDSSR2"]
pub type W = crate::W<u32, super::PDSSR2>;
#[doc = "Register PDSSR2 `reset()`'s with value 0xff"]
impl crate::ResetValue for super::PDSSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `I2S_RXSEL`"]
pub type I2S_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_RXSEL`"]
pub struct I2S_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `I2S_TXSEL`"]
pub type I2S_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TXSEL`"]
pub struct I2S_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PDMA I2S RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral I2S RX. Software can change the channel RX setting by I2S_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : I2S_RXSEL = 4'b0110, that means I2S_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn i2s_rxsel(&self) -> I2S_RXSEL_R {
        I2S_RXSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDMA I2S TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral I2S TX. Software can configure the TX channel setting by I2S_TXSEL. The channel configuration is the same as I2S_RXSEL field. Please refer to the explanation of I2S_RXSEL."]
    #[inline(always)]
    pub fn i2s_txsel(&self) -> I2S_TXSEL_R {
        I2S_TXSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDMA I2S RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral I2S RX. Software can change the channel RX setting by I2S_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : I2S_RXSEL = 4'b0110, that means I2S_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn i2s_rxsel(&mut self) -> I2S_RXSEL_W {
        I2S_RXSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - PDMA I2S TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral I2S TX. Software can configure the TX channel setting by I2S_TXSEL. The channel configuration is the same as I2S_RXSEL field. Please refer to the explanation of I2S_RXSEL."]
    #[inline(always)]
    pub fn i2s_txsel(&mut self) -> I2S_TXSEL_W {
        I2S_TXSEL_W { w: self }
    }
}
