#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISP Control Register"]
    pub ispcon: ISPCON,
    #[doc = "0x04 - ISP Address Register"]
    pub ispadr: ISPADR,
    #[doc = "0x08 - ISP Data Register"]
    pub ispdat: ISPDAT,
    #[doc = "0x0c - ISP Command Register"]
    pub ispcmd: ISPCMD,
    #[doc = "0x10 - ISP Trigger Control Register"]
    pub isptrg: ISPTRG,
    #[doc = "0x14 - Data Flash Base Address Register"]
    pub dfbadr: DFBADR,
    #[doc = "0x18 - Flash Access Time Control Register"]
    pub fatcon: FATCON,
}
#[doc = "ISP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispcon](ispcon) module"]
pub type ISPCON = crate::Reg<u32, _ISPCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPCON;
#[doc = "`read()` method returns [ispcon::R](ispcon::R) reader structure"]
impl crate::Readable for ISPCON {}
#[doc = "`write(|w| ..)` method takes [ispcon::W](ispcon::W) writer structure"]
impl crate::Writable for ISPCON {}
#[doc = "ISP Control Register"]
pub mod ispcon;
#[doc = "ISP Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispadr](ispadr) module"]
pub type ISPADR = crate::Reg<u32, _ISPADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPADR;
#[doc = "`read()` method returns [ispadr::R](ispadr::R) reader structure"]
impl crate::Readable for ISPADR {}
#[doc = "`write(|w| ..)` method takes [ispadr::W](ispadr::W) writer structure"]
impl crate::Writable for ISPADR {}
#[doc = "ISP Address Register"]
pub mod ispadr;
#[doc = "ISP Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispdat](ispdat) module"]
pub type ISPDAT = crate::Reg<u32, _ISPDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPDAT;
#[doc = "`read()` method returns [ispdat::R](ispdat::R) reader structure"]
impl crate::Readable for ISPDAT {}
#[doc = "`write(|w| ..)` method takes [ispdat::W](ispdat::W) writer structure"]
impl crate::Writable for ISPDAT {}
#[doc = "ISP Data Register"]
pub mod ispdat;
#[doc = "ISP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispcmd](ispcmd) module"]
pub type ISPCMD = crate::Reg<u32, _ISPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPCMD;
#[doc = "`read()` method returns [ispcmd::R](ispcmd::R) reader structure"]
impl crate::Readable for ISPCMD {}
#[doc = "`write(|w| ..)` method takes [ispcmd::W](ispcmd::W) writer structure"]
impl crate::Writable for ISPCMD {}
#[doc = "ISP Command Register"]
pub mod ispcmd;
#[doc = "ISP Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isptrg](isptrg) module"]
pub type ISPTRG = crate::Reg<u32, _ISPTRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPTRG;
#[doc = "`read()` method returns [isptrg::R](isptrg::R) reader structure"]
impl crate::Readable for ISPTRG {}
#[doc = "`write(|w| ..)` method takes [isptrg::W](isptrg::W) writer structure"]
impl crate::Writable for ISPTRG {}
#[doc = "ISP Trigger Control Register"]
pub mod isptrg;
#[doc = "Data Flash Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfbadr](dfbadr) module"]
pub type DFBADR = crate::Reg<u32, _DFBADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFBADR;
#[doc = "`read()` method returns [dfbadr::R](dfbadr::R) reader structure"]
impl crate::Readable for DFBADR {}
#[doc = "Data Flash Base Address Register"]
pub mod dfbadr;
#[doc = "Flash Access Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fatcon](fatcon) module"]
pub type FATCON = crate::Reg<u32, _FATCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FATCON;
#[doc = "`read()` method returns [fatcon::R](fatcon::R) reader structure"]
impl crate::Readable for FATCON {}
#[doc = "`write(|w| ..)` method takes [fatcon::W](fatcon::W) writer structure"]
impl crate::Writable for FATCON {}
#[doc = "Flash Access Time Control Register"]
pub mod fatcon;
