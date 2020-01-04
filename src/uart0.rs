#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ua: [u8; 4usize],
    #[doc = "0x04 - Interrupt Enable Register."]
    pub ua_ier: UA_IER,
    #[doc = "0x08 - FIFO Control Register."]
    pub ua_fcr: UA_FCR,
    #[doc = "0x0c - Line Control Register."]
    pub ua_lcr: UA_LCR,
    #[doc = "0x10 - Modem Control Register."]
    pub ua_mcr: UA_MCR,
    #[doc = "0x14 - Modem Status Register."]
    pub ua_msr: UA_MSR,
    #[doc = "0x18 - FIFO Status Register."]
    pub ua_fsr: UA_FSR,
    #[doc = "0x1c - Interrupt Status Register."]
    pub ua_isr: UA_ISR,
    #[doc = "0x20 - Time Out Register"]
    pub ua_tor: UA_TOR,
    #[doc = "0x24 - Baud Rate Divisor Register"]
    pub ua_baud: UA_BAUD,
    #[doc = "0x28 - IrDA Control Register."]
    pub ua_ircr: UA_IRCR,
    #[doc = "0x2c - LIN Break Failed Count Register."]
    pub ua_alt_csr: UA_ALT_CSR,
    #[doc = "0x30 - Function Select Register."]
    pub ua_fun_sel: UA_FUN_SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Holding Register."]
    #[inline(always)]
    pub fn ua_thr(&self) -> &UA_THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UA_THR) }
    }
    #[doc = "0x00 - Transmit Holding Register."]
    #[inline(always)]
    pub fn ua_thr_mut(&self) -> &mut UA_THR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UA_THR) }
    }
    #[doc = "0x00 - Receive Buffer Register."]
    #[inline(always)]
    pub fn ua_rbr(&self) -> &UA_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UA_RBR) }
    }
    #[doc = "0x00 - Receive Buffer Register."]
    #[inline(always)]
    pub fn ua_rbr_mut(&self) -> &mut UA_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UA_RBR) }
    }
}
#[doc = "Receive Buffer Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_rbr](ua_rbr) module"]
pub type UA_RBR = crate::Reg<u32, _UA_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_RBR;
#[doc = "`read()` method returns [ua_rbr::R](ua_rbr::R) reader structure"]
impl crate::Readable for UA_RBR {}
#[doc = "Receive Buffer Register."]
pub mod ua_rbr;
#[doc = "Transmit Holding Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_thr](ua_thr) module"]
pub type UA_THR = crate::Reg<u32, _UA_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_THR;
#[doc = "`read()` method returns [ua_thr::R](ua_thr::R) reader structure"]
impl crate::Readable for UA_THR {}
#[doc = "`write(|w| ..)` method takes [ua_thr::W](ua_thr::W) writer structure"]
impl crate::Writable for UA_THR {}
#[doc = "Transmit Holding Register."]
pub mod ua_thr;
#[doc = "Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_ier](ua_ier) module"]
pub type UA_IER = crate::Reg<u32, _UA_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_IER;
#[doc = "`read()` method returns [ua_ier::R](ua_ier::R) reader structure"]
impl crate::Readable for UA_IER {}
#[doc = "`write(|w| ..)` method takes [ua_ier::W](ua_ier::W) writer structure"]
impl crate::Writable for UA_IER {}
#[doc = "Interrupt Enable Register."]
pub mod ua_ier;
#[doc = "FIFO Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_fcr](ua_fcr) module"]
pub type UA_FCR = crate::Reg<u32, _UA_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_FCR;
#[doc = "`read()` method returns [ua_fcr::R](ua_fcr::R) reader structure"]
impl crate::Readable for UA_FCR {}
#[doc = "`write(|w| ..)` method takes [ua_fcr::W](ua_fcr::W) writer structure"]
impl crate::Writable for UA_FCR {}
#[doc = "FIFO Control Register."]
pub mod ua_fcr;
#[doc = "Line Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_lcr](ua_lcr) module"]
pub type UA_LCR = crate::Reg<u32, _UA_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_LCR;
#[doc = "`read()` method returns [ua_lcr::R](ua_lcr::R) reader structure"]
impl crate::Readable for UA_LCR {}
#[doc = "`write(|w| ..)` method takes [ua_lcr::W](ua_lcr::W) writer structure"]
impl crate::Writable for UA_LCR {}
#[doc = "Line Control Register."]
pub mod ua_lcr;
#[doc = "Modem Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_mcr](ua_mcr) module"]
pub type UA_MCR = crate::Reg<u32, _UA_MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_MCR;
#[doc = "`read()` method returns [ua_mcr::R](ua_mcr::R) reader structure"]
impl crate::Readable for UA_MCR {}
#[doc = "`write(|w| ..)` method takes [ua_mcr::W](ua_mcr::W) writer structure"]
impl crate::Writable for UA_MCR {}
#[doc = "Modem Control Register."]
pub mod ua_mcr;
#[doc = "Modem Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_msr](ua_msr) module"]
pub type UA_MSR = crate::Reg<u32, _UA_MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_MSR;
#[doc = "`read()` method returns [ua_msr::R](ua_msr::R) reader structure"]
impl crate::Readable for UA_MSR {}
#[doc = "`write(|w| ..)` method takes [ua_msr::W](ua_msr::W) writer structure"]
impl crate::Writable for UA_MSR {}
#[doc = "Modem Status Register."]
pub mod ua_msr;
#[doc = "FIFO Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_fsr](ua_fsr) module"]
pub type UA_FSR = crate::Reg<u32, _UA_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_FSR;
#[doc = "`read()` method returns [ua_fsr::R](ua_fsr::R) reader structure"]
impl crate::Readable for UA_FSR {}
#[doc = "`write(|w| ..)` method takes [ua_fsr::W](ua_fsr::W) writer structure"]
impl crate::Writable for UA_FSR {}
#[doc = "FIFO Status Register."]
pub mod ua_fsr;
#[doc = "Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_isr](ua_isr) module"]
pub type UA_ISR = crate::Reg<u32, _UA_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_ISR;
#[doc = "`read()` method returns [ua_isr::R](ua_isr::R) reader structure"]
impl crate::Readable for UA_ISR {}
#[doc = "`write(|w| ..)` method takes [ua_isr::W](ua_isr::W) writer structure"]
impl crate::Writable for UA_ISR {}
#[doc = "Interrupt Status Register."]
pub mod ua_isr;
#[doc = "Time Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_tor](ua_tor) module"]
pub type UA_TOR = crate::Reg<u32, _UA_TOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_TOR;
#[doc = "`read()` method returns [ua_tor::R](ua_tor::R) reader structure"]
impl crate::Readable for UA_TOR {}
#[doc = "`write(|w| ..)` method takes [ua_tor::W](ua_tor::W) writer structure"]
impl crate::Writable for UA_TOR {}
#[doc = "Time Out Register"]
pub mod ua_tor;
#[doc = "Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_baud](ua_baud) module"]
pub type UA_BAUD = crate::Reg<u32, _UA_BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_BAUD;
#[doc = "`read()` method returns [ua_baud::R](ua_baud::R) reader structure"]
impl crate::Readable for UA_BAUD {}
#[doc = "`write(|w| ..)` method takes [ua_baud::W](ua_baud::W) writer structure"]
impl crate::Writable for UA_BAUD {}
#[doc = "Baud Rate Divisor Register"]
pub mod ua_baud;
#[doc = "IrDA Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_ircr](ua_ircr) module"]
pub type UA_IRCR = crate::Reg<u32, _UA_IRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_IRCR;
#[doc = "`read()` method returns [ua_ircr::R](ua_ircr::R) reader structure"]
impl crate::Readable for UA_IRCR {}
#[doc = "`write(|w| ..)` method takes [ua_ircr::W](ua_ircr::W) writer structure"]
impl crate::Writable for UA_IRCR {}
#[doc = "IrDA Control Register."]
pub mod ua_ircr;
#[doc = "LIN Break Failed Count Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_alt_csr](ua_alt_csr) module"]
pub type UA_ALT_CSR = crate::Reg<u32, _UA_ALT_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_ALT_CSR;
#[doc = "`read()` method returns [ua_alt_csr::R](ua_alt_csr::R) reader structure"]
impl crate::Readable for UA_ALT_CSR {}
#[doc = "`write(|w| ..)` method takes [ua_alt_csr::W](ua_alt_csr::W) writer structure"]
impl crate::Writable for UA_ALT_CSR {}
#[doc = "LIN Break Failed Count Register."]
pub mod ua_alt_csr;
#[doc = "Function Select Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ua_fun_sel](ua_fun_sel) module"]
pub type UA_FUN_SEL = crate::Reg<u32, _UA_FUN_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UA_FUN_SEL;
#[doc = "`read()` method returns [ua_fun_sel::R](ua_fun_sel::R) reader structure"]
impl crate::Readable for UA_FUN_SEL {}
#[doc = "`write(|w| ..)` method takes [ua_fun_sel::W](ua_fun_sel::W) writer structure"]
impl crate::Writable for UA_FUN_SEL {}
#[doc = "Function Select Register."]
pub mod ua_fun_sel;
