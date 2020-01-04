#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Data Register 0"]
    pub addr0: ADDR0,
    #[doc = "0x04 - A/D Data Register 1"]
    pub addr1: ADDR1,
    #[doc = "0x08 - A/D Data Register 2"]
    pub addr2: ADDR2,
    #[doc = "0x0c - A/D Data Register 3"]
    pub addr3: ADDR3,
    #[doc = "0x10 - A/D Data Register 4"]
    pub addr4: ADDR4,
    #[doc = "0x14 - A/D Data Register 5"]
    pub addr5: ADDR5,
    #[doc = "0x18 - A/D Data Register 6"]
    pub addr6: ADDR6,
    #[doc = "0x1c - A/D Data Register 7"]
    pub addr7: ADDR7,
    #[doc = "0x20 - A/D Control Register"]
    pub adcr: ADCR,
    #[doc = "0x24 - A/D Channel Enable Register"]
    pub adcher: ADCHER,
    #[doc = "0x28 - A/D Compare Register 0"]
    pub adcmpr0: ADCMPR0,
    #[doc = "0x2c - A/D Compare Register 1"]
    pub adcmpr1: ADCMPR1,
    #[doc = "0x30 - A/D Status Register"]
    pub adsr: ADSR,
    #[doc = "0x34 - A/D Calibration Register"]
    pub adcalr: ADCALR,
    _reserved14: [u8; 8usize],
    #[doc = "0x40 - New description for register"]
    pub adpdma: ADPDMA,
}
#[doc = "A/D Data Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](addr0) module"]
pub type ADDR0 = crate::Reg<u32, _ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0;
#[doc = "`read()` method returns [addr0::R](addr0::R) reader structure"]
impl crate::Readable for ADDR0 {}
#[doc = "A/D Data Register 0"]
pub mod addr0;
#[doc = "A/D Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](addr1) module"]
pub type ADDR1 = crate::Reg<u32, _ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1;
#[doc = "`read()` method returns [addr1::R](addr1::R) reader structure"]
impl crate::Readable for ADDR1 {}
#[doc = "A/D Data Register 1"]
pub mod addr1;
#[doc = "A/D Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr2](addr2) module"]
pub type ADDR2 = crate::Reg<u32, _ADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR2;
#[doc = "`read()` method returns [addr2::R](addr2::R) reader structure"]
impl crate::Readable for ADDR2 {}
#[doc = "A/D Data Register 2"]
pub mod addr2;
#[doc = "A/D Data Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr3](addr3) module"]
pub type ADDR3 = crate::Reg<u32, _ADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR3;
#[doc = "`read()` method returns [addr3::R](addr3::R) reader structure"]
impl crate::Readable for ADDR3 {}
#[doc = "A/D Data Register 3"]
pub mod addr3;
#[doc = "A/D Data Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr4](addr4) module"]
pub type ADDR4 = crate::Reg<u32, _ADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR4;
#[doc = "`read()` method returns [addr4::R](addr4::R) reader structure"]
impl crate::Readable for ADDR4 {}
#[doc = "A/D Data Register 4"]
pub mod addr4;
#[doc = "A/D Data Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr5](addr5) module"]
pub type ADDR5 = crate::Reg<u32, _ADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR5;
#[doc = "`read()` method returns [addr5::R](addr5::R) reader structure"]
impl crate::Readable for ADDR5 {}
#[doc = "A/D Data Register 5"]
pub mod addr5;
#[doc = "A/D Data Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr6](addr6) module"]
pub type ADDR6 = crate::Reg<u32, _ADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR6;
#[doc = "`read()` method returns [addr6::R](addr6::R) reader structure"]
impl crate::Readable for ADDR6 {}
#[doc = "A/D Data Register 6"]
pub mod addr6;
#[doc = "A/D Data Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr7](addr7) module"]
pub type ADDR7 = crate::Reg<u32, _ADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR7;
#[doc = "`read()` method returns [addr7::R](addr7::R) reader structure"]
impl crate::Readable for ADDR7 {}
#[doc = "A/D Data Register 7"]
pub mod addr7;
#[doc = "A/D Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcr](adcr) module"]
pub type ADCR = crate::Reg<u32, _ADCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCR;
#[doc = "`read()` method returns [adcr::R](adcr::R) reader structure"]
impl crate::Readable for ADCR {}
#[doc = "`write(|w| ..)` method takes [adcr::W](adcr::W) writer structure"]
impl crate::Writable for ADCR {}
#[doc = "A/D Control Register"]
pub mod adcr;
#[doc = "A/D Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcher](adcher) module"]
pub type ADCHER = crate::Reg<u32, _ADCHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCHER;
#[doc = "`read()` method returns [adcher::R](adcher::R) reader structure"]
impl crate::Readable for ADCHER {}
#[doc = "`write(|w| ..)` method takes [adcher::W](adcher::W) writer structure"]
impl crate::Writable for ADCHER {}
#[doc = "A/D Channel Enable Register"]
pub mod adcher;
#[doc = "A/D Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpr0](adcmpr0) module"]
pub type ADCMPR0 = crate::Reg<u32, _ADCMPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCMPR0;
#[doc = "`read()` method returns [adcmpr0::R](adcmpr0::R) reader structure"]
impl crate::Readable for ADCMPR0 {}
#[doc = "`write(|w| ..)` method takes [adcmpr0::W](adcmpr0::W) writer structure"]
impl crate::Writable for ADCMPR0 {}
#[doc = "A/D Compare Register 0"]
pub mod adcmpr0;
#[doc = "A/D Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpr1](adcmpr1) module"]
pub type ADCMPR1 = crate::Reg<u32, _ADCMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCMPR1;
#[doc = "`read()` method returns [adcmpr1::R](adcmpr1::R) reader structure"]
impl crate::Readable for ADCMPR1 {}
#[doc = "`write(|w| ..)` method takes [adcmpr1::W](adcmpr1::W) writer structure"]
impl crate::Writable for ADCMPR1 {}
#[doc = "A/D Compare Register 1"]
pub mod adcmpr1;
#[doc = "A/D Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsr](adsr) module"]
pub type ADSR = crate::Reg<u32, _ADSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADSR;
#[doc = "`read()` method returns [adsr::R](adsr::R) reader structure"]
impl crate::Readable for ADSR {}
#[doc = "`write(|w| ..)` method takes [adsr::W](adsr::W) writer structure"]
impl crate::Writable for ADSR {}
#[doc = "A/D Status Register"]
pub mod adsr;
#[doc = "A/D Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalr](adcalr) module"]
pub type ADCALR = crate::Reg<u32, _ADCALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCALR;
#[doc = "`read()` method returns [adcalr::R](adcalr::R) reader structure"]
impl crate::Readable for ADCALR {}
#[doc = "`write(|w| ..)` method takes [adcalr::W](adcalr::W) writer structure"]
impl crate::Writable for ADCALR {}
#[doc = "A/D Calibration Register"]
pub mod adcalr;
#[doc = "New description for register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpdma](adpdma) module"]
pub type ADPDMA = crate::Reg<u32, _ADPDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADPDMA;
#[doc = "`read()` method returns [adpdma::R](adpdma::R) reader structure"]
impl crate::Readable for ADPDMA {}
#[doc = "New description for register"]
pub mod adpdma;
