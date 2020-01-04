#[doc = "Reader of register PDMA_CDARx"]
pub type R = crate::R<u32, super::PDMA_CDARX>;
#[doc = "Reader of field `PDMA_CDAR`"]
pub type PDMA_CDAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PDMA Current Destination Address Register (Read Only) This field indicates the destination address where the PDMA transfer is just occurring."]
    #[inline(always)]
    pub fn pdma_cdar(&self) -> PDMA_CDAR_R {
        PDMA_CDAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
