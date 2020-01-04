#[doc = "Reader of register PDMA_SBUF0_cx"]
pub type R = crate::R<u32, super::PDMA_SBUF0_CX>;
#[doc = "Reader of field `PDMA_SBUF0`"]
pub type PDMA_SBUF0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PDMA Shared Buffer FIFO 0 (Read Only) Each channel has its own 1 word internal buffer."]
    #[inline(always)]
    pub fn pdma_sbuf0(&self) -> PDMA_SBUF0_R {
        PDMA_SBUF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
