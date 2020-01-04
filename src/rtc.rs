#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Initiation Register"]
    pub inir: INIR,
    #[doc = "0x04 - RTC Access Enable Register"]
    pub aer: AER,
    #[doc = "0x08 - RTC Frequency Compensation Register"]
    pub fcr: FCR,
    #[doc = "0x0c - Time Loading Register"]
    pub tlr: TLR,
    #[doc = "0x10 - Calendar Loading Register"]
    pub clr: CLR,
    #[doc = "0x14 - Time Scale Selection Register"]
    pub tssr: TSSR,
    #[doc = "0x18 - Day of the Week Register"]
    pub dwr: DWR,
    #[doc = "0x1c - Time Alarm Register"]
    pub tar: TAR,
    #[doc = "0x20 - Calendar Alarm Register"]
    pub car: CAR,
    #[doc = "0x24 - RTC Leap year Indicator Register"]
    pub lir: LIR,
    #[doc = "0x28 - RTC Interrupt Enable Register"]
    pub rier: RIER,
    #[doc = "0x2c - RTC Interrupt Indicator Register"]
    pub riir: RIIR,
    #[doc = "0x30 - RTC Time Tick Register"]
    pub ttr: TTR,
}
#[doc = "RTC Initiation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inir](inir) module"]
pub type INIR = crate::Reg<u32, _INIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIR;
#[doc = "`read()` method returns [inir::R](inir::R) reader structure"]
impl crate::Readable for INIR {}
#[doc = "`write(|w| ..)` method takes [inir::W](inir::W) writer structure"]
impl crate::Writable for INIR {}
#[doc = "RTC Initiation Register"]
pub mod inir;
#[doc = "RTC Access Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aer](aer) module"]
pub type AER = crate::Reg<u32, _AER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AER;
#[doc = "`read()` method returns [aer::R](aer::R) reader structure"]
impl crate::Readable for AER {}
#[doc = "`write(|w| ..)` method takes [aer::W](aer::W) writer structure"]
impl crate::Writable for AER {}
#[doc = "RTC Access Enable Register"]
pub mod aer;
#[doc = "RTC Frequency Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "RTC Frequency Compensation Register"]
pub mod fcr;
#[doc = "Time Loading Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlr](tlr) module"]
pub type TLR = crate::Reg<u32, _TLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLR;
#[doc = "`read()` method returns [tlr::R](tlr::R) reader structure"]
impl crate::Readable for TLR {}
#[doc = "`write(|w| ..)` method takes [tlr::W](tlr::W) writer structure"]
impl crate::Writable for TLR {}
#[doc = "Time Loading Register"]
pub mod tlr;
#[doc = "Calendar Loading Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`read()` method returns [clr::R](clr::R) reader structure"]
impl crate::Readable for CLR {}
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "Calendar Loading Register"]
pub mod clr;
#[doc = "Time Scale Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tssr](tssr) module"]
pub type TSSR = crate::Reg<u32, _TSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSR;
#[doc = "`read()` method returns [tssr::R](tssr::R) reader structure"]
impl crate::Readable for TSSR {}
#[doc = "`write(|w| ..)` method takes [tssr::W](tssr::W) writer structure"]
impl crate::Writable for TSSR {}
#[doc = "Time Scale Selection Register"]
pub mod tssr;
#[doc = "Day of the Week Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dwr](dwr) module"]
pub type DWR = crate::Reg<u32, _DWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DWR;
#[doc = "`read()` method returns [dwr::R](dwr::R) reader structure"]
impl crate::Readable for DWR {}
#[doc = "`write(|w| ..)` method takes [dwr::W](dwr::W) writer structure"]
impl crate::Writable for DWR {}
#[doc = "Day of the Week Register"]
pub mod dwr;
#[doc = "Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "Time Alarm Register"]
pub mod tar;
#[doc = "Calendar Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [car](car) module"]
pub type CAR = crate::Reg<u32, _CAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAR;
#[doc = "`read()` method returns [car::R](car::R) reader structure"]
impl crate::Readable for CAR {}
#[doc = "`write(|w| ..)` method takes [car::W](car::W) writer structure"]
impl crate::Writable for CAR {}
#[doc = "Calendar Alarm Register"]
pub mod car;
#[doc = "RTC Leap year Indicator Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lir](lir) module"]
pub type LIR = crate::Reg<u32, _LIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIR;
#[doc = "`read()` method returns [lir::R](lir::R) reader structure"]
impl crate::Readable for LIR {}
#[doc = "RTC Leap year Indicator Register"]
pub mod lir;
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rier](rier) module"]
pub type RIER = crate::Reg<u32, _RIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIER;
#[doc = "`read()` method returns [rier::R](rier::R) reader structure"]
impl crate::Readable for RIER {}
#[doc = "`write(|w| ..)` method takes [rier::W](rier::W) writer structure"]
impl crate::Writable for RIER {}
#[doc = "RTC Interrupt Enable Register"]
pub mod rier;
#[doc = "RTC Interrupt Indicator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riir](riir) module"]
pub type RIIR = crate::Reg<u32, _RIIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIIR;
#[doc = "`read()` method returns [riir::R](riir::R) reader structure"]
impl crate::Readable for RIIR {}
#[doc = "`write(|w| ..)` method takes [riir::W](riir::W) writer structure"]
impl crate::Writable for RIIR {}
#[doc = "RTC Interrupt Indicator Register"]
pub mod riir;
#[doc = "RTC Time Tick Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttr](ttr) module"]
pub type TTR = crate::Reg<u32, _TTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTR;
#[doc = "`read()` method returns [ttr::R](ttr::R) reader structure"]
impl crate::Readable for TTR {}
#[doc = "`write(|w| ..)` method takes [ttr::W](ttr::W) writer structure"]
impl crate::Writable for TTR {}
#[doc = "RTC Time Tick Register"]
pub mod ttr;
