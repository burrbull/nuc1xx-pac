#![doc = "Peripheral access API for NUC1XX_REGISTERS microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn BOD_OUT();
    fn WDT_INT();
    fn EINT0();
    fn EINT1();
    fn GPAB_INT();
    fn GPCDE_INT();
    fn PWMA_INT();
    fn PWMB_INT();
    fn TMR0_INT();
    fn TMR1_INT();
    fn TMR2_INT();
    fn TMR3_INT();
    fn UART02_INT();
    fn UART1_INT();
    fn SPI0_INT();
    fn SPI1_INT();
    fn SPI2_INT();
    fn SPI3_INT();
    fn I2C0_INT();
    fn I2C1_INT();
    fn CAN0_INT();
    fn USB_INT();
    fn PS2_INT();
    fn ACMP_INT();
    fn PDMA_INT();
    fn I2S_INT();
    fn PWRWU_INT();
    fn ADC_INT();
    fn RTC_INT();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: BOD_OUT },
    Vector { _handler: WDT_INT },
    Vector { _handler: EINT0 },
    Vector { _handler: EINT1 },
    Vector { _handler: GPAB_INT },
    Vector {
        _handler: GPCDE_INT,
    },
    Vector { _handler: PWMA_INT },
    Vector { _handler: PWMB_INT },
    Vector { _handler: TMR0_INT },
    Vector { _handler: TMR1_INT },
    Vector { _handler: TMR2_INT },
    Vector { _handler: TMR3_INT },
    Vector {
        _handler: UART02_INT,
    },
    Vector {
        _handler: UART1_INT,
    },
    Vector { _handler: SPI0_INT },
    Vector { _handler: SPI1_INT },
    Vector { _handler: SPI2_INT },
    Vector { _handler: SPI3_INT },
    Vector { _handler: I2C0_INT },
    Vector { _handler: I2C1_INT },
    Vector { _handler: CAN0_INT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USB_INT },
    Vector { _handler: PS2_INT },
    Vector { _handler: ACMP_INT },
    Vector { _handler: PDMA_INT },
    Vector { _handler: I2S_INT },
    Vector {
        _handler: PWRWU_INT,
    },
    Vector { _handler: ADC_INT },
    Vector { _reserved: 0 },
    Vector { _handler: RTC_INT },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - BOD_OUT"]
    BOD_OUT = 0,
    #[doc = "1 - WDT_INT"]
    WDT_INT = 1,
    #[doc = "2 - EINT0"]
    EINT0 = 2,
    #[doc = "3 - EINT1"]
    EINT1 = 3,
    #[doc = "4 - GPAB_INT"]
    GPAB_INT = 4,
    #[doc = "5 - GPCDE_INT"]
    GPCDE_INT = 5,
    #[doc = "6 - PWMA_INT"]
    PWMA_INT = 6,
    #[doc = "7 - PWMB_INT"]
    PWMB_INT = 7,
    #[doc = "8 - TMR0_INT"]
    TMR0_INT = 8,
    #[doc = "9 - TMR1_INT"]
    TMR1_INT = 9,
    #[doc = "10 - TMR2_INT"]
    TMR2_INT = 10,
    #[doc = "11 - TMR3_INT"]
    TMR3_INT = 11,
    #[doc = "12 - UART02_INT"]
    UART02_INT = 12,
    #[doc = "13 - UART1_INT"]
    UART1_INT = 13,
    #[doc = "14 - SPI0_INT"]
    SPI0_INT = 14,
    #[doc = "15 - SPI1_INT"]
    SPI1_INT = 15,
    #[doc = "16 - SPI2_INT"]
    SPI2_INT = 16,
    #[doc = "17 - SPI3_INT"]
    SPI3_INT = 17,
    #[doc = "18 - I2C0_INT"]
    I2C0_INT = 18,
    #[doc = "19 - I2C1_INT"]
    I2C1_INT = 19,
    #[doc = "20 - CAN0_INT"]
    CAN0_INT = 20,
    #[doc = "23 - USB_INT"]
    USB_INT = 23,
    #[doc = "24 - PS2_INT"]
    PS2_INT = 24,
    #[doc = "25 - ACMP_INT"]
    ACMP_INT = 25,
    #[doc = "26 - PDMA_INT"]
    PDMA_INT = 26,
    #[doc = "27 - I2S_INT"]
    I2S_INT = 27,
    #[doc = "28 - PWRWU_INT"]
    PWRWU_INT = 28,
    #[doc = "29 - ADC_INT"]
    ADC_INT = 29,
    #[doc = "31 - RTC_INT"]
    RTC_INT = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Registers group"]
