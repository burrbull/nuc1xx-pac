#[doc = "Reader of register CAN_TXREQ2"]
pub type R = crate::R<u32, super::CAN_TXREQ2>;
#[doc = "Reader of field `TxRqst17_32`"]
pub type TXRQST17_32_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission Request Bits 17-32 (of all Message Objects) 1 = The transmission of this Message Object is requested and is not yet done. 0 = This Message Object is not waiting for transmission. These bits are read only."]
    #[inline(always)]
    pub fn tx_rqst17_32(&self) -> TXRQST17_32_R {
        TXRQST17_32_R::new((self.bits & 0xffff) as u16)
    }
}
