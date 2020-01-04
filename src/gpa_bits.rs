#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Pin I/O Bit Output Control"]
    pub dout0: DOUT0,
    #[doc = "0x04 - GPIO Port Pin I/O Bit Output Control"]
    pub dout1: DOUT1,
    #[doc = "0x08 - GPIO Port Pin I/O Bit Output Control"]
    pub dout2: DOUT2,
    #[doc = "0x0c - GPIO Port Pin I/O Bit Output Control"]
    pub dout3: DOUT3,
    #[doc = "0x10 - GPIO Port Pin I/O Bit Output Control"]
    pub dout4: DOUT4,
    #[doc = "0x14 - GPIO Port Pin I/O Bit Output Control"]
    pub dout5: DOUT5,
    #[doc = "0x18 - GPIO Port Pin I/O Bit Output Control"]
    pub dout6: DOUT6,
    #[doc = "0x1c - GPIO Port Pin I/O Bit Output Control"]
    pub dout7: DOUT7,
    #[doc = "0x20 - GPIO Port Pin I/O Bit Output Control"]
    pub dout8: DOUT8,
    #[doc = "0x24 - GPIO Port Pin I/O Bit Output Control"]
    pub dout9: DOUT9,
    #[doc = "0x28 - GPIO Port Pin I/O Bit Output Control"]
    pub dout10: DOUT10,
    #[doc = "0x2c - GPIO Port Pin I/O Bit Output Control"]
    pub dout11: DOUT11,
    #[doc = "0x30 - GPIO Port Pin I/O Bit Output Control"]
    pub dout12: DOUT12,
    #[doc = "0x34 - GPIO Port Pin I/O Bit Output Control"]
    pub dout13: DOUT13,
    #[doc = "0x38 - GPIO Port Pin I/O Bit Output Control"]
    pub dout14: DOUT14,
    #[doc = "0x3c - GPIO Port Pin I/O Bit Output Control"]
    pub dout15: DOUT15,
}
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout0](dout0) module"]
pub type DOUT0 = crate::Reg<u32, _DOUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT0;
#[doc = "`read()` method returns [dout0::R](dout0::R) reader structure"]
impl crate::Readable for DOUT0 {}
#[doc = "`write(|w| ..)` method takes [dout0::W](dout0::W) writer structure"]
impl crate::Writable for DOUT0 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout0;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout1](dout1) module"]
pub type DOUT1 = crate::Reg<u32, _DOUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT1;
#[doc = "`read()` method returns [dout1::R](dout1::R) reader structure"]
impl crate::Readable for DOUT1 {}
#[doc = "`write(|w| ..)` method takes [dout1::W](dout1::W) writer structure"]
impl crate::Writable for DOUT1 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout1;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout2](dout2) module"]
pub type DOUT2 = crate::Reg<u32, _DOUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT2;
#[doc = "`read()` method returns [dout2::R](dout2::R) reader structure"]
impl crate::Readable for DOUT2 {}
#[doc = "`write(|w| ..)` method takes [dout2::W](dout2::W) writer structure"]
impl crate::Writable for DOUT2 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout2;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout3](dout3) module"]
pub type DOUT3 = crate::Reg<u32, _DOUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT3;
#[doc = "`read()` method returns [dout3::R](dout3::R) reader structure"]
impl crate::Readable for DOUT3 {}
#[doc = "`write(|w| ..)` method takes [dout3::W](dout3::W) writer structure"]
impl crate::Writable for DOUT3 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout3;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout4](dout4) module"]
pub type DOUT4 = crate::Reg<u32, _DOUT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT4;
#[doc = "`read()` method returns [dout4::R](dout4::R) reader structure"]
impl crate::Readable for DOUT4 {}
#[doc = "`write(|w| ..)` method takes [dout4::W](dout4::W) writer structure"]
impl crate::Writable for DOUT4 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout4;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout5](dout5) module"]
pub type DOUT5 = crate::Reg<u32, _DOUT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT5;
#[doc = "`read()` method returns [dout5::R](dout5::R) reader structure"]
impl crate::Readable for DOUT5 {}
#[doc = "`write(|w| ..)` method takes [dout5::W](dout5::W) writer structure"]
impl crate::Writable for DOUT5 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout5;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout6](dout6) module"]
pub type DOUT6 = crate::Reg<u32, _DOUT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT6;
#[doc = "`read()` method returns [dout6::R](dout6::R) reader structure"]
impl crate::Readable for DOUT6 {}
#[doc = "`write(|w| ..)` method takes [dout6::W](dout6::W) writer structure"]
impl crate::Writable for DOUT6 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout6;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout7](dout7) module"]
pub type DOUT7 = crate::Reg<u32, _DOUT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT7;
#[doc = "`read()` method returns [dout7::R](dout7::R) reader structure"]
impl crate::Readable for DOUT7 {}
#[doc = "`write(|w| ..)` method takes [dout7::W](dout7::W) writer structure"]
impl crate::Writable for DOUT7 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout7;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout8](dout8) module"]
pub type DOUT8 = crate::Reg<u32, _DOUT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT8;
#[doc = "`read()` method returns [dout8::R](dout8::R) reader structure"]
impl crate::Readable for DOUT8 {}
#[doc = "`write(|w| ..)` method takes [dout8::W](dout8::W) writer structure"]
impl crate::Writable for DOUT8 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout8;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout9](dout9) module"]
pub type DOUT9 = crate::Reg<u32, _DOUT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT9;
#[doc = "`read()` method returns [dout9::R](dout9::R) reader structure"]
impl crate::Readable for DOUT9 {}
#[doc = "`write(|w| ..)` method takes [dout9::W](dout9::W) writer structure"]
impl crate::Writable for DOUT9 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout9;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout10](dout10) module"]
pub type DOUT10 = crate::Reg<u32, _DOUT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT10;
#[doc = "`read()` method returns [dout10::R](dout10::R) reader structure"]
impl crate::Readable for DOUT10 {}
#[doc = "`write(|w| ..)` method takes [dout10::W](dout10::W) writer structure"]
impl crate::Writable for DOUT10 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout10;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout11](dout11) module"]
pub type DOUT11 = crate::Reg<u32, _DOUT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT11;
#[doc = "`read()` method returns [dout11::R](dout11::R) reader structure"]
impl crate::Readable for DOUT11 {}
#[doc = "`write(|w| ..)` method takes [dout11::W](dout11::W) writer structure"]
impl crate::Writable for DOUT11 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout11;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout12](dout12) module"]
pub type DOUT12 = crate::Reg<u32, _DOUT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT12;
#[doc = "`read()` method returns [dout12::R](dout12::R) reader structure"]
impl crate::Readable for DOUT12 {}
#[doc = "`write(|w| ..)` method takes [dout12::W](dout12::W) writer structure"]
impl crate::Writable for DOUT12 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout12;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout13](dout13) module"]
pub type DOUT13 = crate::Reg<u32, _DOUT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT13;
#[doc = "`read()` method returns [dout13::R](dout13::R) reader structure"]
impl crate::Readable for DOUT13 {}
#[doc = "`write(|w| ..)` method takes [dout13::W](dout13::W) writer structure"]
impl crate::Writable for DOUT13 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout13;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout14](dout14) module"]
pub type DOUT14 = crate::Reg<u32, _DOUT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT14;
#[doc = "`read()` method returns [dout14::R](dout14::R) reader structure"]
impl crate::Readable for DOUT14 {}
#[doc = "`write(|w| ..)` method takes [dout14::W](dout14::W) writer structure"]
impl crate::Writable for DOUT14 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout14;
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout15](dout15) module"]
pub type DOUT15 = crate::Reg<u32, _DOUT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT15;
#[doc = "`read()` method returns [dout15::R](dout15::R) reader structure"]
impl crate::Readable for DOUT15 {}
#[doc = "`write(|w| ..)` method takes [dout15::W](dout15::W) writer structure"]
impl crate::Writable for DOUT15 {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout15;