pub struct PWMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWMA {}
impl PWMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwma::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for PWMA {
    type Target = pwma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWMA::ptr() }
    }
}
#[doc = "Registers group"]
pub mod pwma;
#[doc = "Registers group"]
pub struct PWMB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWMB {}
impl PWMB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwma::RegisterBlock {
        0x4014_0000 as *const _
    }
}
impl Deref for PWMB {
    type Target = pwma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWMB::ptr() }
    }
}
#[doc = "Registers group"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x400e_0000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Registers group"]
pub mod adc;
#[doc = "Registers group"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can::RegisterBlock {
        0x4018_0000 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Registers group"]
pub mod can;
#[doc = "Registers group"]
pub struct CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLK {}
impl CLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clk::RegisterBlock {
        0x5000_0200 as *const _
    }
}
impl Deref for CLK {
    type Target = clk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLK::ptr() }
    }
}
#[doc = "Registers group"]
pub mod clk;
#[doc = "Registers group"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp::RegisterBlock {
        0x400d_0000 as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "Registers group"]
pub mod cmp;
#[doc = "Registers group"]
pub struct EBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EBI {}
impl EBI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ebi::RegisterBlock {
        0x5001_0000 as *const _
    }
}
impl Deref for EBI {
    type Target = ebi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EBI::ptr() }
    }
}
#[doc = "Registers group"]
pub mod ebi;
#[doc = "Registers group"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Registers group"]
pub mod fmc;
#[doc = "Registers group"]
pub struct GCR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCR {}
impl GCR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gcr::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for GCR {
    type Target = gcr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCR::ptr() }
    }
}
#[doc = "Registers group"]
pub mod gcr;
#[doc = "Registers group"]
pub struct GPA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPA {}
impl GPA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for GPA {
    type Target = gpa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPA::ptr() }
    }
}
#[doc = "Registers group"]
pub mod gpa;
#[doc = "Registers group"]
pub struct GPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPB {}
impl GPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa::RegisterBlock {
        0x5000_4040 as *const _
    }
}
impl Deref for GPB {
    type Target = gpa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPB::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPC {}
impl GPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa::RegisterBlock {
        0x5000_4080 as *const _
    }
}
impl Deref for GPC {
    type Target = gpa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPC::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPD {}
impl GPD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa::RegisterBlock {
        0x5000_40c0 as *const _
    }
}
impl Deref for GPD {
    type Target = gpa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPD::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPE {}
impl GPE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa::RegisterBlock {
        0x5000_4100 as *const _
    }
}
impl Deref for GPE {
    type Target = gpa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPE::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x5000_4180 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "Registers group"]
