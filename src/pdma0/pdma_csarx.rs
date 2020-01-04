#[doc = "Reader of register PDMA_CSARx"]
pub type R = crate::R<u32, super::PDMA_CSARX>;
#[doc = "Reader of field `PDMA_CSAR`"]
pub type PDMA_CSAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PDMA Current Source Address Register (Read Only) This field indicates the source address where the PDMA transfer is just occurring."]
    #[inline(always)]
    pub fn pdma_csar(&self) -> PDMA_CSAR_R {
        PDMA_CSAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
