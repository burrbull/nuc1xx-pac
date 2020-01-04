#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA Global Control Register"]
    pub pdma_gcrcsr: PDMA_GCRCSR,
    #[doc = "0x04 - PDMA Service Selection Control Register 0"]
    pub pdssr0: PDSSR0,
    #[doc = "0x08 - PDMA Service Selection Control Register 1"]
    pub pdssr1: PDSSR1,
    #[doc = "0x0c - PDMA Global Interrupt Register"]
    pub pdma_gcrisr: PDMA_GCRISR,
    #[doc = "0x10 - PDMA Service Selection Control Register 2"]
    pub pdssr2: PDSSR2,
}
#[doc = "PDMA Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_gcrcsr](pdma_gcrcsr) module"]
pub type PDMA_GCRCSR = crate::Reg<u32, _PDMA_GCRCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_GCRCSR;
#[doc = "`read()` method returns [pdma_gcrcsr::R](pdma_gcrcsr::R) reader structure"]
impl crate::Readable for PDMA_GCRCSR {}
#[doc = "`write(|w| ..)` method takes [pdma_gcrcsr::W](pdma_gcrcsr::W) writer structure"]
impl crate::Writable for PDMA_GCRCSR {}
#[doc = "PDMA Global Control Register"]
pub mod pdma_gcrcsr;
#[doc = "PDMA Service Selection Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdssr0](pdssr0) module"]
pub type PDSSR0 = crate::Reg<u32, _PDSSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSSR0;
#[doc = "`read()` method returns [pdssr0::R](pdssr0::R) reader structure"]
impl crate::Readable for PDSSR0 {}
#[doc = "`write(|w| ..)` method takes [pdssr0::W](pdssr0::W) writer structure"]
impl crate::Writable for PDSSR0 {}
#[doc = "PDMA Service Selection Control Register 0"]
pub mod pdssr0;
#[doc = "PDMA Service Selection Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdssr1](pdssr1) module"]
pub type PDSSR1 = crate::Reg<u32, _PDSSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSSR1;
#[doc = "`read()` method returns [pdssr1::R](pdssr1::R) reader structure"]
impl crate::Readable for PDSSR1 {}
#[doc = "`write(|w| ..)` method takes [pdssr1::W](pdssr1::W) writer structure"]
impl crate::Writable for PDSSR1 {}
#[doc = "PDMA Service Selection Control Register 1"]
pub mod pdssr1;
#[doc = "PDMA Global Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_gcrisr](pdma_gcrisr) module"]
pub type PDMA_GCRISR = crate::Reg<u32, _PDMA_GCRISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_GCRISR;
#[doc = "`read()` method returns [pdma_gcrisr::R](pdma_gcrisr::R) reader structure"]
impl crate::Readable for PDMA_GCRISR {}
#[doc = "PDMA Global Interrupt Register"]
pub mod pdma_gcrisr;
#[doc = "PDMA Service Selection Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdssr2](pdssr2) module"]
pub type PDSSR2 = crate::Reg<u32, _PDSSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSSR2;
#[doc = "`read()` method returns [pdssr2::R](pdssr2::R) reader structure"]
impl crate::Readable for PDSSR2 {}
#[doc = "`write(|w| ..)` method takes [pdssr2::W](pdssr2::W) writer structure"]
impl crate::Writable for PDSSR2 {}
#[doc = "PDMA Service Selection Control Register 2"]
pub mod pdssr2;
