#[doc = "Reader of register UA_RBR"]
pub type R = crate::R<u32, super::UA_RBR>;
#[doc = "Reader of field `RBR`"]
pub type RBR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Buffer Register By reading this register, the UART will return an 8-bit data received from Rx pin (LSB first)."]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
