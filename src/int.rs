#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU IRQ0 (BOD) interrupt source identify"]
    pub irq0_src: IRQ0_SRC,
    #[doc = "0x04 - MCU IRQ1 (WDG) interrupt source identify"]
    pub irq1_src: IRQ1_SRC,
    #[doc = "0x08 - MCU IRQ2 (EINT0) interrupt source identify"]
    pub irq2_src: IRQ2_SRC,
    #[doc = "0x0c - MCU IRQ3 (EINT1) interrupt source identify"]
    pub irq3_src: IRQ3_SRC,
    #[doc = "0x10 - MCU IRQ4 (GPA/B) interrupt source identify"]
    pub irq4_src: IRQ4_SRC,
    #[doc = "0x14 - MCU IRQ5 (GPC/D/E) interrupt source identify"]
    pub irq5_src: IRQ5_SRC,
    #[doc = "0x18 - MCU IRQ6 (PWMA) interrupt source identify"]
    pub irq6_src: IRQ6_SRC,
    #[doc = "0x1c - MCU IRQ7 (PWMB) interrupt source identify"]
    pub irq7_src: IRQ7_SRC,
    #[doc = "0x20 - MCU IRQ8 (TMR0) interrupt source identify"]
    pub irq8_src: IRQ8_SRC,
    #[doc = "0x24 - MCU IRQ9 (TMR1) interrupt source identify"]
    pub irq9_src: IRQ9_SRC,
    #[doc = "0x28 - MCU IRQ10 (TMR2) interrupt source identify"]
    pub irq10_src: IRQ10_SRC,
    #[doc = "0x2c - MCU IRQ11 (TMR3) interrupt source identify"]
    pub irq11_src: IRQ11_SRC,
    #[doc = "0x30 - MCU IRQ12 (URT0) interrupt source identify"]
    pub irq12_src: IRQ12_SRC,
    #[doc = "0x34 - MCU IRQ13 (URT1) interrupt source identify"]
    pub irq13_src: IRQ13_SRC,
    #[doc = "0x38 - MCU IRQ14 (SPI0) interrupt source identify"]
    pub irq14_src: IRQ14_SRC,
    #[doc = "0x3c - MCU IRQ15 (SPI1) interrupt source identify"]
    pub irq15_src: IRQ15_SRC,
    #[doc = "0x40 - MCU IRQ16 (SPI2) interrupt source identify"]
    pub irq16_src: IRQ16_SRC,
    #[doc = "0x44 - MCU IRQ17 (SPI3) interrupt source identify"]
    pub irq17_src: IRQ17_SRC,
    #[doc = "0x48 - MCU IRQ18 (I2C0) interrupt source identify"]
    pub irq18_src: IRQ18_SRC,
    #[doc = "0x4c - MCU IRQ19 (I2C1) interrupt source identify"]
    pub irq19_src: IRQ19_SRC,
    #[doc = "0x50 - MCU IRQ20 (CAN0) interrupt source identify"]
    pub irq20_src: IRQ20_SRC,
    #[doc = "0x54 - MCU IRQ21 (Reserved) interrupt source identity"]
    pub irq21_src: IRQ21_SRC,
    #[doc = "0x58 - MCU IRQ22 (Reserved) interrupt source identify"]
    pub irq22_src: IRQ22_SRC,
    #[doc = "0x5c - MCU IRQ23 (USBD) interrupt source identify"]
    pub irq23_src: IRQ23_SRC,
    #[doc = "0x60 - MCU IRQ24 (PS2) interrupt source identify"]
    pub irq24_src: IRQ24_SRC,
    #[doc = "0x64 - MCU IRQ25 (ACMP) interrupt source identify"]
    pub irq25_src: IRQ25_SRC,
    #[doc = "0x68 - MCU IRQ26 (PDMA) interrupt source identify"]
    pub irq26_src: IRQ26_SRC,
    #[doc = "0x6c - MCU IRQ27 (Reserved) interrupt source identify"]
    pub irq27_src: IRQ27_SRC,
    #[doc = "0x70 - MCU IRQ28 (PWRWU) interrupt source identify"]
    pub irq28_src: IRQ28_SRC,
    #[doc = "0x74 - MCU IRQ29 (ADC) interrupt source identify"]
    pub irq29_src: IRQ29_SRC,
    #[doc = "0x78 - MCU IRQ30 (Reserved) interrupt source identify"]
    pub irq30_src: IRQ30_SRC,
    #[doc = "0x7c - MCU IRQ31 (RTC) interrupt source identify"]
    pub irq31_src: IRQ31_SRC,
    #[doc = "0x80 - NMI source interrupt select control register"]
    pub nmi_sel: NMI_SEL,
    #[doc = "0x84 - MCU IRQ Number identify register"]
    pub mcu_irq: MCU_IRQ,
}
#[doc = "MCU IRQ0 (BOD) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq0_src](irq0_src) module"]
pub type IRQ0_SRC = crate::Reg<u32, _IRQ0_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ0_SRC;
#[doc = "`read()` method returns [irq0_src::R](irq0_src::R) reader structure"]
impl crate::Readable for IRQ0_SRC {}
#[doc = "MCU IRQ0 (BOD) interrupt source identify"]
pub mod irq0_src;
#[doc = "MCU IRQ1 (WDG) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq1_src](irq1_src) module"]
pub type IRQ1_SRC = crate::Reg<u32, _IRQ1_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ1_SRC;
#[doc = "`read()` method returns [irq1_src::R](irq1_src::R) reader structure"]
impl crate::Readable for IRQ1_SRC {}
#[doc = "MCU IRQ1 (WDG) interrupt source identify"]
pub mod irq1_src;
#[doc = "MCU IRQ2 (EINT0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq2_src](irq2_src) module"]
pub type IRQ2_SRC = crate::Reg<u32, _IRQ2_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ2_SRC;
#[doc = "`read()` method returns [irq2_src::R](irq2_src::R) reader structure"]
impl crate::Readable for IRQ2_SRC {}
#[doc = "MCU IRQ2 (EINT0) interrupt source identify"]
pub mod irq2_src;
#[doc = "MCU IRQ3 (EINT1) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq3_src](irq3_src) module"]
pub type IRQ3_SRC = crate::Reg<u32, _IRQ3_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ3_SRC;
#[doc = "`read()` method returns [irq3_src::R](irq3_src::R) reader structure"]
impl crate::Readable for IRQ3_SRC {}
#[doc = "MCU IRQ3 (EINT1) interrupt source identify"]
pub mod irq3_src;
#[doc = "MCU IRQ4 (GPA/B) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq4_src](irq4_src) module"]
pub type IRQ4_SRC = crate::Reg<u32, _IRQ4_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ4_SRC;
#[doc = "`read()` method returns [irq4_src::R](irq4_src::R) reader structure"]
impl crate::Readable for IRQ4_SRC {}
#[doc = "MCU IRQ4 (GPA/B) interrupt source identify"]
pub mod irq4_src;
#[doc = "MCU IRQ5 (GPC/D/E) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq5_src](irq5_src) module"]
pub type IRQ5_SRC = crate::Reg<u32, _IRQ5_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ5_SRC;
#[doc = "`read()` method returns [irq5_src::R](irq5_src::R) reader structure"]
impl crate::Readable for IRQ5_SRC {}
#[doc = "MCU IRQ5 (GPC/D/E) interrupt source identify"]
pub mod irq5_src;
#[doc = "MCU IRQ6 (PWMA) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq6_src](irq6_src) module"]
pub type IRQ6_SRC = crate::Reg<u32, _IRQ6_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ6_SRC;
#[doc = "`read()` method returns [irq6_src::R](irq6_src::R) reader structure"]
impl crate::Readable for IRQ6_SRC {}
#[doc = "MCU IRQ6 (PWMA) interrupt source identify"]
pub mod irq6_src;
#[doc = "MCU IRQ7 (PWMB) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq7_src](irq7_src) module"]
pub type IRQ7_SRC = crate::Reg<u32, _IRQ7_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ7_SRC;
#[doc = "`read()` method returns [irq7_src::R](irq7_src::R) reader structure"]
impl crate::Readable for IRQ7_SRC {}
#[doc = "MCU IRQ7 (PWMB) interrupt source identify"]
pub mod irq7_src;
#[doc = "MCU IRQ8 (TMR0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq8_src](irq8_src) module"]
pub type IRQ8_SRC = crate::Reg<u32, _IRQ8_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ8_SRC;
#[doc = "`read()` method returns [irq8_src::R](irq8_src::R) reader structure"]
impl crate::Readable for IRQ8_SRC {}
#[doc = "MCU IRQ8 (TMR0) interrupt source identify"]
pub mod irq8_src;
#[doc = "MCU IRQ9 (TMR1) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq9_src](irq9_src) module"]
pub type IRQ9_SRC = crate::Reg<u32, _IRQ9_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ9_SRC;
#[doc = "`read()` method returns [irq9_src::R](irq9_src::R) reader structure"]
impl crate::Readable for IRQ9_SRC {}
#[doc = "MCU IRQ9 (TMR1) interrupt source identify"]
pub mod irq9_src;
#[doc = "MCU IRQ10 (TMR2) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq10_src](irq10_src) module"]
pub type IRQ10_SRC = crate::Reg<u32, _IRQ10_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ10_SRC;
#[doc = "`read()` method returns [irq10_src::R](irq10_src::R) reader structure"]
impl crate::Readable for IRQ10_SRC {}
#[doc = "MCU IRQ10 (TMR2) interrupt source identify"]
pub mod irq10_src;
#[doc = "MCU IRQ11 (TMR3) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq11_src](irq11_src) module"]
pub type IRQ11_SRC = crate::Reg<u32, _IRQ11_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ11_SRC;
#[doc = "`read()` method returns [irq11_src::R](irq11_src::R) reader structure"]
impl crate::Readable for IRQ11_SRC {}
#[doc = "MCU IRQ11 (TMR3) interrupt source identify"]
pub mod irq11_src;
#[doc = "MCU IRQ12 (URT0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq12_src](irq12_src) module"]
pub type IRQ12_SRC = crate::Reg<u32, _IRQ12_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ12_SRC;
#[doc = "`read()` method returns [irq12_src::R](irq12_src::R) reader structure"]
impl crate::Readable for IRQ12_SRC {}
#[doc = "MCU IRQ12 (URT0) interrupt source identify"]
pub mod irq12_src;
#[doc = "MCU IRQ13 (URT1) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq13_src](irq13_src) module"]
pub type IRQ13_SRC = crate::Reg<u32, _IRQ13_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ13_SRC;
#[doc = "`read()` method returns [irq13_src::R](irq13_src::R) reader structure"]
impl crate::Readable for IRQ13_SRC {}
#[doc = "MCU IRQ13 (URT1) interrupt source identify"]
pub mod irq13_src;
#[doc = "MCU IRQ14 (SPI0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq14_src](irq14_src) module"]
pub type IRQ14_SRC = crate::Reg<u32, _IRQ14_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ14_SRC;
#[doc = "`read()` method returns [irq14_src::R](irq14_src::R) reader structure"]
impl crate::Readable for IRQ14_SRC {}
#[doc = "MCU IRQ14 (SPI0) interrupt source identify"]
pub mod irq14_src;
#[doc = "MCU IRQ15 (SPI1) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq15_src](irq15_src) module"]
pub type IRQ15_SRC = crate::Reg<u32, _IRQ15_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ15_SRC;
#[doc = "`read()` method returns [irq15_src::R](irq15_src::R) reader structure"]
impl crate::Readable for IRQ15_SRC {}
#[doc = "MCU IRQ15 (SPI1) interrupt source identify"]
pub mod irq15_src;
#[doc = "MCU IRQ16 (SPI2) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq16_src](irq16_src) module"]
pub type IRQ16_SRC = crate::Reg<u32, _IRQ16_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ16_SRC;
#[doc = "`read()` method returns [irq16_src::R](irq16_src::R) reader structure"]
impl crate::Readable for IRQ16_SRC {}
#[doc = "MCU IRQ16 (SPI2) interrupt source identify"]
pub mod irq16_src;
#[doc = "MCU IRQ17 (SPI3) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq17_src](irq17_src) module"]
pub type IRQ17_SRC = crate::Reg<u32, _IRQ17_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ17_SRC;
#[doc = "`read()` method returns [irq17_src::R](irq17_src::R) reader structure"]
impl crate::Readable for IRQ17_SRC {}
#[doc = "MCU IRQ17 (SPI3) interrupt source identify"]
pub mod irq17_src;
#[doc = "MCU IRQ18 (I2C0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq18_src](irq18_src) module"]
pub type IRQ18_SRC = crate::Reg<u32, _IRQ18_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ18_SRC;
#[doc = "`read()` method returns [irq18_src::R](irq18_src::R) reader structure"]
impl crate::Readable for IRQ18_SRC {}
#[doc = "MCU IRQ18 (I2C0) interrupt source identify"]
pub mod irq18_src;
#[doc = "MCU IRQ19 (I2C1) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq19_src](irq19_src) module"]
pub type IRQ19_SRC = crate::Reg<u32, _IRQ19_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ19_SRC;
#[doc = "`read()` method returns [irq19_src::R](irq19_src::R) reader structure"]
impl crate::Readable for IRQ19_SRC {}
#[doc = "MCU IRQ19 (I2C1) interrupt source identify"]
pub mod irq19_src;
#[doc = "MCU IRQ20 (CAN0) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq20_src](irq20_src) module"]
pub type IRQ20_SRC = crate::Reg<u32, _IRQ20_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ20_SRC;
#[doc = "`read()` method returns [irq20_src::R](irq20_src::R) reader structure"]
impl crate::Readable for IRQ20_SRC {}
#[doc = "MCU IRQ20 (CAN0) interrupt source identify"]
pub mod irq20_src;
#[doc = "MCU IRQ21 (Reserved) interrupt source identity\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq21_src](irq21_src) module"]
pub type IRQ21_SRC = crate::Reg<u32, _IRQ21_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ21_SRC;
#[doc = "`read()` method returns [irq21_src::R](irq21_src::R) reader structure"]
impl crate::Readable for IRQ21_SRC {}
#[doc = "MCU IRQ21 (Reserved) interrupt source identity"]
pub mod irq21_src;
#[doc = "MCU IRQ22 (Reserved) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq22_src](irq22_src) module"]
pub type IRQ22_SRC = crate::Reg<u32, _IRQ22_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ22_SRC;
#[doc = "`read()` method returns [irq22_src::R](irq22_src::R) reader structure"]
impl crate::Readable for IRQ22_SRC {}
#[doc = "MCU IRQ22 (Reserved) interrupt source identify"]
pub mod irq22_src;
#[doc = "MCU IRQ23 (USBD) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq23_src](irq23_src) module"]
pub type IRQ23_SRC = crate::Reg<u32, _IRQ23_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ23_SRC;
#[doc = "`read()` method returns [irq23_src::R](irq23_src::R) reader structure"]
impl crate::Readable for IRQ23_SRC {}
#[doc = "MCU IRQ23 (USBD) interrupt source identify"]
pub mod irq23_src;
#[doc = "MCU IRQ24 (PS2) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq24_src](irq24_src) module"]
pub type IRQ24_SRC = crate::Reg<u32, _IRQ24_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ24_SRC;
#[doc = "`read()` method returns [irq24_src::R](irq24_src::R) reader structure"]
impl crate::Readable for IRQ24_SRC {}
#[doc = "MCU IRQ24 (PS2) interrupt source identify"]
pub mod irq24_src;
#[doc = "MCU IRQ25 (ACMP) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq25_src](irq25_src) module"]
pub type IRQ25_SRC = crate::Reg<u32, _IRQ25_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ25_SRC;
#[doc = "`read()` method returns [irq25_src::R](irq25_src::R) reader structure"]
impl crate::Readable for IRQ25_SRC {}
#[doc = "MCU IRQ25 (ACMP) interrupt source identify"]
pub mod irq25_src;
#[doc = "MCU IRQ26 (PDMA) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq26_src](irq26_src) module"]
pub type IRQ26_SRC = crate::Reg<u32, _IRQ26_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ26_SRC;
#[doc = "`read()` method returns [irq26_src::R](irq26_src::R) reader structure"]
impl crate::Readable for IRQ26_SRC {}
#[doc = "MCU IRQ26 (PDMA) interrupt source identify"]
pub mod irq26_src;
#[doc = "MCU IRQ27 (Reserved) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq27_src](irq27_src) module"]
pub type IRQ27_SRC = crate::Reg<u32, _IRQ27_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ27_SRC;
#[doc = "`read()` method returns [irq27_src::R](irq27_src::R) reader structure"]
impl crate::Readable for IRQ27_SRC {}
#[doc = "MCU IRQ27 (Reserved) interrupt source identify"]
pub mod irq27_src;
#[doc = "MCU IRQ28 (PWRWU) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq28_src](irq28_src) module"]
pub type IRQ28_SRC = crate::Reg<u32, _IRQ28_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ28_SRC;
#[doc = "`read()` method returns [irq28_src::R](irq28_src::R) reader structure"]
impl crate::Readable for IRQ28_SRC {}
#[doc = "MCU IRQ28 (PWRWU) interrupt source identify"]
pub mod irq28_src;
#[doc = "MCU IRQ29 (ADC) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq29_src](irq29_src) module"]
pub type IRQ29_SRC = crate::Reg<u32, _IRQ29_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ29_SRC;
#[doc = "`read()` method returns [irq29_src::R](irq29_src::R) reader structure"]
impl crate::Readable for IRQ29_SRC {}
#[doc = "MCU IRQ29 (ADC) interrupt source identify"]
pub mod irq29_src;
#[doc = "MCU IRQ30 (Reserved) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq30_src](irq30_src) module"]
pub type IRQ30_SRC = crate::Reg<u32, _IRQ30_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ30_SRC;
#[doc = "`read()` method returns [irq30_src::R](irq30_src::R) reader structure"]
impl crate::Readable for IRQ30_SRC {}
#[doc = "MCU IRQ30 (Reserved) interrupt source identify"]
pub mod irq30_src;
#[doc = "MCU IRQ31 (RTC) interrupt source identify\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq31_src](irq31_src) module"]
pub type IRQ31_SRC = crate::Reg<u32, _IRQ31_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ31_SRC;
#[doc = "`read()` method returns [irq31_src::R](irq31_src::R) reader structure"]
impl crate::Readable for IRQ31_SRC {}
#[doc = "MCU IRQ31 (RTC) interrupt source identify"]
pub mod irq31_src;
#[doc = "NMI source interrupt select control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmi_sel](nmi_sel) module"]
pub type NMI_SEL = crate::Reg<u32, _NMI_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMI_SEL;
#[doc = "`read()` method returns [nmi_sel::R](nmi_sel::R) reader structure"]
impl crate::Readable for NMI_SEL {}
#[doc = "`write(|w| ..)` method takes [nmi_sel::W](nmi_sel::W) writer structure"]
impl crate::Writable for NMI_SEL {}
#[doc = "NMI source interrupt select control register"]
pub mod nmi_sel;
#[doc = "MCU IRQ Number identify register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_irq](mcu_irq) module"]
pub type MCU_IRQ = crate::Reg<u32, _MCU_IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_IRQ;
#[doc = "`read()` method returns [mcu_irq::R](mcu_irq::R) reader structure"]
impl crate::Readable for MCU_IRQ {}
#[doc = "`write(|w| ..)` method takes [mcu_irq::W](mcu_irq::W) writer structure"]
impl crate::Writable for MCU_IRQ {}
#[doc = "MCU IRQ Number identify register"]
pub mod mcu_irq;
