#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Register"]
    pub i2con: I2CON,
    #[doc = "0x04 - I2C Slave Address Register0"]
    pub i2caddr0: I2CADDR0,
    #[doc = "0x08 - I2C Data Register"]
    pub i2cdat: I2CDAT,
    #[doc = "0x0c - I2C Status Register"]
    pub i2cstatus: I2CSTATUS,
    #[doc = "0x10 - I2C Clock Divided Register"]
    pub i2clk: I2CLK,
    #[doc = "0x14 - I2C Time-Out Control Register"]
    pub i2ctoc: I2CTOC,
    #[doc = "0x18 - I2C Slave address Register1"]
    pub i2caddr1: I2CADDR1,
    #[doc = "0x1c - I2C Slave address Register2"]
    pub i2caddr2: I2CADDR2,
    #[doc = "0x20 - I2C Slave address Register3"]
    pub i2caddr3: I2CADDR3,
    #[doc = "0x24 - I2C Slave Address Mask Register0"]
    pub i2cadm0: I2CADM0,
    #[doc = "0x28 - I2C Slave Address Mask Register1"]
    pub i2cadm1: I2CADM1,
    #[doc = "0x2c - I2C Slave Address Mask Register2"]
    pub i2cadm2: I2CADM2,
    #[doc = "0x30 - I2C Slave Address Mask Register3"]
    pub i2cadm3: I2CADM3,
}
#[doc = "I2C Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2con](i2con) module"]
pub type I2CON = crate::Reg<u32, _I2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CON;
#[doc = "`read()` method returns [i2con::R](i2con::R) reader structure"]
impl crate::Readable for I2CON {}
#[doc = "`write(|w| ..)` method takes [i2con::W](i2con::W) writer structure"]
impl crate::Writable for I2CON {}
#[doc = "I2C Control Register"]
pub mod i2con;
#[doc = "I2C Slave Address Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2caddr0](i2caddr0) module"]
pub type I2CADDR0 = crate::Reg<u32, _I2CADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADDR0;
#[doc = "`read()` method returns [i2caddr0::R](i2caddr0::R) reader structure"]
impl crate::Readable for I2CADDR0 {}
#[doc = "`write(|w| ..)` method takes [i2caddr0::W](i2caddr0::W) writer structure"]
impl crate::Writable for I2CADDR0 {}
#[doc = "I2C Slave Address Register0"]
pub mod i2caddr0;
#[doc = "I2C Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cdat](i2cdat) module"]
pub type I2CDAT = crate::Reg<u32, _I2CDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CDAT;
#[doc = "`read()` method returns [i2cdat::R](i2cdat::R) reader structure"]
impl crate::Readable for I2CDAT {}
#[doc = "`write(|w| ..)` method takes [i2cdat::W](i2cdat::W) writer structure"]
impl crate::Writable for I2CDAT {}
#[doc = "I2C Data Register"]
pub mod i2cdat;
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cstatus](i2cstatus) module"]
pub type I2CSTATUS = crate::Reg<u32, _I2CSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSTATUS;
#[doc = "`read()` method returns [i2cstatus::R](i2cstatus::R) reader structure"]
impl crate::Readable for I2CSTATUS {}
#[doc = "I2C Status Register"]
pub mod i2cstatus;
#[doc = "I2C Clock Divided Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2clk](i2clk) module"]
pub type I2CLK = crate::Reg<u32, _I2CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CLK;
#[doc = "`read()` method returns [i2clk::R](i2clk::R) reader structure"]
impl crate::Readable for I2CLK {}
#[doc = "`write(|w| ..)` method takes [i2clk::W](i2clk::W) writer structure"]
impl crate::Writable for I2CLK {}
#[doc = "I2C Clock Divided Register"]
pub mod i2clk;
#[doc = "I2C Time-Out Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2ctoc](i2ctoc) module"]
pub type I2CTOC = crate::Reg<u32, _I2CTOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CTOC;
#[doc = "`read()` method returns [i2ctoc::R](i2ctoc::R) reader structure"]
impl crate::Readable for I2CTOC {}
#[doc = "`write(|w| ..)` method takes [i2ctoc::W](i2ctoc::W) writer structure"]
impl crate::Writable for I2CTOC {}
#[doc = "I2C Time-Out Control Register"]
pub mod i2ctoc;
#[doc = "I2C Slave address Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2caddr1](i2caddr1) module"]
pub type I2CADDR1 = crate::Reg<u32, _I2CADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADDR1;
#[doc = "`read()` method returns [i2caddr1::R](i2caddr1::R) reader structure"]
impl crate::Readable for I2CADDR1 {}
#[doc = "`write(|w| ..)` method takes [i2caddr1::W](i2caddr1::W) writer structure"]
impl crate::Writable for I2CADDR1 {}
#[doc = "I2C Slave address Register1"]
pub mod i2caddr1;
#[doc = "I2C Slave address Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2caddr2](i2caddr2) module"]
pub type I2CADDR2 = crate::Reg<u32, _I2CADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADDR2;
#[doc = "`read()` method returns [i2caddr2::R](i2caddr2::R) reader structure"]
impl crate::Readable for I2CADDR2 {}
#[doc = "`write(|w| ..)` method takes [i2caddr2::W](i2caddr2::W) writer structure"]
impl crate::Writable for I2CADDR2 {}
#[doc = "I2C Slave address Register2"]
pub mod i2caddr2;
#[doc = "I2C Slave address Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2caddr3](i2caddr3) module"]
pub type I2CADDR3 = crate::Reg<u32, _I2CADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADDR3;
#[doc = "`read()` method returns [i2caddr3::R](i2caddr3::R) reader structure"]
impl crate::Readable for I2CADDR3 {}
#[doc = "`write(|w| ..)` method takes [i2caddr3::W](i2caddr3::W) writer structure"]
impl crate::Writable for I2CADDR3 {}
#[doc = "I2C Slave address Register3"]
pub mod i2caddr3;
#[doc = "I2C Slave Address Mask Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cadm0](i2cadm0) module"]
pub type I2CADM0 = crate::Reg<u32, _I2CADM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADM0;
#[doc = "`read()` method returns [i2cadm0::R](i2cadm0::R) reader structure"]
impl crate::Readable for I2CADM0 {}
#[doc = "`write(|w| ..)` method takes [i2cadm0::W](i2cadm0::W) writer structure"]
impl crate::Writable for I2CADM0 {}
#[doc = "I2C Slave Address Mask Register0"]
pub mod i2cadm0;
#[doc = "I2C Slave Address Mask Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cadm1](i2cadm1) module"]
pub type I2CADM1 = crate::Reg<u32, _I2CADM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADM1;
#[doc = "`read()` method returns [i2cadm1::R](i2cadm1::R) reader structure"]
impl crate::Readable for I2CADM1 {}
#[doc = "`write(|w| ..)` method takes [i2cadm1::W](i2cadm1::W) writer structure"]
impl crate::Writable for I2CADM1 {}
#[doc = "I2C Slave Address Mask Register1"]
pub mod i2cadm1;
#[doc = "I2C Slave Address Mask Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cadm2](i2cadm2) module"]
pub type I2CADM2 = crate::Reg<u32, _I2CADM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADM2;
#[doc = "`read()` method returns [i2cadm2::R](i2cadm2::R) reader structure"]
impl crate::Readable for I2CADM2 {}
#[doc = "`write(|w| ..)` method takes [i2cadm2::W](i2cadm2::W) writer structure"]
impl crate::Writable for I2CADM2 {}
#[doc = "I2C Slave Address Mask Register2"]
pub mod i2cadm2;
#[doc = "I2C Slave Address Mask Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cadm3](i2cadm3) module"]
pub type I2CADM3 = crate::Reg<u32, _I2CADM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CADM3;
#[doc = "`read()` method returns [i2cadm3::R](i2cadm3::R) reader structure"]
impl crate::Readable for I2CADM3 {}
#[doc = "`write(|w| ..)` method takes [i2cadm3::W](i2cadm3::W) writer structure"]
impl crate::Writable for I2CADM3 {}
#[doc = "I2C Slave Address Mask Register3"]
pub mod i2cadm3;
