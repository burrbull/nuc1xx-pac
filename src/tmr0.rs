#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0 Control and Status Register"]
    pub tcsr: TCSR,
    #[doc = "0x04 - Timer0 Compare Register"]
    pub tcmpr: TCMPR,
    #[doc = "0x08 - Timer0 Interrupt Status Register"]
    pub tisr: TISR,
    #[doc = "0x0c - Timer0 Data Register"]
    pub tdr: TDR,
}
#[doc = "Timer0 Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr](tcsr) module"]
pub type TCSR = crate::Reg<u32, _TCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCSR;
#[doc = "`read()` method returns [tcsr::R](tcsr::R) reader structure"]
impl crate::Readable for TCSR {}
#[doc = "`write(|w| ..)` method takes [tcsr::W](tcsr::W) writer structure"]
impl crate::Writable for TCSR {}
#[doc = "Timer0 Control and Status Register"]
pub mod tcsr;
#[doc = "Timer0 Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmpr](tcmpr) module"]
pub type TCMPR = crate::Reg<u32, _TCMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCMPR;
#[doc = "`read()` method returns [tcmpr::R](tcmpr::R) reader structure"]
impl crate::Readable for TCMPR {}
#[doc = "`write(|w| ..)` method takes [tcmpr::W](tcmpr::W) writer structure"]
impl crate::Writable for TCMPR {}
#[doc = "Timer0 Compare Register"]
pub mod tcmpr;
#[doc = "Timer0 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisr](tisr) module"]
pub type TISR = crate::Reg<u32, _TISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TISR;
#[doc = "`read()` method returns [tisr::R](tisr::R) reader structure"]
impl crate::Readable for TISR {}
#[doc = "`write(|w| ..)` method takes [tisr::W](tisr::W) writer structure"]
impl crate::Writable for TISR {}
#[doc = "Timer0 Interrupt Status Register"]
pub mod tisr;
#[doc = "Timer0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`read()` method returns [tdr::R](tdr::R) reader structure"]
impl crate::Readable for TDR {}
#[doc = "Timer0 Data Register"]
pub mod tdr;