pub mod gpio;
#[doc = "Registers group"]
pub struct GPA_BITS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPA_BITS {}
impl GPA_BITS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa_bits::RegisterBlock {
        0x5000_4200 as *const _
    }
}
impl Deref for GPA_BITS {
    type Target = gpa_bits::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPA_BITS::ptr() }
    }
}
#[doc = "Registers group"]
pub mod gpa_bits;
#[doc = "Registers group"]
pub struct GPB_BITS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPB_BITS {}
impl GPB_BITS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa_bits::RegisterBlock {
        0x5000_4240 as *const _
    }
}
impl Deref for GPB_BITS {
    type Target = gpa_bits::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPB_BITS::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPC_BITS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPC_BITS {}
impl GPC_BITS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa_bits::RegisterBlock {
        0x5000_4280 as *const _
    }
}
impl Deref for GPC_BITS {
    type Target = gpa_bits::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPC_BITS::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPD_BITS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPD_BITS {}
impl GPD_BITS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa_bits::RegisterBlock {
        0x5000_42c0 as *const _
    }
}
impl Deref for GPD_BITS {
    type Target = gpa_bits::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPD_BITS::ptr() }
    }
}
#[doc = "Registers group"]
pub struct GPE_BITS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPE_BITS {}
impl GPE_BITS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpa_bits::RegisterBlock {
        0x5000_4300 as *const _
    }
}
impl Deref for GPE_BITS {
    type Target = gpa_bits::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPE_BITS::ptr() }
    }
}
#[doc = "Registers group"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod i2c0;
#[doc = "Registers group"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4012_0000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Registers group"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x401a_0000 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "Registers group"]
pub mod i2s;
#[doc = "Registers group"]
pub struct INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INT {}
impl INT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const int::RegisterBlock {
        0x5000_0300 as *const _
    }
}
impl Deref for INT {
    type Target = int::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*INT::ptr() }
    }
}
#[doc = "Registers group"]
pub mod int;
#[doc = "Registers group"]
pub struct PDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA0 {}
impl PDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for PDMA0 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod pdma0;
#[doc = "Registers group"]
pub struct PDMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA1 {}
impl PDMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8100 as *const _
    }
}
impl Deref for PDMA1 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA1::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA2 {}
impl PDMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8200 as *const _
    }
}
impl Deref for PDMA2 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA2::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA3 {}
impl PDMA3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8300 as *const _
    }
}
impl Deref for PDMA3 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA3::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA4 {}
impl PDMA4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8400 as *const _
    }
}
impl Deref for PDMA4 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA4::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA5 {}
impl PDMA5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8500 as *const _
    }
}
impl Deref for PDMA5 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA5::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA6 {}
impl PDMA6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8600 as *const _
    }
}
impl Deref for PDMA6 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA6::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA7 {}
impl PDMA7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8700 as *const _
    }
}
impl Deref for PDMA7 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA7::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA8 {}
impl PDMA8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma0::RegisterBlock {
        0x5000_8800 as *const _
    }
}
impl Deref for PDMA8 {
    type Target = pdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA8::ptr() }
    }
}
#[doc = "Registers group"]
pub struct PDMA_GCR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA_GCR {}
impl PDMA_GCR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma_gcr::RegisterBlock {
        0x5000_8f00 as *const _
    }
}
impl Deref for PDMA_GCR {
    type Target = pdma_gcr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA_GCR::ptr() }
    }
}
#[doc = "Registers group"]
pub mod pdma_gcr;
#[doc = "Registers group"]
pub struct PS2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PS2 {}
impl PS2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ps2::RegisterBlock {
        0x4010_0000 as *const _
    }
}
impl Deref for PS2 {
    type Target = ps2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PS2::ptr() }
    }
}
#[doc = "Registers group"]
pub mod ps2;
#[doc = "Registers group"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Registers group"]
pub mod rtc;
#[doc = "Registers group"]
pub struct SCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCS {}
impl SCS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scs::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SCS {
    type Target = scs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCS::ptr() }
    }
}
#[doc = "Registers group"]
pub mod scs;
#[doc = "Registers group"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod spi0;
#[doc = "Registers group"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Registers group"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4013_0000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Registers group"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4013_4000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Registers group"]
pub struct TMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR0 {}
impl TMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TMR0 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod tmr0;
#[doc = "Registers group"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4001_0020 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "Registers group"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4011_0000 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "Registers group"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4011_0020 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "Registers group"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod uart0;
#[doc = "Registers group"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4015_0000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Registers group"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4015_4000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Registers group"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Registers group"]
pub mod usb;
#[doc = "Registers group"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Registers group"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PWMA"]
    pub PWMA: PWMA,
    #[doc = "PWMB"]
    pub PWMB: PWMB,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "CLK"]
    pub CLK: CLK,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "EBI"]
    pub EBI: EBI,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "GCR"]
    pub GCR: GCR,
    #[doc = "GPA"]
    pub GPA: GPA,
    #[doc = "GPB"]
    pub GPB: GPB,
    #[doc = "GPC"]
    pub GPC: GPC,
    #[doc = "GPD"]
    pub GPD: GPD,
    #[doc = "GPE"]
    pub GPE: GPE,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPA_BITS"]
    pub GPA_BITS: GPA_BITS,
    #[doc = "GPB_BITS"]
    pub GPB_BITS: GPB_BITS,
    #[doc = "GPC_BITS"]
    pub GPC_BITS: GPC_BITS,
    #[doc = "GPD_BITS"]
    pub GPD_BITS: GPD_BITS,
    #[doc = "GPE_BITS"]
    pub GPE_BITS: GPE_BITS,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "INT"]
    pub INT: INT,
    #[doc = "PDMA0"]
    pub PDMA0: PDMA0,
    #[doc = "PDMA1"]
    pub PDMA1: PDMA1,
    #[doc = "PDMA2"]
    pub PDMA2: PDMA2,
    #[doc = "PDMA3"]
    pub PDMA3: PDMA3,
    #[doc = "PDMA4"]
    pub PDMA4: PDMA4,
    #[doc = "PDMA5"]
    pub PDMA5: PDMA5,
    #[doc = "PDMA6"]
    pub PDMA6: PDMA6,
    #[doc = "PDMA7"]
    pub PDMA7: PDMA7,
    #[doc = "PDMA8"]
    pub PDMA8: PDMA8,
    #[doc = "PDMA_GCR"]
    pub PDMA_GCR: PDMA_GCR,
    #[doc = "PS2"]
    pub PS2: PS2,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SCS"]
    pub SCS: SCS,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "TMR0"]
    pub TMR0: TMR0,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PWMA: PWMA {
                _marker: PhantomData,
            },
            PWMB: PWMB {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CAN: CAN {
                _marker: PhantomData,
            },
            CLK: CLK {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            EBI: EBI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            GCR: GCR {
                _marker: PhantomData,
            },
            GPA: GPA {
                _marker: PhantomData,
            },
            GPB: GPB {
                _marker: PhantomData,
            },
            GPC: GPC {
                _marker: PhantomData,
            },
            GPD: GPD {
                _marker: PhantomData,
            },
            GPE: GPE {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPA_BITS: GPA_BITS {
                _marker: PhantomData,
            },
            GPB_BITS: GPB_BITS {
                _marker: PhantomData,
            },
            GPC_BITS: GPC_BITS {
                _marker: PhantomData,
            },
            GPD_BITS: GPD_BITS {
                _marker: PhantomData,
            },
            GPE_BITS: GPE_BITS {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            INT: INT {
                _marker: PhantomData,
            },
            PDMA0: PDMA0 {
                _marker: PhantomData,
            },
            PDMA1: PDMA1 {
                _marker: PhantomData,
            },
            PDMA2: PDMA2 {
                _marker: PhantomData,
            },
            PDMA3: PDMA3 {
                _marker: PhantomData,
            },
            PDMA4: PDMA4 {
                _marker: PhantomData,
            },
            PDMA5: PDMA5 {
                _marker: PhantomData,
            },
            PDMA6: PDMA6 {
                _marker: PhantomData,
            },
            PDMA7: PDMA7 {
                _marker: PhantomData,
            },
            PDMA8: PDMA8 {
                _marker: PhantomData,
            },
            PDMA_GCR: PDMA_GCR {
                _marker: PhantomData,
            },
            PS2: PS2 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SCS: SCS {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            TMR0: TMR0 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
