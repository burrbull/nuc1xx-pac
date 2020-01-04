#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Flag"]
    pub usb_inten: USB_INTEN,
    #[doc = "0x04 - Interrupt Event Flag"]
    pub usb_intsts: USB_INTSTS,
    #[doc = "0x08 - Function Address"]
    pub usb_faddr: USB_FADDR,
    #[doc = "0x0c - System state"]
    pub usb_epsts: USB_EPSTS,
    #[doc = "0x10 - Bus state & attribution"]
    pub usb_attr: USB_ATTR,
    #[doc = "0x14 - Device Floating Detected"]
    pub usb_fldet: USB_FLDET,
    #[doc = "0x18 - Buffer Segmentation"]
    pub usb_bufseg: USB_BUFSEG,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Buffer Segmentation of endpoint 0"]
    pub usb_bufseg0: USB_BUFSEG0,
    #[doc = "0x24 - Maximal payload of endpoint 0"]
    pub usb_mxpld0: USB_MXPLD0,
    #[doc = "0x28 - Configuration of endpoint 0"]
    pub usb_cfg0: USB_CFG0,
    #[doc = "0x2c - stall control register and In/out ready clear flag of endpoint 0"]
    pub usb_cfgp0: USB_CFGP0,
    #[doc = "0x30 - Buffer Segmentation of endpoint 1"]
    pub usb_bufseg1: USB_BUFSEG1,
    #[doc = "0x34 - Maximal payload of endpoint 1"]
    pub usb_mxpld1: USB_MXPLD1,
    #[doc = "0x38 - Configuration of endpoint 1"]
    pub usb_cfg1: USB_CFG1,
    #[doc = "0x3c - stall control register and In/out ready clear flag of endpoint 1"]
    pub usb_cfgp1: USB_CFGP1,
    #[doc = "0x40 - Buffer Segmentation of endpoint 2"]
    pub usb_bufseg2: USB_BUFSEG2,
    #[doc = "0x44 - Maximal payload of endpoint 2"]
    pub usb_mxpld2: USB_MXPLD2,
    #[doc = "0x48 - Configuration of endpoint 2"]
    pub usb_cfg2: USB_CFG2,
    #[doc = "0x4c - stall control register and In/out ready clear flag of endpoint 2"]
    pub usb_cfgp2: USB_CFGP2,
    #[doc = "0x50 - Buffer Segmentation of endpoint 3"]
    pub usb_bufseg3: USB_BUFSEG3,
    #[doc = "0x54 - Maximal payload of endpoint 3"]
    pub usb_mxpld3: USB_MXPLD3,
    #[doc = "0x58 - Configuration of endpoint 3"]
    pub usb_cfg3: USB_CFG3,
    #[doc = "0x5c - stall control register and In/out ready clear flag of endpoint 3"]
    pub usb_cfgp3: USB_CFGP3,
    #[doc = "0x60 - Buffer Segmentation of endpoint 4"]
    pub usb_bufseg4: USB_BUFSEG4,
    #[doc = "0x64 - Maximal payload of endpoint 4"]
    pub usb_mxpld4: USB_MXPLD4,
    #[doc = "0x68 - Configuration of endpoint 4"]
    pub usb_cfg4: USB_CFG4,
    #[doc = "0x6c - stall control register and In/out ready clear flag of endpoint 4"]
    pub usb_cfgp4: USB_CFGP4,
    #[doc = "0x70 - Buffer Segmentation of endpoint 5"]
    pub usb_bufseg5: USB_BUFSEG5,
    #[doc = "0x74 - Maximal payload of endpoint 5"]
    pub usb_mxpld5: USB_MXPLD5,
    #[doc = "0x78 - Configuration of endpoint 5"]
    pub usb_cfg5: USB_CFG5,
    #[doc = "0x7c - In ready clear flag of endpoint 5"]
    pub usb_cfgp5: USB_CFGP5,
    _reserved31: [u8; 16usize],
    #[doc = "0x90 - Drive Single Ended Zero (SE0) in USB Bus"]
    pub usb_drvse0: USB_DRVSE0,
    _reserved32: [u8; 16usize],
    #[doc = "0xa4 - New description for register"]
    pub usb_pdma: USB_PDMA,
}
#[doc = "Interrupt Enable Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_inten](usb_inten) module"]
pub type USB_INTEN = crate::Reg<u32, _USB_INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_INTEN;
#[doc = "`read()` method returns [usb_inten::R](usb_inten::R) reader structure"]
impl crate::Readable for USB_INTEN {}
#[doc = "`write(|w| ..)` method takes [usb_inten::W](usb_inten::W) writer structure"]
impl crate::Writable for USB_INTEN {}
#[doc = "Interrupt Enable Flag"]
pub mod usb_inten;
#[doc = "Interrupt Event Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_intsts](usb_intsts) module"]
pub type USB_INTSTS = crate::Reg<u32, _USB_INTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_INTSTS;
#[doc = "`read()` method returns [usb_intsts::R](usb_intsts::R) reader structure"]
impl crate::Readable for USB_INTSTS {}
#[doc = "`write(|w| ..)` method takes [usb_intsts::W](usb_intsts::W) writer structure"]
impl crate::Writable for USB_INTSTS {}
#[doc = "Interrupt Event Flag"]
pub mod usb_intsts;
#[doc = "Function Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_faddr](usb_faddr) module"]
pub type USB_FADDR = crate::Reg<u32, _USB_FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_FADDR;
#[doc = "`read()` method returns [usb_faddr::R](usb_faddr::R) reader structure"]
impl crate::Readable for USB_FADDR {}
#[doc = "`write(|w| ..)` method takes [usb_faddr::W](usb_faddr::W) writer structure"]
impl crate::Writable for USB_FADDR {}
#[doc = "Function Address"]
pub mod usb_faddr;
#[doc = "System state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_epsts](usb_epsts) module"]
pub type USB_EPSTS = crate::Reg<u32, _USB_EPSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_EPSTS;
#[doc = "`read()` method returns [usb_epsts::R](usb_epsts::R) reader structure"]
impl crate::Readable for USB_EPSTS {}
#[doc = "System state"]
pub mod usb_epsts;
#[doc = "Bus state & attribution\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_attr](usb_attr) module"]
pub type USB_ATTR = crate::Reg<u32, _USB_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_ATTR;
#[doc = "`read()` method returns [usb_attr::R](usb_attr::R) reader structure"]
impl crate::Readable for USB_ATTR {}
#[doc = "`write(|w| ..)` method takes [usb_attr::W](usb_attr::W) writer structure"]
impl crate::Writable for USB_ATTR {}
#[doc = "Bus state & attribution"]
pub mod usb_attr;
#[doc = "Device Floating Detected\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_fldet](usb_fldet) module"]
pub type USB_FLDET = crate::Reg<u32, _USB_FLDET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_FLDET;
#[doc = "`read()` method returns [usb_fldet::R](usb_fldet::R) reader structure"]
impl crate::Readable for USB_FLDET {}
#[doc = "Device Floating Detected"]
pub mod usb_fldet;
#[doc = "Buffer Segmentation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg](usb_bufseg) module"]
pub type USB_BUFSEG = crate::Reg<u32, _USB_BUFSEG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG;
#[doc = "`read()` method returns [usb_bufseg::R](usb_bufseg::R) reader structure"]
impl crate::Readable for USB_BUFSEG {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg::W](usb_bufseg::W) writer structure"]
impl crate::Writable for USB_BUFSEG {}
#[doc = "Buffer Segmentation"]
pub mod usb_bufseg;
#[doc = "Buffer Segmentation of endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg0](usb_bufseg0) module"]
pub type USB_BUFSEG0 = crate::Reg<u32, _USB_BUFSEG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG0;
#[doc = "`read()` method returns [usb_bufseg0::R](usb_bufseg0::R) reader structure"]
impl crate::Readable for USB_BUFSEG0 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg0::W](usb_bufseg0::W) writer structure"]
impl crate::Writable for USB_BUFSEG0 {}
#[doc = "Buffer Segmentation of endpoint 0"]
pub mod usb_bufseg0;
#[doc = "Maximal payload of endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld0](usb_mxpld0) module"]
pub type USB_MXPLD0 = crate::Reg<u32, _USB_MXPLD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD0;
#[doc = "`read()` method returns [usb_mxpld0::R](usb_mxpld0::R) reader structure"]
impl crate::Readable for USB_MXPLD0 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld0::W](usb_mxpld0::W) writer structure"]
impl crate::Writable for USB_MXPLD0 {}
#[doc = "Maximal payload of endpoint 0"]
pub mod usb_mxpld0;
#[doc = "Configuration of endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg0](usb_cfg0) module"]
pub type USB_CFG0 = crate::Reg<u32, _USB_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG0;
#[doc = "`read()` method returns [usb_cfg0::R](usb_cfg0::R) reader structure"]
impl crate::Readable for USB_CFG0 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg0::W](usb_cfg0::W) writer structure"]
impl crate::Writable for USB_CFG0 {}
#[doc = "Configuration of endpoint 0"]
pub mod usb_cfg0;
#[doc = "stall control register and In/out ready clear flag of endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp0](usb_cfgp0) module"]
pub type USB_CFGP0 = crate::Reg<u32, _USB_CFGP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP0;
#[doc = "`read()` method returns [usb_cfgp0::R](usb_cfgp0::R) reader structure"]
impl crate::Readable for USB_CFGP0 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp0::W](usb_cfgp0::W) writer structure"]
impl crate::Writable for USB_CFGP0 {}
#[doc = "stall control register and In/out ready clear flag of endpoint 0"]
pub mod usb_cfgp0;
#[doc = "Buffer Segmentation of endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg1](usb_bufseg1) module"]
pub type USB_BUFSEG1 = crate::Reg<u32, _USB_BUFSEG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG1;
#[doc = "`read()` method returns [usb_bufseg1::R](usb_bufseg1::R) reader structure"]
impl crate::Readable for USB_BUFSEG1 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg1::W](usb_bufseg1::W) writer structure"]
impl crate::Writable for USB_BUFSEG1 {}
#[doc = "Buffer Segmentation of endpoint 1"]
pub mod usb_bufseg1;
#[doc = "Maximal payload of endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld1](usb_mxpld1) module"]
pub type USB_MXPLD1 = crate::Reg<u32, _USB_MXPLD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD1;
#[doc = "`read()` method returns [usb_mxpld1::R](usb_mxpld1::R) reader structure"]
impl crate::Readable for USB_MXPLD1 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld1::W](usb_mxpld1::W) writer structure"]
impl crate::Writable for USB_MXPLD1 {}
#[doc = "Maximal payload of endpoint 1"]
pub mod usb_mxpld1;
#[doc = "Configuration of endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg1](usb_cfg1) module"]
pub type USB_CFG1 = crate::Reg<u32, _USB_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG1;
#[doc = "`read()` method returns [usb_cfg1::R](usb_cfg1::R) reader structure"]
impl crate::Readable for USB_CFG1 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg1::W](usb_cfg1::W) writer structure"]
impl crate::Writable for USB_CFG1 {}
#[doc = "Configuration of endpoint 1"]
pub mod usb_cfg1;
#[doc = "stall control register and In/out ready clear flag of endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp1](usb_cfgp1) module"]
pub type USB_CFGP1 = crate::Reg<u32, _USB_CFGP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP1;
#[doc = "`read()` method returns [usb_cfgp1::R](usb_cfgp1::R) reader structure"]
impl crate::Readable for USB_CFGP1 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp1::W](usb_cfgp1::W) writer structure"]
impl crate::Writable for USB_CFGP1 {}
#[doc = "stall control register and In/out ready clear flag of endpoint 1"]
pub mod usb_cfgp1;
#[doc = "Buffer Segmentation of endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg2](usb_bufseg2) module"]
pub type USB_BUFSEG2 = crate::Reg<u32, _USB_BUFSEG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG2;
#[doc = "`read()` method returns [usb_bufseg2::R](usb_bufseg2::R) reader structure"]
impl crate::Readable for USB_BUFSEG2 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg2::W](usb_bufseg2::W) writer structure"]
impl crate::Writable for USB_BUFSEG2 {}
#[doc = "Buffer Segmentation of endpoint 2"]
pub mod usb_bufseg2;
#[doc = "Maximal payload of endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld2](usb_mxpld2) module"]
pub type USB_MXPLD2 = crate::Reg<u32, _USB_MXPLD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD2;
#[doc = "`read()` method returns [usb_mxpld2::R](usb_mxpld2::R) reader structure"]
impl crate::Readable for USB_MXPLD2 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld2::W](usb_mxpld2::W) writer structure"]
impl crate::Writable for USB_MXPLD2 {}
#[doc = "Maximal payload of endpoint 2"]
pub mod usb_mxpld2;
#[doc = "Configuration of endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg2](usb_cfg2) module"]
pub type USB_CFG2 = crate::Reg<u32, _USB_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG2;
#[doc = "`read()` method returns [usb_cfg2::R](usb_cfg2::R) reader structure"]
impl crate::Readable for USB_CFG2 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg2::W](usb_cfg2::W) writer structure"]
impl crate::Writable for USB_CFG2 {}
#[doc = "Configuration of endpoint 2"]
pub mod usb_cfg2;
#[doc = "stall control register and In/out ready clear flag of endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp2](usb_cfgp2) module"]
pub type USB_CFGP2 = crate::Reg<u32, _USB_CFGP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP2;
#[doc = "`read()` method returns [usb_cfgp2::R](usb_cfgp2::R) reader structure"]
impl crate::Readable for USB_CFGP2 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp2::W](usb_cfgp2::W) writer structure"]
impl crate::Writable for USB_CFGP2 {}
#[doc = "stall control register and In/out ready clear flag of endpoint 2"]
pub mod usb_cfgp2;
#[doc = "Buffer Segmentation of endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg3](usb_bufseg3) module"]
pub type USB_BUFSEG3 = crate::Reg<u32, _USB_BUFSEG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG3;
#[doc = "`read()` method returns [usb_bufseg3::R](usb_bufseg3::R) reader structure"]
impl crate::Readable for USB_BUFSEG3 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg3::W](usb_bufseg3::W) writer structure"]
impl crate::Writable for USB_BUFSEG3 {}
#[doc = "Buffer Segmentation of endpoint 3"]
pub mod usb_bufseg3;
#[doc = "Maximal payload of endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld3](usb_mxpld3) module"]
pub type USB_MXPLD3 = crate::Reg<u32, _USB_MXPLD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD3;
#[doc = "`read()` method returns [usb_mxpld3::R](usb_mxpld3::R) reader structure"]
impl crate::Readable for USB_MXPLD3 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld3::W](usb_mxpld3::W) writer structure"]
impl crate::Writable for USB_MXPLD3 {}
#[doc = "Maximal payload of endpoint 3"]
pub mod usb_mxpld3;
#[doc = "Configuration of endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg3](usb_cfg3) module"]
pub type USB_CFG3 = crate::Reg<u32, _USB_CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG3;
#[doc = "`read()` method returns [usb_cfg3::R](usb_cfg3::R) reader structure"]
impl crate::Readable for USB_CFG3 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg3::W](usb_cfg3::W) writer structure"]
impl crate::Writable for USB_CFG3 {}
#[doc = "Configuration of endpoint 3"]
pub mod usb_cfg3;
#[doc = "stall control register and In/out ready clear flag of endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp3](usb_cfgp3) module"]
pub type USB_CFGP3 = crate::Reg<u32, _USB_CFGP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP3;
#[doc = "`read()` method returns [usb_cfgp3::R](usb_cfgp3::R) reader structure"]
impl crate::Readable for USB_CFGP3 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp3::W](usb_cfgp3::W) writer structure"]
impl crate::Writable for USB_CFGP3 {}
#[doc = "stall control register and In/out ready clear flag of endpoint 3"]
pub mod usb_cfgp3;
#[doc = "Buffer Segmentation of endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg4](usb_bufseg4) module"]
pub type USB_BUFSEG4 = crate::Reg<u32, _USB_BUFSEG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG4;
#[doc = "`read()` method returns [usb_bufseg4::R](usb_bufseg4::R) reader structure"]
impl crate::Readable for USB_BUFSEG4 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg4::W](usb_bufseg4::W) writer structure"]
impl crate::Writable for USB_BUFSEG4 {}
#[doc = "Buffer Segmentation of endpoint 4"]
pub mod usb_bufseg4;
#[doc = "Maximal payload of endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld4](usb_mxpld4) module"]
pub type USB_MXPLD4 = crate::Reg<u32, _USB_MXPLD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD4;
#[doc = "`read()` method returns [usb_mxpld4::R](usb_mxpld4::R) reader structure"]
impl crate::Readable for USB_MXPLD4 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld4::W](usb_mxpld4::W) writer structure"]
impl crate::Writable for USB_MXPLD4 {}
#[doc = "Maximal payload of endpoint 4"]
pub mod usb_mxpld4;
#[doc = "Configuration of endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg4](usb_cfg4) module"]
pub type USB_CFG4 = crate::Reg<u32, _USB_CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG4;
#[doc = "`read()` method returns [usb_cfg4::R](usb_cfg4::R) reader structure"]
impl crate::Readable for USB_CFG4 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg4::W](usb_cfg4::W) writer structure"]
impl crate::Writable for USB_CFG4 {}
#[doc = "Configuration of endpoint 4"]
pub mod usb_cfg4;
#[doc = "stall control register and In/out ready clear flag of endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp4](usb_cfgp4) module"]
pub type USB_CFGP4 = crate::Reg<u32, _USB_CFGP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP4;
#[doc = "`read()` method returns [usb_cfgp4::R](usb_cfgp4::R) reader structure"]
impl crate::Readable for USB_CFGP4 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp4::W](usb_cfgp4::W) writer structure"]
impl crate::Writable for USB_CFGP4 {}
#[doc = "stall control register and In/out ready clear flag of endpoint 4"]
pub mod usb_cfgp4;
#[doc = "Buffer Segmentation of endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bufseg5](usb_bufseg5) module"]
pub type USB_BUFSEG5 = crate::Reg<u32, _USB_BUFSEG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_BUFSEG5;
#[doc = "`read()` method returns [usb_bufseg5::R](usb_bufseg5::R) reader structure"]
impl crate::Readable for USB_BUFSEG5 {}
#[doc = "`write(|w| ..)` method takes [usb_bufseg5::W](usb_bufseg5::W) writer structure"]
impl crate::Writable for USB_BUFSEG5 {}
#[doc = "Buffer Segmentation of endpoint 5"]
pub mod usb_bufseg5;
#[doc = "Maximal payload of endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_mxpld5](usb_mxpld5) module"]
pub type USB_MXPLD5 = crate::Reg<u32, _USB_MXPLD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_MXPLD5;
#[doc = "`read()` method returns [usb_mxpld5::R](usb_mxpld5::R) reader structure"]
impl crate::Readable for USB_MXPLD5 {}
#[doc = "`write(|w| ..)` method takes [usb_mxpld5::W](usb_mxpld5::W) writer structure"]
impl crate::Writable for USB_MXPLD5 {}
#[doc = "Maximal payload of endpoint 5"]
pub mod usb_mxpld5;
#[doc = "Configuration of endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfg5](usb_cfg5) module"]
pub type USB_CFG5 = crate::Reg<u32, _USB_CFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFG5;
#[doc = "`read()` method returns [usb_cfg5::R](usb_cfg5::R) reader structure"]
impl crate::Readable for USB_CFG5 {}
#[doc = "`write(|w| ..)` method takes [usb_cfg5::W](usb_cfg5::W) writer structure"]
impl crate::Writable for USB_CFG5 {}
#[doc = "Configuration of endpoint 5"]
pub mod usb_cfg5;
#[doc = "In ready clear flag of endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cfgp5](usb_cfgp5) module"]
pub type USB_CFGP5 = crate::Reg<u32, _USB_CFGP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CFGP5;
#[doc = "`read()` method returns [usb_cfgp5::R](usb_cfgp5::R) reader structure"]
impl crate::Readable for USB_CFGP5 {}
#[doc = "`write(|w| ..)` method takes [usb_cfgp5::W](usb_cfgp5::W) writer structure"]
impl crate::Writable for USB_CFGP5 {}
#[doc = "In ready clear flag of endpoint 5"]
pub mod usb_cfgp5;
#[doc = "Drive Single Ended Zero (SE0) in USB Bus\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_drvse0](usb_drvse0) module"]
pub type USB_DRVSE0 = crate::Reg<u32, _USB_DRVSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_DRVSE0;
#[doc = "`read()` method returns [usb_drvse0::R](usb_drvse0::R) reader structure"]
impl crate::Readable for USB_DRVSE0 {}
#[doc = "`write(|w| ..)` method takes [usb_drvse0::W](usb_drvse0::W) writer structure"]
impl crate::Writable for USB_DRVSE0 {}
#[doc = "Drive Single Ended Zero (SE0) in USB Bus"]
pub mod usb_drvse0;
#[doc = "New description for register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_pdma](usb_pdma) module"]
pub type USB_PDMA = crate::Reg<u32, _USB_PDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_PDMA;
#[doc = "`read()` method returns [usb_pdma::R](usb_pdma::R) reader structure"]
impl crate::Readable for USB_PDMA {}
#[doc = "`write(|w| ..)` method takes [usb_pdma::W](usb_pdma::W) writer structure"]
impl crate::Writable for USB_PDMA {}
#[doc = "New description for register"]
pub mod usb_pdma;
