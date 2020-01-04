#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Prescaler Register"]
    pub ppr: PPR,
    #[doc = "0x04 - PWM Clock Select Register"]
    pub csr: CSR,
    #[doc = "0x08 - PWM Control Register"]
    pub pcr: PCR,
    #[doc = "0x0c - PWM Counter Register 0"]
    pub cnr0: CNR0,
    #[doc = "0x10 - PWM Comparator Register 0"]
    pub cmr0: CMR0,
    #[doc = "0x14 - PWM Data Register 0"]
    pub pdr0: PDR0,
    #[doc = "0x18 - PWM Counter Register 1"]
    pub cnr1: CNR1,
    #[doc = "0x1c - PWM Comparator Register 1"]
    pub cmr1: CMR1,
    #[doc = "0x20 - PWM Data Register 1"]
    pub pdr1: PDR1,
    #[doc = "0x24 - PWM Counter Register 2"]
    pub cnr2: CNR2,
    #[doc = "0x28 - PWM Comparator Register 2"]
    pub cmr2: CMR2,
    #[doc = "0x2c - PWM Data Register 2"]
    pub pdr2: PDR2,
    #[doc = "0x30 - PWM Counter Register 3"]
    pub cnr3: CNR3,
    #[doc = "0x34 - PWM Comparator Register 3"]
    pub cmr3: CMR3,
    #[doc = "0x38 - PWM Data Register 3"]
    pub pdr3: PDR3,
    #[doc = "0x3c - New description for register"]
    pub pbcr: PBCR,
    #[doc = "0x40 - PWM Interrupt Enable Register"]
    pub pier: PIER,
    #[doc = "0x44 - PWM Interrupt Indication Register"]
    pub piir: PIIR,
    _reserved18: [u8; 8usize],
    #[doc = "0x50 - Capture Control Register 0"]
    pub ccr0: CCR0,
    #[doc = "0x54 - Capture Control Register 2"]
    pub ccr2: CCR2,
    #[doc = "0x58 - Capture Rising Latch Register (Channel 0)"]
    pub crlr0: CRLR0,
    #[doc = "0x5c - Capture Falling Latch Register (Channel 0)"]
    pub cflr0: CFLR0,
    #[doc = "0x60 - Capture Rising Latch Register (Channel 1)"]
    pub crlr1: CRLR1,
    #[doc = "0x64 - Capture Falling Latch Register (Channel 1)"]
    pub cflr1: CFLR1,
    #[doc = "0x68 - Capture Rising Latch Register (channel 2)"]
    pub crlr2: CRLR2,
    #[doc = "0x6c - Capture Falling Latch Register (channel 2)"]
    pub cflr2: CFLR2,
    #[doc = "0x70 - Capture Rising Latch Register (channel 3)"]
    pub crlr3: CRLR3,
    #[doc = "0x74 - Capture Falling Latch Register (channel 3)"]
    pub cflr3: CFLR3,
    #[doc = "0x78 - Capture Input Enable Register"]
    pub capenr: CAPENR,
    #[doc = "0x7c - PWM Output Enable Register"]
    pub poe: POE,
}
#[doc = "PWM Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppr](ppr) module"]
pub type PPR = crate::Reg<u32, _PPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPR;
#[doc = "`read()` method returns [ppr::R](ppr::R) reader structure"]
impl crate::Readable for PPR {}
#[doc = "`write(|w| ..)` method takes [ppr::W](ppr::W) writer structure"]
impl crate::Writable for PPR {}
#[doc = "PWM Prescaler Register"]
pub mod ppr;
#[doc = "PWM Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "PWM Clock Select Register"]
pub mod csr;
#[doc = "PWM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "PWM Control Register"]
pub mod pcr;
#[doc = "PWM Counter Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr0](cnr0) module"]
pub type CNR0 = crate::Reg<u32, _CNR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNR0;
#[doc = "`read()` method returns [cnr0::R](cnr0::R) reader structure"]
impl crate::Readable for CNR0 {}
#[doc = "`write(|w| ..)` method takes [cnr0::W](cnr0::W) writer structure"]
impl crate::Writable for CNR0 {}
#[doc = "PWM Counter Register 0"]
pub mod cnr0;
#[doc = "PWM Comparator Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr0](cmr0) module"]
pub type CMR0 = crate::Reg<u32, _CMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR0;
#[doc = "`read()` method returns [cmr0::R](cmr0::R) reader structure"]
impl crate::Readable for CMR0 {}
#[doc = "`write(|w| ..)` method takes [cmr0::W](cmr0::W) writer structure"]
impl crate::Writable for CMR0 {}
#[doc = "PWM Comparator Register 0"]
pub mod cmr0;
#[doc = "PWM Data Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr0](pdr0) module"]
pub type PDR0 = crate::Reg<u32, _PDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR0;
#[doc = "`read()` method returns [pdr0::R](pdr0::R) reader structure"]
impl crate::Readable for PDR0 {}
#[doc = "PWM Data Register 0"]
pub mod pdr0;
#[doc = "PWM Counter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr1](cnr1) module"]
pub type CNR1 = crate::Reg<u32, _CNR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNR1;
#[doc = "`read()` method returns [cnr1::R](cnr1::R) reader structure"]
impl crate::Readable for CNR1 {}
#[doc = "`write(|w| ..)` method takes [cnr1::W](cnr1::W) writer structure"]
impl crate::Writable for CNR1 {}
#[doc = "PWM Counter Register 1"]
pub mod cnr1;
#[doc = "PWM Comparator Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr1](cmr1) module"]
pub type CMR1 = crate::Reg<u32, _CMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR1;
#[doc = "`read()` method returns [cmr1::R](cmr1::R) reader structure"]
impl crate::Readable for CMR1 {}
#[doc = "`write(|w| ..)` method takes [cmr1::W](cmr1::W) writer structure"]
impl crate::Writable for CMR1 {}
#[doc = "PWM Comparator Register 1"]
pub mod cmr1;
#[doc = "PWM Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr1](pdr1) module"]
pub type PDR1 = crate::Reg<u32, _PDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR1;
#[doc = "`read()` method returns [pdr1::R](pdr1::R) reader structure"]
impl crate::Readable for PDR1 {}
#[doc = "PWM Data Register 1"]
pub mod pdr1;
#[doc = "PWM Counter Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr2](cnr2) module"]
pub type CNR2 = crate::Reg<u32, _CNR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNR2;
#[doc = "`read()` method returns [cnr2::R](cnr2::R) reader structure"]
impl crate::Readable for CNR2 {}
#[doc = "`write(|w| ..)` method takes [cnr2::W](cnr2::W) writer structure"]
impl crate::Writable for CNR2 {}
#[doc = "PWM Counter Register 2"]
pub mod cnr2;
#[doc = "PWM Comparator Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr2](cmr2) module"]
pub type CMR2 = crate::Reg<u32, _CMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR2;
#[doc = "`read()` method returns [cmr2::R](cmr2::R) reader structure"]
impl crate::Readable for CMR2 {}
#[doc = "`write(|w| ..)` method takes [cmr2::W](cmr2::W) writer structure"]
impl crate::Writable for CMR2 {}
#[doc = "PWM Comparator Register 2"]
pub mod cmr2;
#[doc = "PWM Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr2](pdr2) module"]
pub type PDR2 = crate::Reg<u32, _PDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR2;
#[doc = "`read()` method returns [pdr2::R](pdr2::R) reader structure"]
impl crate::Readable for PDR2 {}
#[doc = "PWM Data Register 2"]
pub mod pdr2;
#[doc = "PWM Counter Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr3](cnr3) module"]
pub type CNR3 = crate::Reg<u32, _CNR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNR3;
#[doc = "`read()` method returns [cnr3::R](cnr3::R) reader structure"]
impl crate::Readable for CNR3 {}
#[doc = "`write(|w| ..)` method takes [cnr3::W](cnr3::W) writer structure"]
impl crate::Writable for CNR3 {}
#[doc = "PWM Counter Register 3"]
pub mod cnr3;
#[doc = "PWM Comparator Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr3](cmr3) module"]
pub type CMR3 = crate::Reg<u32, _CMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR3;
#[doc = "`read()` method returns [cmr3::R](cmr3::R) reader structure"]
impl crate::Readable for CMR3 {}
#[doc = "`write(|w| ..)` method takes [cmr3::W](cmr3::W) writer structure"]
impl crate::Writable for CMR3 {}
#[doc = "PWM Comparator Register 3"]
pub mod cmr3;
#[doc = "PWM Data Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr3](pdr3) module"]
pub type PDR3 = crate::Reg<u32, _PDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR3;
#[doc = "`read()` method returns [pdr3::R](pdr3::R) reader structure"]
impl crate::Readable for PDR3 {}
#[doc = "PWM Data Register 3"]
pub mod pdr3;
#[doc = "New description for register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcr](pbcr) module"]
pub type PBCR = crate::Reg<u32, _PBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBCR;
#[doc = "`read()` method returns [pbcr::R](pbcr::R) reader structure"]
impl crate::Readable for PBCR {}
#[doc = "`write(|w| ..)` method takes [pbcr::W](pbcr::W) writer structure"]
impl crate::Writable for PBCR {}
#[doc = "New description for register"]
pub mod pbcr;
#[doc = "PWM Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pier](pier) module"]
pub type PIER = crate::Reg<u32, _PIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIER;
#[doc = "`read()` method returns [pier::R](pier::R) reader structure"]
impl crate::Readable for PIER {}
#[doc = "`write(|w| ..)` method takes [pier::W](pier::W) writer structure"]
impl crate::Writable for PIER {}
#[doc = "PWM Interrupt Enable Register"]
pub mod pier;
#[doc = "PWM Interrupt Indication Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piir](piir) module"]
pub type PIIR = crate::Reg<u32, _PIIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIIR;
#[doc = "`read()` method returns [piir::R](piir::R) reader structure"]
impl crate::Readable for PIIR {}
#[doc = "`write(|w| ..)` method takes [piir::W](piir::W) writer structure"]
impl crate::Writable for PIIR {}
#[doc = "PWM Interrupt Indication Register"]
pub mod piir;
#[doc = "Capture Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr0](ccr0) module"]
pub type CCR0 = crate::Reg<u32, _CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR0;
#[doc = "`read()` method returns [ccr0::R](ccr0::R) reader structure"]
impl crate::Readable for CCR0 {}
#[doc = "`write(|w| ..)` method takes [ccr0::W](ccr0::W) writer structure"]
impl crate::Writable for CCR0 {}
#[doc = "Capture Control Register 0"]
pub mod ccr0;
#[doc = "Capture Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2](ccr2) module"]
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
#[doc = "`read()` method returns [ccr2::R](ccr2::R) reader structure"]
impl crate::Readable for CCR2 {}
#[doc = "`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure"]
impl crate::Writable for CCR2 {}
#[doc = "Capture Control Register 2"]
pub mod ccr2;
#[doc = "Capture Rising Latch Register (Channel 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crlr0](crlr0) module"]
pub type CRLR0 = crate::Reg<u32, _CRLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRLR0;
#[doc = "`read()` method returns [crlr0::R](crlr0::R) reader structure"]
impl crate::Readable for CRLR0 {}
#[doc = "Capture Rising Latch Register (Channel 0)"]
pub mod crlr0;
#[doc = "Capture Falling Latch Register (Channel 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cflr0](cflr0) module"]
pub type CFLR0 = crate::Reg<u32, _CFLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFLR0;
#[doc = "`read()` method returns [cflr0::R](cflr0::R) reader structure"]
impl crate::Readable for CFLR0 {}
#[doc = "Capture Falling Latch Register (Channel 0)"]
pub mod cflr0;
#[doc = "Capture Rising Latch Register (Channel 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crlr1](crlr1) module"]
pub type CRLR1 = crate::Reg<u32, _CRLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRLR1;
#[doc = "`read()` method returns [crlr1::R](crlr1::R) reader structure"]
impl crate::Readable for CRLR1 {}
#[doc = "Capture Rising Latch Register (Channel 1)"]
pub mod crlr1;
#[doc = "Capture Falling Latch Register (Channel 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cflr1](cflr1) module"]
pub type CFLR1 = crate::Reg<u32, _CFLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFLR1;
#[doc = "`read()` method returns [cflr1::R](cflr1::R) reader structure"]
impl crate::Readable for CFLR1 {}
#[doc = "Capture Falling Latch Register (Channel 1)"]
pub mod cflr1;
#[doc = "Capture Rising Latch Register (channel 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crlr2](crlr2) module"]
pub type CRLR2 = crate::Reg<u32, _CRLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRLR2;
#[doc = "`read()` method returns [crlr2::R](crlr2::R) reader structure"]
impl crate::Readable for CRLR2 {}
#[doc = "Capture Rising Latch Register (channel 2)"]
pub mod crlr2;
#[doc = "Capture Falling Latch Register (channel 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cflr2](cflr2) module"]
pub type CFLR2 = crate::Reg<u32, _CFLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFLR2;
#[doc = "`read()` method returns [cflr2::R](cflr2::R) reader structure"]
impl crate::Readable for CFLR2 {}
#[doc = "Capture Falling Latch Register (channel 2)"]
pub mod cflr2;
#[doc = "Capture Rising Latch Register (channel 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crlr3](crlr3) module"]
pub type CRLR3 = crate::Reg<u32, _CRLR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRLR3;
#[doc = "`read()` method returns [crlr3::R](crlr3::R) reader structure"]
impl crate::Readable for CRLR3 {}
#[doc = "Capture Rising Latch Register (channel 3)"]
pub mod crlr3;
#[doc = "Capture Falling Latch Register (channel 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cflr3](cflr3) module"]
pub type CFLR3 = crate::Reg<u32, _CFLR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFLR3;
#[doc = "`read()` method returns [cflr3::R](cflr3::R) reader structure"]
impl crate::Readable for CFLR3 {}
#[doc = "Capture Falling Latch Register (channel 3)"]
pub mod cflr3;
#[doc = "Capture Input Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capenr](capenr) module"]
pub type CAPENR = crate::Reg<u32, _CAPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPENR;
#[doc = "`read()` method returns [capenr::R](capenr::R) reader structure"]
impl crate::Readable for CAPENR {}
#[doc = "`write(|w| ..)` method takes [capenr::W](capenr::W) writer structure"]
impl crate::Writable for CAPENR {}
#[doc = "Capture Input Enable Register"]
pub mod capenr;
#[doc = "PWM Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poe](poe) module"]
pub type POE = crate::Reg<u32, _POE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POE;
#[doc = "`read()` method returns [poe::R](poe::R) reader structure"]
impl crate::Readable for POE {}
#[doc = "`write(|w| ..)` method takes [poe::W](poe::W) writer structure"]
impl crate::Writable for POE {}
#[doc = "PWM Output Enable Register"]
pub mod poe;
