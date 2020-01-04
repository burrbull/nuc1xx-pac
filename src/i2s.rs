#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S Control Register"]
    pub i2s_con: I2S_CON,
    #[doc = "0x04 - I2S Clock Divider Register"]
    pub i2s_clkdiv: I2S_CLKDIV,
    #[doc = "0x08 - I2S Interrupt Enable Register"]
    pub i2s_ie: I2S_IE,
    #[doc = "0x0c - I2S Status Register"]
    pub i2s_status: I2S_STATUS,
    #[doc = "0x10 - I2S Transmit FIFO Register"]
    pub i2s_txfifo: I2S_TXFIFO,
    #[doc = "0x14 - I2S Receive FIFO Register"]
    pub i2s_rxfifo: I2S_RXFIFO,
}
#[doc = "I2S Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_con](i2s_con) module"]
pub type I2S_CON = crate::Reg<u32, _I2S_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CON;
#[doc = "`read()` method returns [i2s_con::R](i2s_con::R) reader structure"]
impl crate::Readable for I2S_CON {}
#[doc = "`write(|w| ..)` method takes [i2s_con::W](i2s_con::W) writer structure"]
impl crate::Writable for I2S_CON {}
#[doc = "I2S Control Register"]
pub mod i2s_con;
#[doc = "I2S Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_clkdiv](i2s_clkdiv) module"]
pub type I2S_CLKDIV = crate::Reg<u32, _I2S_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CLKDIV;
#[doc = "`read()` method returns [i2s_clkdiv::R](i2s_clkdiv::R) reader structure"]
impl crate::Readable for I2S_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [i2s_clkdiv::W](i2s_clkdiv::W) writer structure"]
impl crate::Writable for I2S_CLKDIV {}
#[doc = "I2S Clock Divider Register"]
pub mod i2s_clkdiv;
#[doc = "I2S Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_ie](i2s_ie) module"]
pub type I2S_IE = crate::Reg<u32, _I2S_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IE;
#[doc = "`read()` method returns [i2s_ie::R](i2s_ie::R) reader structure"]
impl crate::Readable for I2S_IE {}
#[doc = "`write(|w| ..)` method takes [i2s_ie::W](i2s_ie::W) writer structure"]
impl crate::Writable for I2S_IE {}
#[doc = "I2S Interrupt Enable Register"]
pub mod i2s_ie;
#[doc = "I2S Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_status](i2s_status) module"]
pub type I2S_STATUS = crate::Reg<u32, _I2S_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_STATUS;
#[doc = "`read()` method returns [i2s_status::R](i2s_status::R) reader structure"]
impl crate::Readable for I2S_STATUS {}
#[doc = "`write(|w| ..)` method takes [i2s_status::W](i2s_status::W) writer structure"]
impl crate::Writable for I2S_STATUS {}
#[doc = "I2S Status Register"]
pub mod i2s_status;
#[doc = "I2S Transmit FIFO Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_txfifo](i2s_txfifo) module"]
pub type I2S_TXFIFO = crate::Reg<u32, _I2S_TXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TXFIFO;
#[doc = "`write(|w| ..)` method takes [i2s_txfifo::W](i2s_txfifo::W) writer structure"]
impl crate::Writable for I2S_TXFIFO {}
#[doc = "I2S Transmit FIFO Register"]
pub mod i2s_txfifo;
#[doc = "I2S Receive FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rxfifo](i2s_rxfifo) module"]
pub type I2S_RXFIFO = crate::Reg<u32, _I2S_RXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RXFIFO;
#[doc = "`read()` method returns [i2s_rxfifo::R](i2s_rxfifo::R) reader structure"]
impl crate::Readable for I2S_RXFIFO {}
#[doc = "I2S Receive FIFO Register"]
pub mod i2s_rxfifo;
