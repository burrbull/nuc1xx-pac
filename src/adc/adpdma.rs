#[doc = "Reader of register ADPDMA"]
pub type R = crate::R<u32, super::ADPDMA>;
#[doc = "Reader of field `AD_PDMA`"]
pub type AD_PDMA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC PDMA current transfer data register When PDMA transferring, read this register can monitor current PDMA transfer data. This is a read only register."]
    #[inline(always)]
    pub fn ad_pdma(&self) -> AD_PDMA_R {
        AD_PDMA_R::new((self.bits & 0x0fff) as u16)
    }
}
