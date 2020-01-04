#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Part Device Identification Number Register"]
    pub pdid: PDID,
    #[doc = "0x04 - System Reset Source Register"]
    pub rstsrc: RSTSRC,
    #[doc = "0x08 - IP Reset Control Resister1"]
    pub iprstc1: IPRSTC1,
    #[doc = "0x0c - IP Reset Control Resister 2"]
    pub iprstc2: IPRSTC2,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Brown Out Detector Control Register"]
    pub bodcr: BODCR,
    #[doc = "0x1c - Temperature Sensor Control Register"]
    pub tempcr: TEMPCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - Power-On-Reset Controller Register"]
    pub porcr: PORCR,
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - GPIOA multiple function and input type control register"]
    pub gpa_mfp: GPA_MFP,
    #[doc = "0x34 - GPIOB multiple function and input type control register"]
    pub gpb_mfp: GPB_MFP,
    #[doc = "0x38 - GPIOC multiple function and input type control register"]
    pub gpc_mfp: GPC_MFP,
    #[doc = "0x3c - GPIOD multiple function and input type control register"]
    pub gpd_mfp: GPD_MFP,
    #[doc = "0x40 - GPIOE multiple function and input type control register"]
    pub gpe_mfp: GPE_MFP,
    _reserved12: [u8; 12usize],
    #[doc = "0x50 - Alternative Multiple Function Pin Control Register"]
    pub alt_mfp: ALT_MFP,
    _reserved13: [u8; 172usize],
    #[doc = "0x100 - Register Write Protect Register"]
    pub regwrprot: REGWRPROT,
}
#[doc = "Part Device Identification Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdid](pdid) module"]
pub type PDID = crate::Reg<u32, _PDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDID;
#[doc = "`read()` method returns [pdid::R](pdid::R) reader structure"]
impl crate::Readable for PDID {}
#[doc = "Part Device Identification Number Register"]
pub mod pdid;
#[doc = "System Reset Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsrc](rstsrc) module"]
pub type RSTSRC = crate::Reg<u32, _RSTSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSRC;
#[doc = "`read()` method returns [rstsrc::R](rstsrc::R) reader structure"]
impl crate::Readable for RSTSRC {}
#[doc = "`write(|w| ..)` method takes [rstsrc::W](rstsrc::W) writer structure"]
impl crate::Writable for RSTSRC {}
#[doc = "System Reset Source Register"]
pub mod rstsrc;
#[doc = "IP Reset Control Resister1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprstc1](iprstc1) module"]
pub type IPRSTC1 = crate::Reg<u32, _IPRSTC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRSTC1;
#[doc = "`read()` method returns [iprstc1::R](iprstc1::R) reader structure"]
impl crate::Readable for IPRSTC1 {}
#[doc = "`write(|w| ..)` method takes [iprstc1::W](iprstc1::W) writer structure"]
impl crate::Writable for IPRSTC1 {}
#[doc = "IP Reset Control Resister1"]
pub mod iprstc1;
#[doc = "IP Reset Control Resister 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprstc2](iprstc2) module"]
pub type IPRSTC2 = crate::Reg<u32, _IPRSTC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRSTC2;
#[doc = "`read()` method returns [iprstc2::R](iprstc2::R) reader structure"]
impl crate::Readable for IPRSTC2 {}
#[doc = "`write(|w| ..)` method takes [iprstc2::W](iprstc2::W) writer structure"]
impl crate::Writable for IPRSTC2 {}
#[doc = "IP Reset Control Resister 2"]
pub mod iprstc2;
#[doc = "Brown Out Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcr](bodcr) module"]
pub type BODCR = crate::Reg<u32, _BODCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCR;
#[doc = "`read()` method returns [bodcr::R](bodcr::R) reader structure"]
impl crate::Readable for BODCR {}
#[doc = "`write(|w| ..)` method takes [bodcr::W](bodcr::W) writer structure"]
impl crate::Writable for BODCR {}
#[doc = "Brown Out Detector Control Register"]
pub mod bodcr;
#[doc = "Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempcr](tempcr) module"]
pub type TEMPCR = crate::Reg<u32, _TEMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPCR;
#[doc = "`read()` method returns [tempcr::R](tempcr::R) reader structure"]
impl crate::Readable for TEMPCR {}
#[doc = "`write(|w| ..)` method takes [tempcr::W](tempcr::W) writer structure"]
impl crate::Writable for TEMPCR {}
#[doc = "Temperature Sensor Control Register"]
pub mod tempcr;
#[doc = "Power-On-Reset Controller Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porcr](porcr) module"]
pub type PORCR = crate::Reg<u32, _PORCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORCR;
#[doc = "`read()` method returns [porcr::R](porcr::R) reader structure"]
impl crate::Readable for PORCR {}
#[doc = "`write(|w| ..)` method takes [porcr::W](porcr::W) writer structure"]
impl crate::Writable for PORCR {}
#[doc = "Power-On-Reset Controller Register"]
pub mod porcr;
#[doc = "GPIOA multiple function and input type control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpa_mfp](gpa_mfp) module"]
pub type GPA_MFP = crate::Reg<u32, _GPA_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPA_MFP;
#[doc = "`read()` method returns [gpa_mfp::R](gpa_mfp::R) reader structure"]
impl crate::Readable for GPA_MFP {}
#[doc = "`write(|w| ..)` method takes [gpa_mfp::W](gpa_mfp::W) writer structure"]
impl crate::Writable for GPA_MFP {}
#[doc = "GPIOA multiple function and input type control register"]
pub mod gpa_mfp;
#[doc = "GPIOB multiple function and input type control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpb_mfp](gpb_mfp) module"]
pub type GPB_MFP = crate::Reg<u32, _GPB_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPB_MFP;
#[doc = "`read()` method returns [gpb_mfp::R](gpb_mfp::R) reader structure"]
impl crate::Readable for GPB_MFP {}
#[doc = "`write(|w| ..)` method takes [gpb_mfp::W](gpb_mfp::W) writer structure"]
impl crate::Writable for GPB_MFP {}
#[doc = "GPIOB multiple function and input type control register"]
pub mod gpb_mfp;
#[doc = "GPIOC multiple function and input type control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpc_mfp](gpc_mfp) module"]
pub type GPC_MFP = crate::Reg<u32, _GPC_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPC_MFP;
#[doc = "`read()` method returns [gpc_mfp::R](gpc_mfp::R) reader structure"]
impl crate::Readable for GPC_MFP {}
#[doc = "`write(|w| ..)` method takes [gpc_mfp::W](gpc_mfp::W) writer structure"]
impl crate::Writable for GPC_MFP {}
#[doc = "GPIOC multiple function and input type control register"]
pub mod gpc_mfp;
#[doc = "GPIOD multiple function and input type control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpd_mfp](gpd_mfp) module"]
pub type GPD_MFP = crate::Reg<u32, _GPD_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPD_MFP;
#[doc = "`read()` method returns [gpd_mfp::R](gpd_mfp::R) reader structure"]
impl crate::Readable for GPD_MFP {}
#[doc = "`write(|w| ..)` method takes [gpd_mfp::W](gpd_mfp::W) writer structure"]
impl crate::Writable for GPD_MFP {}
#[doc = "GPIOD multiple function and input type control register"]
pub mod gpd_mfp;
#[doc = "GPIOE multiple function and input type control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpe_mfp](gpe_mfp) module"]
pub type GPE_MFP = crate::Reg<u32, _GPE_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPE_MFP;
#[doc = "`read()` method returns [gpe_mfp::R](gpe_mfp::R) reader structure"]
impl crate::Readable for GPE_MFP {}
#[doc = "`write(|w| ..)` method takes [gpe_mfp::W](gpe_mfp::W) writer structure"]
impl crate::Writable for GPE_MFP {}
#[doc = "GPIOE multiple function and input type control register"]
pub mod gpe_mfp;
#[doc = "Alternative Multiple Function Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_mfp](alt_mfp) module"]
pub type ALT_MFP = crate::Reg<u32, _ALT_MFP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALT_MFP;
#[doc = "`read()` method returns [alt_mfp::R](alt_mfp::R) reader structure"]
impl crate::Readable for ALT_MFP {}
#[doc = "`write(|w| ..)` method takes [alt_mfp::W](alt_mfp::W) writer structure"]
impl crate::Writable for ALT_MFP {}
#[doc = "Alternative Multiple Function Pin Control Register"]
pub mod alt_mfp;
#[doc = "Register Write Protect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regwrprot](regwrprot) module"]
pub type REGWRPROT = crate::Reg<u32, _REGWRPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGWRPROT;
#[doc = "`read()` method returns [regwrprot::R](regwrprot::R) reader structure"]
impl crate::Readable for REGWRPROT {}
#[doc = "`write(|w| ..)` method takes [regwrprot::W](regwrprot::W) writer structure"]
impl crate::Writable for REGWRPROT {}
#[doc = "Register Write Protect Register"]
pub mod regwrprot;
