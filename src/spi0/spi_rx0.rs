#[doc = "Reader of register SPI_RX0"]
pub type R = crate::R<u32, super::SPI_RX0>;
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Receive Register The Data Receive Registers hold the value of received data of the last executed transfer. The number of valid bits depend on the transmit bit length field in the SPI_CNTRL register. For example, if TX_BIT_LEN is set to 0x08 and TX_NUM is set to 0x0, bit RX0\\[7:0\\]
holds the received data. NOTE: The Data Receive Registers are read only registers."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
