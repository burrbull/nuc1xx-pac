#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator0 Control Register"]
    pub cmp0cr: CMP0CR,
    #[doc = "0x04 - Comparator1 Control Register"]
    pub cmp1cr: CMP1CR,
    #[doc = "0x08 - Comparator Channel Selection Enable Register"]
    pub cmpsr: CMPSR,
}
#[doc = "Comparator0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp0cr](cmp0cr) module"]
pub type CMP0CR = crate::Reg<u32, _CMP0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP0CR;
#[doc = "`read()` method returns [cmp0cr::R](cmp0cr::R) reader structure"]
impl crate::Readable for CMP0CR {}
#[doc = "`write(|w| ..)` method takes [cmp0cr::W](cmp0cr::W) writer structure"]
impl crate::Writable for CMP0CR {}
#[doc = "Comparator0 Control Register"]
pub mod cmp0cr;
#[doc = "Comparator1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1cr](cmp1cr) module"]
pub type CMP1CR = crate::Reg<u32, _CMP1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CR;
#[doc = "`read()` method returns [cmp1cr::R](cmp1cr::R) reader structure"]
impl crate::Readable for CMP1CR {}
#[doc = "`write(|w| ..)` method takes [cmp1cr::W](cmp1cr::W) writer structure"]
impl crate::Writable for CMP1CR {}
#[doc = "Comparator1 Control Register"]
pub mod cmp1cr;
#[doc = "Comparator Channel Selection Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpsr](cmpsr) module"]
pub type CMPSR = crate::Reg<u32, _CMPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPSR;
#[doc = "`read()` method returns [cmpsr::R](cmpsr::R) reader structure"]
impl crate::Readable for CMPSR {}
#[doc = "`write(|w| ..)` method takes [cmpsr::W](cmpsr::W) writer structure"]
impl crate::Writable for CMPSR {}
#[doc = "Comparator Channel Selection Enable Register"]
pub mod cmpsr;
