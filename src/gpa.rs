#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Pin I/O Mode Control"]
    pub pmd: PMD,
    #[doc = "0x04 - GPIO Port Pin OFF Digital Enable"]
    pub offd: OFFD,
    #[doc = "0x08 - GPIO Port Data Output Value"]
    pub dout: DOUT,
    #[doc = "0x0c - GPIO Port Data Output Write Mask"]
    pub dmask: DMASK,
    #[doc = "0x10 - GPIO Port Pin Value"]
    pub pin: PIN,
    #[doc = "0x14 - GPIO Port De-bounce Enable"]
    pub dben: DBEN,
    #[doc = "0x18 - GPIO Port Interrupt Mode Control"]
    pub imd: IMD,
    #[doc = "0x1c - GPIO Port Interrupt Enable"]
    pub ien: IEN,
    #[doc = "0x20 - GPIO Port Interrupt Trigger Source Indicator"]
    pub isrc: ISRC,
}
#[doc = "GPIO Port Pin I/O Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd](pmd) module"]
pub type PMD = crate::Reg<u32, _PMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD;
#[doc = "`read()` method returns [pmd::R](pmd::R) reader structure"]
impl crate::Readable for PMD {}
#[doc = "`write(|w| ..)` method takes [pmd::W](pmd::W) writer structure"]
impl crate::Writable for PMD {}
#[doc = "GPIO Port Pin I/O Mode Control"]
pub mod pmd;
#[doc = "GPIO Port Pin OFF Digital Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offd](offd) module"]
pub type OFFD = crate::Reg<u32, _OFFD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFD;
#[doc = "`read()` method returns [offd::R](offd::R) reader structure"]
impl crate::Readable for OFFD {}
#[doc = "`write(|w| ..)` method takes [offd::W](offd::W) writer structure"]
impl crate::Writable for OFFD {}
#[doc = "GPIO Port Pin OFF Digital Enable"]
pub mod offd;
#[doc = "GPIO Port Data Output Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout](dout) module"]
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
#[doc = "`read()` method returns [dout::R](dout::R) reader structure"]
impl crate::Readable for DOUT {}
#[doc = "`write(|w| ..)` method takes [dout::W](dout::W) writer structure"]
impl crate::Writable for DOUT {}
#[doc = "GPIO Port Data Output Value"]
pub mod dout;
#[doc = "GPIO Port Data Output Write Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmask](dmask) module"]
pub type DMASK = crate::Reg<u32, _DMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASK;
#[doc = "`read()` method returns [dmask::R](dmask::R) reader structure"]
impl crate::Readable for DMASK {}
#[doc = "`write(|w| ..)` method takes [dmask::W](dmask::W) writer structure"]
impl crate::Writable for DMASK {}
#[doc = "GPIO Port Data Output Write Mask"]
pub mod dmask;
#[doc = "GPIO Port Pin Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "GPIO Port Pin Value"]
pub mod pin;
#[doc = "GPIO Port De-bounce Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dben](dben) module"]
pub type DBEN = crate::Reg<u32, _DBEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBEN;
#[doc = "`read()` method returns [dben::R](dben::R) reader structure"]
impl crate::Readable for DBEN {}
#[doc = "`write(|w| ..)` method takes [dben::W](dben::W) writer structure"]
impl crate::Writable for DBEN {}
#[doc = "GPIO Port De-bounce Enable"]
pub mod dben;
#[doc = "GPIO Port Interrupt Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imd](imd) module"]
pub type IMD = crate::Reg<u32, _IMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMD;
#[doc = "`read()` method returns [imd::R](imd::R) reader structure"]
impl crate::Readable for IMD {}
#[doc = "`write(|w| ..)` method takes [imd::W](imd::W) writer structure"]
impl crate::Writable for IMD {}
#[doc = "GPIO Port Interrupt Mode Control"]
pub mod imd;
#[doc = "GPIO Port Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "GPIO Port Interrupt Enable"]
pub mod ien;
#[doc = "GPIO Port Interrupt Trigger Source Indicator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrc](isrc) module"]
pub type ISRC = crate::Reg<u32, _ISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISRC;
#[doc = "`read()` method returns [isrc::R](isrc::R) reader structure"]
impl crate::Readable for ISRC {}
#[doc = "`write(|w| ..)` method takes [isrc::W](isrc::W) writer structure"]
impl crate::Writable for ISRC {}
#[doc = "GPIO Port Interrupt Trigger Source Indicator"]
pub mod isrc;
