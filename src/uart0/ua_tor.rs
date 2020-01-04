#[doc = "Reader of register UA_TOR"]
pub type R = crate::R<u32, super::UA_TOR>;
#[doc = "Writer for register UA_TOR"]
pub type W = crate::W<u32, super::UA_TOR>;
#[doc = "Register UA_TOR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_TOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOIC`"]
pub type TOIC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOIC`"]
pub struct TOIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `DLY`"]
pub type DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY`"]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Time Out Interrupt Comparator The time out counter resets and starts counting (the counting clock = baud rate) whenever the RX FIFO receives a new data word. Once the content of time out counter (TOUT_CNT) is equal to that of time out interrupt comparator (TOIC), a receiver time out interrupt (INTR_TOUT) is generated if UA_IER \\[RTO_IEN\\]. A new incoming data word or RX FIFO empty clears INTR_TOUT."]
    #[inline(always)]
    pub fn toic(&self) -> TOIC_R {
        TOIC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - TX Delay time value (Low Density Only) This field is use to programming the transfer delay time between the last stop bit and next start bit."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Time Out Interrupt Comparator The time out counter resets and starts counting (the counting clock = baud rate) whenever the RX FIFO receives a new data word. Once the content of time out counter (TOUT_CNT) is equal to that of time out interrupt comparator (TOIC), a receiver time out interrupt (INTR_TOUT) is generated if UA_IER \\[RTO_IEN\\]. A new incoming data word or RX FIFO empty clears INTR_TOUT."]
    #[inline(always)]
    pub fn toic(&mut self) -> TOIC_W {
        TOIC_W { w: self }
    }
    #[doc = "Bits 8:15 - TX Delay time value (Low Density Only) This field is use to programming the transfer delay time between the last stop bit and next start bit."]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
}
