#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA Control and Status Register CHx"]
    pub pdma_csrx: PDMA_CSRX,
    #[doc = "0x04 - PDMA Transfer Source Address Register CHx"]
    pub pdma_sarx: PDMA_SARX,
    #[doc = "0x08 - PDMA Transfer Destination Address Register CHx"]
    pub pdma_darx: PDMA_DARX,
    #[doc = "0x0c - PDMA Transfer Byte Count Register CHx"]
    pub pdma_bcrx: PDMA_BCRX,
    #[doc = "0x10 - PDMA Internal Buffer Pointer Register CHx"]
    pub pdma_pointx: PDMA_POINTX,
    #[doc = "0x14 - PDMA Current Source Address Register CHx"]
    pub pdma_csarx: PDMA_CSARX,
    #[doc = "0x18 - PDMA Current Destination Address Register CHx"]
    pub pdma_cdarx: PDMA_CDARX,
    #[doc = "0x1c - PDMA Current Byte Count Register CHx"]
    pub pdma_cbcrx: PDMA_CBCRX,
    #[doc = "0x20 - PDMA Interrupt Enable Control Register CHx"]
    pub pdma_ierx: PDMA_IERX,
    #[doc = "0x24 - PDMA Interrupt Status Register CHx"]
    pub pdma_isrx: PDMA_ISRX,
    _reserved10: [u8; 88usize],
    #[doc = "0x80 - PDMA Shared Buffer FIFO 0 Register CHx"]
    pub pdma_sbuf0_cx: PDMA_SBUF0_CX,
}
#[doc = "PDMA Control and Status Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_csrx](pdma_csrx) module"]
pub type PDMA_CSRX = crate::Reg<u32, _PDMA_CSRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CSRX;
#[doc = "`read()` method returns [pdma_csrx::R](pdma_csrx::R) reader structure"]
impl crate::Readable for PDMA_CSRX {}
#[doc = "`write(|w| ..)` method takes [pdma_csrx::W](pdma_csrx::W) writer structure"]
impl crate::Writable for PDMA_CSRX {}
#[doc = "PDMA Control and Status Register CHx"]
pub mod pdma_csrx;
#[doc = "PDMA Transfer Source Address Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_sarx](pdma_sarx) module"]
pub type PDMA_SARX = crate::Reg<u32, _PDMA_SARX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_SARX;
#[doc = "`read()` method returns [pdma_sarx::R](pdma_sarx::R) reader structure"]
impl crate::Readable for PDMA_SARX {}
#[doc = "`write(|w| ..)` method takes [pdma_sarx::W](pdma_sarx::W) writer structure"]
impl crate::Writable for PDMA_SARX {}
#[doc = "PDMA Transfer Source Address Register CHx"]
pub mod pdma_sarx;
#[doc = "PDMA Transfer Destination Address Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_darx](pdma_darx) module"]
pub type PDMA_DARX = crate::Reg<u32, _PDMA_DARX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_DARX;
#[doc = "`read()` method returns [pdma_darx::R](pdma_darx::R) reader structure"]
impl crate::Readable for PDMA_DARX {}
#[doc = "`write(|w| ..)` method takes [pdma_darx::W](pdma_darx::W) writer structure"]
impl crate::Writable for PDMA_DARX {}
#[doc = "PDMA Transfer Destination Address Register CHx"]
pub mod pdma_darx;
#[doc = "PDMA Transfer Byte Count Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_bcrx](pdma_bcrx) module"]
pub type PDMA_BCRX = crate::Reg<u32, _PDMA_BCRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_BCRX;
#[doc = "`read()` method returns [pdma_bcrx::R](pdma_bcrx::R) reader structure"]
impl crate::Readable for PDMA_BCRX {}
#[doc = "`write(|w| ..)` method takes [pdma_bcrx::W](pdma_bcrx::W) writer structure"]
impl crate::Writable for PDMA_BCRX {}
#[doc = "PDMA Transfer Byte Count Register CHx"]
pub mod pdma_bcrx;
#[doc = "PDMA Internal Buffer Pointer Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_pointx](pdma_pointx) module"]
pub type PDMA_POINTX = crate::Reg<u32, _PDMA_POINTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_POINTX;
#[doc = "`read()` method returns [pdma_pointx::R](pdma_pointx::R) reader structure"]
impl crate::Readable for PDMA_POINTX {}
#[doc = "PDMA Internal Buffer Pointer Register CHx"]
pub mod pdma_pointx;
#[doc = "PDMA Current Source Address Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_csarx](pdma_csarx) module"]
pub type PDMA_CSARX = crate::Reg<u32, _PDMA_CSARX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CSARX;
#[doc = "`read()` method returns [pdma_csarx::R](pdma_csarx::R) reader structure"]
impl crate::Readable for PDMA_CSARX {}
#[doc = "PDMA Current Source Address Register CHx"]
pub mod pdma_csarx;
#[doc = "PDMA Current Destination Address Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_cdarx](pdma_cdarx) module"]
pub type PDMA_CDARX = crate::Reg<u32, _PDMA_CDARX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CDARX;
#[doc = "`read()` method returns [pdma_cdarx::R](pdma_cdarx::R) reader structure"]
impl crate::Readable for PDMA_CDARX {}
#[doc = "PDMA Current Destination Address Register CHx"]
pub mod pdma_cdarx;
#[doc = "PDMA Current Byte Count Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_cbcrx](pdma_cbcrx) module"]
pub type PDMA_CBCRX = crate::Reg<u32, _PDMA_CBCRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CBCRX;
#[doc = "`read()` method returns [pdma_cbcrx::R](pdma_cbcrx::R) reader structure"]
impl crate::Readable for PDMA_CBCRX {}
#[doc = "PDMA Current Byte Count Register CHx"]
pub mod pdma_cbcrx;
#[doc = "PDMA Interrupt Enable Control Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ierx](pdma_ierx) module"]
pub type PDMA_IERX = crate::Reg<u32, _PDMA_IERX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_IERX;
#[doc = "`read()` method returns [pdma_ierx::R](pdma_ierx::R) reader structure"]
impl crate::Readable for PDMA_IERX {}
#[doc = "`write(|w| ..)` method takes [pdma_ierx::W](pdma_ierx::W) writer structure"]
impl crate::Writable for PDMA_IERX {}
#[doc = "PDMA Interrupt Enable Control Register CHx"]
pub mod pdma_ierx;
#[doc = "PDMA Interrupt Status Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_isrx](pdma_isrx) module"]
pub type PDMA_ISRX = crate::Reg<u32, _PDMA_ISRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISRX;
#[doc = "`read()` method returns [pdma_isrx::R](pdma_isrx::R) reader structure"]
impl crate::Readable for PDMA_ISRX {}
#[doc = "`write(|w| ..)` method takes [pdma_isrx::W](pdma_isrx::W) writer structure"]
impl crate::Writable for PDMA_ISRX {}
#[doc = "PDMA Interrupt Status Register CHx"]
pub mod pdma_isrx;
#[doc = "PDMA Shared Buffer FIFO 0 Register CHx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_sbuf0_cx](pdma_sbuf0_cx) module"]
pub type PDMA_SBUF0_CX = crate::Reg<u32, _PDMA_SBUF0_CX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_SBUF0_CX;
#[doc = "`read()` method returns [pdma_sbuf0_cx::R](pdma_sbuf0_cx::R) reader structure"]
impl crate::Readable for PDMA_SBUF0_CX {}
#[doc = "PDMA Shared Buffer FIFO 0 Register CHx"]
pub mod pdma_sbuf0_cx;
