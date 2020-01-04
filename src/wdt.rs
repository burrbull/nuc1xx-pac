#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    pub wtcr: WTCR,
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcr](wtcr) module"]
pub type WTCR = crate::Reg<u32, _WTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCR;
#[doc = "`read()` method returns [wtcr::R](wtcr::R) reader structure"]
impl crate::Readable for WTCR {}
#[doc = "`write(|w| ..)` method takes [wtcr::W](wtcr::W) writer structure"]
impl crate::Writable for WTCR {}
#[doc = "Watchdog Timer Control Register"]
pub mod wtcr;
