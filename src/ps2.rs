#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PS2 Control Register"]
    pub ps2con: PS2CON,
    #[doc = "0x04 - PS2 Transmit DATA Register 0"]
    pub ps2txdata0: PS2TXDATA0,
    #[doc = "0x08 - PS2 Transmit DATA Register 1"]
    pub ps2txdata1: PS2TXDATA1,
    #[doc = "0x0c - PS2 Transmit DATA Register 2"]
    pub ps2txdata2: PS2TXDATA2,
    #[doc = "0x10 - PS2 Transmit DATA Register 3"]
    pub ps2txdata3: PS2TXDATA3,
    #[doc = "0x14 - PS2 Receive DATA Register"]
    pub ps2rxdata: PS2RXDATA,
    #[doc = "0x18 - PS2 Status Register"]
    pub ps2status: PS2STATUS,
    #[doc = "0x1c - PS2 Interrupt Identification Register"]
    pub ps2intid: PS2INTID,
}
#[doc = "PS2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2con](ps2con) module"]
pub type PS2CON = crate::Reg<u32, _PS2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2CON;
#[doc = "`read()` method returns [ps2con::R](ps2con::R) reader structure"]
impl crate::Readable for PS2CON {}
#[doc = "`write(|w| ..)` method takes [ps2con::W](ps2con::W) writer structure"]
impl crate::Writable for PS2CON {}
#[doc = "PS2 Control Register"]
pub mod ps2con;
#[doc = "PS2 Transmit DATA Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2txdata0](ps2txdata0) module"]
pub type PS2TXDATA0 = crate::Reg<u32, _PS2TXDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2TXDATA0;
#[doc = "`read()` method returns [ps2txdata0::R](ps2txdata0::R) reader structure"]
impl crate::Readable for PS2TXDATA0 {}
#[doc = "`write(|w| ..)` method takes [ps2txdata0::W](ps2txdata0::W) writer structure"]
impl crate::Writable for PS2TXDATA0 {}
#[doc = "PS2 Transmit DATA Register 0"]
pub mod ps2txdata0;
#[doc = "PS2 Transmit DATA Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2txdata1](ps2txdata1) module"]
pub type PS2TXDATA1 = crate::Reg<u32, _PS2TXDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2TXDATA1;
#[doc = "`read()` method returns [ps2txdata1::R](ps2txdata1::R) reader structure"]
impl crate::Readable for PS2TXDATA1 {}
#[doc = "`write(|w| ..)` method takes [ps2txdata1::W](ps2txdata1::W) writer structure"]
impl crate::Writable for PS2TXDATA1 {}
#[doc = "PS2 Transmit DATA Register 1"]
pub mod ps2txdata1;
#[doc = "PS2 Transmit DATA Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2txdata2](ps2txdata2) module"]
pub type PS2TXDATA2 = crate::Reg<u32, _PS2TXDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2TXDATA2;
#[doc = "`read()` method returns [ps2txdata2::R](ps2txdata2::R) reader structure"]
impl crate::Readable for PS2TXDATA2 {}
#[doc = "`write(|w| ..)` method takes [ps2txdata2::W](ps2txdata2::W) writer structure"]
impl crate::Writable for PS2TXDATA2 {}
#[doc = "PS2 Transmit DATA Register 2"]
pub mod ps2txdata2;
#[doc = "PS2 Transmit DATA Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2txdata3](ps2txdata3) module"]
pub type PS2TXDATA3 = crate::Reg<u32, _PS2TXDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2TXDATA3;
#[doc = "`read()` method returns [ps2txdata3::R](ps2txdata3::R) reader structure"]
impl crate::Readable for PS2TXDATA3 {}
#[doc = "`write(|w| ..)` method takes [ps2txdata3::W](ps2txdata3::W) writer structure"]
impl crate::Writable for PS2TXDATA3 {}
#[doc = "PS2 Transmit DATA Register 3"]
pub mod ps2txdata3;
#[doc = "PS2 Receive DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2rxdata](ps2rxdata) module"]
pub type PS2RXDATA = crate::Reg<u32, _PS2RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2RXDATA;
#[doc = "`read()` method returns [ps2rxdata::R](ps2rxdata::R) reader structure"]
impl crate::Readable for PS2RXDATA {}
#[doc = "`write(|w| ..)` method takes [ps2rxdata::W](ps2rxdata::W) writer structure"]
impl crate::Writable for PS2RXDATA {}
#[doc = "PS2 Receive DATA Register"]
pub mod ps2rxdata;
#[doc = "PS2 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2status](ps2status) module"]
pub type PS2STATUS = crate::Reg<u32, _PS2STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2STATUS;
#[doc = "`read()` method returns [ps2status::R](ps2status::R) reader structure"]
impl crate::Readable for PS2STATUS {}
#[doc = "`write(|w| ..)` method takes [ps2status::W](ps2status::W) writer structure"]
impl crate::Writable for PS2STATUS {}
#[doc = "PS2 Status Register"]
pub mod ps2status;
#[doc = "PS2 Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2intid](ps2intid) module"]
pub type PS2INTID = crate::Reg<u32, _PS2INTID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS2INTID;
#[doc = "`read()` method returns [ps2intid::R](ps2intid::R) reader structure"]
impl crate::Readable for PS2INTID {}
#[doc = "`write(|w| ..)` method takes [ps2intid::W](ps2intid::W) writer structure"]
impl crate::Writable for PS2INTID {}
#[doc = "PS2 Interrupt Identification Register"]
pub mod ps2intid;
