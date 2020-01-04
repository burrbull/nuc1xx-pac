#[doc = "Reader of register PDSSR1"]
pub type R = crate::R<u32, super::PDSSR1>;
#[doc = "Writer for register PDSSR1"]
pub type W = crate::W<u32, super::PDSSR1>;
#[doc = "Register PDSSR1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PDSSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `UART0_RXSEL`"]
pub type UART0_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART0_RXSEL`"]
pub struct UART0_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `UART0_TXSEL`"]
pub type UART0_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART0_TXSEL`"]
pub struct UART0_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART1_RXSEL`"]
pub type UART1_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART1_RXSEL`"]
pub struct UART1_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART1_TXSEL`"]
pub type UART1_TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART1_TXSEL`"]
pub struct UART1_TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC_RXSEL`"]
pub type ADC_RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_RXSEL`"]
pub struct ADC_RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - This filed defines which PDMA channel is connected to the on-chip peripheral UART0 RX. Software can change the channel RX setting by UART0_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : UART0_RXSEL = 4'b0110, that means UART0_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn uart0_rxsel(&self) -> UART0_RXSEL_R {
        UART0_RXSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDMA UART0 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART0 TX. Software can configure the TX channel setting by UART0_TXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart0_txsel(&self) -> UART0_TXSEL_R {
        UART0_TXSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDMA UART1 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART1 RX. Software can configure the RX channel setting by UART1_RXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart1_rxsel(&self) -> UART1_RXSEL_R {
        UART1_RXSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDMA UART1 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART1 TX. Software can configure the TX channel setting by UART1_TXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart1_txsel(&self) -> UART1_TXSEL_R {
        UART1_TXSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDMA ADC RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral ADC RX. Software can configure the RX channel setting by ADC_RXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn adc_rxsel(&self) -> ADC_RXSEL_R {
        ADC_RXSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This filed defines which PDMA channel is connected to the on-chip peripheral UART0 RX. Software can change the channel RX setting by UART0_RXSEL 4'b0000: CH0 4'b0001: CH1 4'b0010: CH2 4'b0011: CH3 4'b0100: CH4 4'b0101: CH5 4'b0110: CH6 4'b0111: CH7 4'b1000: CH8 Others : Reserved Note : Ex : UART0_RXSEL = 4'b0110, that means UART0_RX is connected to PDMA_CH6(Low Density should set as 4'b0000 for PDMA channel 0 only)"]
    #[inline(always)]
    pub fn uart0_rxsel(&mut self) -> UART0_RXSEL_W {
        UART0_RXSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - PDMA UART0 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART0 TX. Software can configure the TX channel setting by UART0_TXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart0_txsel(&mut self) -> UART0_TXSEL_W {
        UART0_TXSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - PDMA UART1 RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART1 RX. Software can configure the RX channel setting by UART1_RXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart1_rxsel(&mut self) -> UART1_RXSEL_W {
        UART1_RXSEL_W { w: self }
    }
    #[doc = "Bits 12:15 - PDMA UART1 TX Selection This filed defines which PDMA channel is connected to the on-chip peripheral UART1 TX. Software can configure the TX channel setting by UART1_TXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn uart1_txsel(&mut self) -> UART1_TXSEL_W {
        UART1_TXSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - PDMA ADC RX Selection This filed defines which PDMA channel is connected to the on-chip peripheral ADC RX. Software can configure the RX channel setting by ADC_RXSEL. The channel configuration is the same as UART0_RXSEL field. Please refer to the explanation of UART0_RXSEL"]
    #[inline(always)]
    pub fn adc_rxsel(&mut self) -> ADC_RXSEL_W {
        ADC_RXSEL_W { w: self }
    }
}
