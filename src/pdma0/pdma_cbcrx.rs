#[doc = "Reader of register PDMA_CBCRx"]
pub type R = crate::R<u32, super::PDMA_CBCRX>;
#[doc = "Reader of field `PDMA_CBCR`"]
pub type PDMA_CBCR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PDMA Current Byte Count Register (Read Only) This field indicates the current remained byte count of PDMA. Note : SW_RST will clear this register value."]
    #[inline(always)]
    pub fn pdma_cbcr(&self) -> PDMA_CBCR_R {
        PDMA_CBCR_R::new((self.bits & 0xffff) as u16)
    }
}
