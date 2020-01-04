#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Power Down Control Register"]
    pub pwrcon: PWRCON,
    #[doc = "0x04 - AHB Devices Clock Enable Control Register"]
    pub ahbclk: AHBCLK,
    #[doc = "0x08 - APB Devices Clock Enable Control Register"]
    pub apbclk: APBCLK,
    #[doc = "0x0c - Clock status monitor Register"]
    pub clkstatus: CLKSTATUS,
    #[doc = "0x10 - Clock Source Select Control Register 0"]
    pub clksel0: CLKSEL0,
    #[doc = "0x14 - Clock Source Select Control Register 1"]
    pub clksel1: CLKSEL1,
    #[doc = "0x18 - Clock Divider Number Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x1c - Clock Source Select Control Register 2"]
    pub clksel2: CLKSEL2,
    #[doc = "0x20 - PLL Control Register"]
    pub pllcon: PLLCON,
    #[doc = "0x24 - Frequency Divider Control Register"]
    pub frqdiv: FRQDIV,
}
#[doc = "System Power Down Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcon](pwrcon) module"]
pub type PWRCON = crate::Reg<u32, _PWRCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCON;
#[doc = "`read()` method returns [pwrcon::R](pwrcon::R) reader structure"]
impl crate::Readable for PWRCON {}
#[doc = "`write(|w| ..)` method takes [pwrcon::W](pwrcon::W) writer structure"]
impl crate::Writable for PWRCON {}
#[doc = "System Power Down Control Register"]
pub mod pwrcon;
#[doc = "AHB Devices Clock Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclk](ahbclk) module"]
pub type AHBCLK = crate::Reg<u32, _AHBCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLK;
#[doc = "`read()` method returns [ahbclk::R](ahbclk::R) reader structure"]
impl crate::Readable for AHBCLK {}
#[doc = "`write(|w| ..)` method takes [ahbclk::W](ahbclk::W) writer structure"]
impl crate::Writable for AHBCLK {}
#[doc = "AHB Devices Clock Enable Control Register"]
pub mod ahbclk;
#[doc = "APB Devices Clock Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbclk](apbclk) module"]
pub type APBCLK = crate::Reg<u32, _APBCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCLK;
#[doc = "`read()` method returns [apbclk::R](apbclk::R) reader structure"]
impl crate::Readable for APBCLK {}
#[doc = "`write(|w| ..)` method takes [apbclk::W](apbclk::W) writer structure"]
impl crate::Writable for APBCLK {}
#[doc = "APB Devices Clock Enable Control Register"]
pub mod apbclk;
#[doc = "Clock status monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstatus](clkstatus) module"]
pub type CLKSTATUS = crate::Reg<u32, _CLKSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTATUS;
#[doc = "`read()` method returns [clkstatus::R](clkstatus::R) reader structure"]
impl crate::Readable for CLKSTATUS {}
#[doc = "`write(|w| ..)` method takes [clkstatus::W](clkstatus::W) writer structure"]
impl crate::Writable for CLKSTATUS {}
#[doc = "Clock status monitor Register"]
pub mod clkstatus;
#[doc = "Clock Source Select Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel0](clksel0) module"]
pub type CLKSEL0 = crate::Reg<u32, _CLKSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSEL0;
#[doc = "`read()` method returns [clksel0::R](clksel0::R) reader structure"]
impl crate::Readable for CLKSEL0 {}
#[doc = "`write(|w| ..)` method takes [clksel0::W](clksel0::W) writer structure"]
impl crate::Writable for CLKSEL0 {}
#[doc = "Clock Source Select Control Register 0"]
pub mod clksel0;
#[doc = "Clock Source Select Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel1](clksel1) module"]
pub type CLKSEL1 = crate::Reg<u32, _CLKSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSEL1;
#[doc = "`read()` method returns [clksel1::R](clksel1::R) reader structure"]
impl crate::Readable for CLKSEL1 {}
#[doc = "`write(|w| ..)` method takes [clksel1::W](clksel1::W) writer structure"]
impl crate::Writable for CLKSEL1 {}
#[doc = "Clock Source Select Control Register 1"]
pub mod clksel1;
#[doc = "Clock Divider Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock Divider Number Register"]
pub mod clkdiv;
#[doc = "Clock Source Select Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel2](clksel2) module"]
pub type CLKSEL2 = crate::Reg<u32, _CLKSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSEL2;
#[doc = "`read()` method returns [clksel2::R](clksel2::R) reader structure"]
impl crate::Readable for CLKSEL2 {}
#[doc = "`write(|w| ..)` method takes [clksel2::W](clksel2::W) writer structure"]
impl crate::Writable for CLKSEL2 {}
#[doc = "Clock Source Select Control Register 2"]
pub mod clksel2;
#[doc = "PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon](pllcon) module"]
pub type PLLCON = crate::Reg<u32, _PLLCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCON;
#[doc = "`read()` method returns [pllcon::R](pllcon::R) reader structure"]
impl crate::Readable for PLLCON {}
#[doc = "`write(|w| ..)` method takes [pllcon::W](pllcon::W) writer structure"]
impl crate::Writable for PLLCON {}
#[doc = "PLL Control Register"]
pub mod pllcon;
#[doc = "Frequency Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frqdiv](frqdiv) module"]
pub type FRQDIV = crate::Reg<u32, _FRQDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRQDIV;
#[doc = "`read()` method returns [frqdiv::R](frqdiv::R) reader structure"]
impl crate::Readable for FRQDIV {}
#[doc = "`write(|w| ..)` method takes [frqdiv::W](frqdiv::W) writer structure"]
impl crate::Writable for FRQDIV {}
#[doc = "Frequency Divider Control Register"]
pub mod frqdiv;
