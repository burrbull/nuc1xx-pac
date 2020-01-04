#[doc = "Reader of register PDMA_POINTx"]
pub type R = crate::R<u32, super::PDMA_POINTX>;
#[doc = "Reader of field `PDMA_POINT`"]
pub type PDMA_POINT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PDMA Internal Buffer Pointer Register (Read Only) This field indicates the internal buffer pointer."]
    #[inline(always)]
    pub fn pdma_point(&self) -> PDMA_POINT_R {
        PDMA_POINT_R::new((self.bits & 0x0f) as u8)
    }
}
