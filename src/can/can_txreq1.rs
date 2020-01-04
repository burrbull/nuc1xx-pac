#[doc = "Reader of register CAN_TXREQ1"]
pub type R = crate::R<u32, super::CAN_TXREQ1>;
#[doc = "Reader of field `TxRqst1_16`"]
pub type TXRQST1_16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission Request Bits 1-16 (of all Message Objects) 1 = The transmission of this Message Object is requested and is not yet done. 0 = This Message Object is not waiting for transmission. These bits are read only."]
    #[inline(always)]
    pub fn tx_rqst1_16(&self) -> TXRQST1_16_R {
        TXRQST1_16_R::new((self.bits & 0xffff) as u16)
    }
}
