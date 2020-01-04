#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - SysTick Control and Status"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x14 - SysTick Reload value"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x18 - SysTick Current value"]
    pub syst_cvr: SYST_CVR,
    _reserved3: [u8; 228usize],
    #[doc = "0x100 - IRQ0 ~ IRQ31 Set-Enable Control Register"]
    pub nvic_iser: NVIC_ISER,
    _reserved4: [u8; 124usize],
    #[doc = "0x180 - IRQ0 ~ IRQ31 Clear-Enable Control Register"]
    pub nvic_icer: NVIC_ICER,
    _reserved5: [u8; 124usize],
    #[doc = "0x200 - IRQ0 ~ IRQ31 Set-Pending Control Register"]
    pub nvic_ispr: NVIC_ISPR,
    _reserved6: [u8; 124usize],
    #[doc = "0x280 - IRQ0 ~ IRQ31 Clear-Pending Control Register"]
    pub nvic_icpr: NVIC_ICPR,
    _reserved7: [u8; 380usize],
    #[doc = "0x400 - IRQ0 ~ IRQ3 Priority Control Register"]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - IRQ4 ~ IRQ7 Priority Control Register"]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - IRQ8 ~ IRQ11 Priority Control Register"]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - IRQ12 ~ IRQ15 Priority Control Register"]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - IRQ16 ~ IRQ19 Priority Control Register"]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - IRQ20 ~ IRQ23 Priority Control Register"]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - IRQ24 ~ IRQ27 Priority Control Register"]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - IRQ28 ~ IRQ31 Priority Control Register"]
    pub nvic_ipr7: NVIC_IPR7,
    _reserved15: [u8; 2272usize],
    #[doc = "0xd00 - CPUID Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control State Register"]
    pub icsr: ICSR,
    _reserved17: [u8; 4usize],
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    _reserved19: [u8; 8usize],
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
}
#[doc = "SysTick Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_csr](syst_csr) module"]
pub type SYST_CSR = crate::Reg<u32, _SYST_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CSR;
#[doc = "`read()` method returns [syst_csr::R](syst_csr::R) reader structure"]
impl crate::Readable for SYST_CSR {}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](syst_csr::W) writer structure"]
impl crate::Writable for SYST_CSR {}
#[doc = "SysTick Control and Status"]
pub mod syst_csr;
#[doc = "SysTick Reload value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_rvr](syst_rvr) module"]
pub type SYST_RVR = crate::Reg<u32, _SYST_RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_RVR;
#[doc = "`read()` method returns [syst_rvr::R](syst_rvr::R) reader structure"]
impl crate::Readable for SYST_RVR {}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](syst_rvr::W) writer structure"]
impl crate::Writable for SYST_RVR {}
#[doc = "SysTick Reload value"]
pub mod syst_rvr;
#[doc = "SysTick Current value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_cvr](syst_cvr) module"]
pub type SYST_CVR = crate::Reg<u32, _SYST_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CVR;
#[doc = "`read()` method returns [syst_cvr::R](syst_cvr::R) reader structure"]
impl crate::Readable for SYST_CVR {}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](syst_cvr::W) writer structure"]
impl crate::Writable for SYST_CVR {}
#[doc = "SysTick Current value"]
pub mod syst_cvr;
#[doc = "IRQ0 ~ IRQ31 Set-Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser](nvic_iser) module"]
pub type NVIC_ISER = crate::Reg<u32, _NVIC_ISER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER;
#[doc = "`read()` method returns [nvic_iser::R](nvic_iser::R) reader structure"]
impl crate::Readable for NVIC_ISER {}
#[doc = "`write(|w| ..)` method takes [nvic_iser::W](nvic_iser::W) writer structure"]
impl crate::Writable for NVIC_ISER {}
#[doc = "IRQ0 ~ IRQ31 Set-Enable Control Register"]
pub mod nvic_iser;
#[doc = "IRQ0 ~ IRQ31 Clear-Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer](nvic_icer) module"]
pub type NVIC_ICER = crate::Reg<u32, _NVIC_ICER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER;
#[doc = "`read()` method returns [nvic_icer::R](nvic_icer::R) reader structure"]
impl crate::Readable for NVIC_ICER {}
#[doc = "`write(|w| ..)` method takes [nvic_icer::W](nvic_icer::W) writer structure"]
impl crate::Writable for NVIC_ICER {}
#[doc = "IRQ0 ~ IRQ31 Clear-Enable Control Register"]
pub mod nvic_icer;
#[doc = "IRQ0 ~ IRQ31 Set-Pending Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr](nvic_ispr) module"]
pub type NVIC_ISPR = crate::Reg<u32, _NVIC_ISPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR;
#[doc = "`read()` method returns [nvic_ispr::R](nvic_ispr::R) reader structure"]
impl crate::Readable for NVIC_ISPR {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr::W](nvic_ispr::W) writer structure"]
impl crate::Writable for NVIC_ISPR {}
#[doc = "IRQ0 ~ IRQ31 Set-Pending Control Register"]
pub mod nvic_ispr;
#[doc = "IRQ0 ~ IRQ31 Clear-Pending Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr](nvic_icpr) module"]
pub type NVIC_ICPR = crate::Reg<u32, _NVIC_ICPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR;
#[doc = "`read()` method returns [nvic_icpr::R](nvic_icpr::R) reader structure"]
impl crate::Readable for NVIC_ICPR {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr::W](nvic_icpr::W) writer structure"]
impl crate::Writable for NVIC_ICPR {}
#[doc = "IRQ0 ~ IRQ31 Clear-Pending Control Register"]
pub mod nvic_icpr;
#[doc = "IRQ0 ~ IRQ3 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr0](nvic_ipr0) module"]
pub type NVIC_IPR0 = crate::Reg<u32, _NVIC_IPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR0;
#[doc = "`read()` method returns [nvic_ipr0::R](nvic_ipr0::R) reader structure"]
impl crate::Readable for NVIC_IPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](nvic_ipr0::W) writer structure"]
impl crate::Writable for NVIC_IPR0 {}
#[doc = "IRQ0 ~ IRQ3 Priority Control Register"]
pub mod nvic_ipr0;
#[doc = "IRQ4 ~ IRQ7 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr1](nvic_ipr1) module"]
pub type NVIC_IPR1 = crate::Reg<u32, _NVIC_IPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR1;
#[doc = "`read()` method returns [nvic_ipr1::R](nvic_ipr1::R) reader structure"]
impl crate::Readable for NVIC_IPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](nvic_ipr1::W) writer structure"]
impl crate::Writable for NVIC_IPR1 {}
#[doc = "IRQ4 ~ IRQ7 Priority Control Register"]
pub mod nvic_ipr1;
#[doc = "IRQ8 ~ IRQ11 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr2](nvic_ipr2) module"]
pub type NVIC_IPR2 = crate::Reg<u32, _NVIC_IPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR2;
#[doc = "`read()` method returns [nvic_ipr2::R](nvic_ipr2::R) reader structure"]
impl crate::Readable for NVIC_IPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](nvic_ipr2::W) writer structure"]
impl crate::Writable for NVIC_IPR2 {}
#[doc = "IRQ8 ~ IRQ11 Priority Control Register"]
pub mod nvic_ipr2;
#[doc = "IRQ12 ~ IRQ15 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr3](nvic_ipr3) module"]
pub type NVIC_IPR3 = crate::Reg<u32, _NVIC_IPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR3;
#[doc = "`read()` method returns [nvic_ipr3::R](nvic_ipr3::R) reader structure"]
impl crate::Readable for NVIC_IPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](nvic_ipr3::W) writer structure"]
impl crate::Writable for NVIC_IPR3 {}
#[doc = "IRQ12 ~ IRQ15 Priority Control Register"]
pub mod nvic_ipr3;
#[doc = "IRQ16 ~ IRQ19 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr4](nvic_ipr4) module"]
pub type NVIC_IPR4 = crate::Reg<u32, _NVIC_IPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR4;
#[doc = "`read()` method returns [nvic_ipr4::R](nvic_ipr4::R) reader structure"]
impl crate::Readable for NVIC_IPR4 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](nvic_ipr4::W) writer structure"]
impl crate::Writable for NVIC_IPR4 {}
#[doc = "IRQ16 ~ IRQ19 Priority Control Register"]
pub mod nvic_ipr4;
#[doc = "IRQ20 ~ IRQ23 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr5](nvic_ipr5) module"]
pub type NVIC_IPR5 = crate::Reg<u32, _NVIC_IPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR5;
#[doc = "`read()` method returns [nvic_ipr5::R](nvic_ipr5::R) reader structure"]
impl crate::Readable for NVIC_IPR5 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](nvic_ipr5::W) writer structure"]
impl crate::Writable for NVIC_IPR5 {}
#[doc = "IRQ20 ~ IRQ23 Priority Control Register"]
pub mod nvic_ipr5;
#[doc = "IRQ24 ~ IRQ27 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr6](nvic_ipr6) module"]
pub type NVIC_IPR6 = crate::Reg<u32, _NVIC_IPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR6;
#[doc = "`read()` method returns [nvic_ipr6::R](nvic_ipr6::R) reader structure"]
impl crate::Readable for NVIC_IPR6 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](nvic_ipr6::W) writer structure"]
impl crate::Writable for NVIC_IPR6 {}
#[doc = "IRQ24 ~ IRQ27 Priority Control Register"]
pub mod nvic_ipr6;
#[doc = "IRQ28 ~ IRQ31 Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr7](nvic_ipr7) module"]
pub type NVIC_IPR7 = crate::Reg<u32, _NVIC_IPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR7;
#[doc = "`read()` method returns [nvic_ipr7::R](nvic_ipr7::R) reader structure"]
impl crate::Readable for NVIC_IPR7 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](nvic_ipr7::W) writer structure"]
impl crate::Writable for NVIC_IPR7 {}
#[doc = "IRQ28 ~ IRQ31 Priority Control Register"]
pub mod nvic_ipr7;
#[doc = "CPUID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPUID Register"]
pub mod cpuid;
#[doc = "Interrupt Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control State Register"]
pub mod icsr;
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
