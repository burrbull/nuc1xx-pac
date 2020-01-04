#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    pub spi_cntrl: SPI_CNTRL,
    #[doc = "0x04 - Clock Divider Register"]
    pub spi_divider: SPI_DIVIDER,
    #[doc = "0x08 - Slave Select Register"]
    pub spi_ssr: SPI_SSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Data Receive Register 0"]
    pub spi_rx0: SPI_RX0,
    #[doc = "0x14 - Data Receive Register 1"]
    pub spi_rx1: SPI_RX1,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - Data Transmit Register 0"]
    pub spi_tx0: SPI_TX0,
    #[doc = "0x24 - Data Transmit Register 1"]
    pub spi_tx1: SPI_TX1,
    _reserved7: [u8; 12usize],
    #[doc = "0x34 - Variable Clock Pattern Register"]
    pub spi_varclk: SPI_VARCLK,
    #[doc = "0x38 - SPI DMA control register"]
    pub spi_dma: SPI_DMA,
}
#[doc = "Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cntrl](spi_cntrl) module"]
pub type SPI_CNTRL = crate::Reg<u32, _SPI_CNTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CNTRL;
#[doc = "`read()` method returns [spi_cntrl::R](spi_cntrl::R) reader structure"]
impl crate::Readable for SPI_CNTRL {}
#[doc = "`write(|w| ..)` method takes [spi_cntrl::W](spi_cntrl::W) writer structure"]
impl crate::Writable for SPI_CNTRL {}
#[doc = "Control and Status Register"]
pub mod spi_cntrl;
#[doc = "Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_divider](spi_divider) module"]
pub type SPI_DIVIDER = crate::Reg<u32, _SPI_DIVIDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DIVIDER;
#[doc = "`read()` method returns [spi_divider::R](spi_divider::R) reader structure"]
impl crate::Readable for SPI_DIVIDER {}
#[doc = "`write(|w| ..)` method takes [spi_divider::W](spi_divider::W) writer structure"]
impl crate::Writable for SPI_DIVIDER {}
#[doc = "Clock Divider Register"]
pub mod spi_divider;
#[doc = "Slave Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ssr](spi_ssr) module"]
pub type SPI_SSR = crate::Reg<u32, _SPI_SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SSR;
#[doc = "`read()` method returns [spi_ssr::R](spi_ssr::R) reader structure"]
impl crate::Readable for SPI_SSR {}
#[doc = "`write(|w| ..)` method takes [spi_ssr::W](spi_ssr::W) writer structure"]
impl crate::Writable for SPI_SSR {}
#[doc = "Slave Select Register"]
pub mod spi_ssr;
#[doc = "Data Receive Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rx0](spi_rx0) module"]
pub type SPI_RX0 = crate::Reg<u32, _SPI_RX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RX0;
#[doc = "`read()` method returns [spi_rx0::R](spi_rx0::R) reader structure"]
impl crate::Readable for SPI_RX0 {}
#[doc = "Data Receive Register 0"]
pub mod spi_rx0;
#[doc = "Data Receive Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rx1](spi_rx1) module"]
pub type SPI_RX1 = crate::Reg<u32, _SPI_RX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RX1;
#[doc = "`read()` method returns [spi_rx1::R](spi_rx1::R) reader structure"]
impl crate::Readable for SPI_RX1 {}
#[doc = "Data Receive Register 1"]
pub mod spi_rx1;
#[doc = "Data Transmit Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_tx0](spi_tx0) module"]
pub type SPI_TX0 = crate::Reg<u32, _SPI_TX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_TX0;
#[doc = "`write(|w| ..)` method takes [spi_tx0::W](spi_tx0::W) writer structure"]
impl crate::Writable for SPI_TX0 {}
#[doc = "Data Transmit Register 0"]
pub mod spi_tx0;
#[doc = "Data Transmit Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_tx1](spi_tx1) module"]
pub type SPI_TX1 = crate::Reg<u32, _SPI_TX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_TX1;
#[doc = "`write(|w| ..)` method takes [spi_tx1::W](spi_tx1::W) writer structure"]
impl crate::Writable for SPI_TX1 {}
#[doc = "Data Transmit Register 1"]
pub mod spi_tx1;
#[doc = "Variable Clock Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_varclk](spi_varclk) module"]
pub type SPI_VARCLK = crate::Reg<u32, _SPI_VARCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_VARCLK;
#[doc = "`read()` method returns [spi_varclk::R](spi_varclk::R) reader structure"]
impl crate::Readable for SPI_VARCLK {}
#[doc = "`write(|w| ..)` method takes [spi_varclk::W](spi_varclk::W) writer structure"]
impl crate::Writable for SPI_VARCLK {}
#[doc = "Variable Clock Pattern Register"]
pub mod spi_varclk;
#[doc = "SPI DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma](spi_dma) module"]
pub type SPI_DMA = crate::Reg<u32, _SPI_DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA;
#[doc = "`read()` method returns [spi_dma::R](spi_dma::R) reader structure"]
impl crate::Readable for SPI_DMA {}
#[doc = "`write(|w| ..)` method takes [spi_dma::W](spi_dma::W) writer structure"]
impl crate::Writable for SPI_DMA {}
#[doc = "SPI DMA control register"]
pub mod spi_dma;
