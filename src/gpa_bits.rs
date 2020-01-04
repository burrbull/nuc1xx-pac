#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dout0: [u8; 19usize],
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout0(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DOUT) }
    }
    #[doc = "0x00 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout0_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DOUT) }
    }
    #[doc = "0x01 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout1(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(1usize) as *const DOUT) }
    }
    #[doc = "0x01 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout1_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1usize) as *mut DOUT) }
    }
    #[doc = "0x02 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout2(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const DOUT) }
    }
    #[doc = "0x02 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout2_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut DOUT) }
    }
    #[doc = "0x03 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout3(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(3usize) as *const DOUT) }
    }
    #[doc = "0x03 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout3_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3usize) as *mut DOUT) }
    }
    #[doc = "0x04 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout4(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DOUT) }
    }
    #[doc = "0x04 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout4_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut DOUT) }
    }
    #[doc = "0x05 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout5(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(5usize) as *const DOUT) }
    }
    #[doc = "0x05 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout5_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5usize) as *mut DOUT) }
    }
    #[doc = "0x06 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout6(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const DOUT) }
    }
    #[doc = "0x06 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout6_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut DOUT) }
    }
    #[doc = "0x07 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout7(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(7usize) as *const DOUT) }
    }
    #[doc = "0x07 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout7_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(7usize) as *mut DOUT) }
    }
    #[doc = "0x08 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout8(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const DOUT) }
    }
    #[doc = "0x08 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout8_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut DOUT) }
    }
    #[doc = "0x09 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout9(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(9usize) as *const DOUT) }
    }
    #[doc = "0x09 - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout9_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(9usize) as *mut DOUT) }
    }
    #[doc = "0x0a - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout10(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const DOUT) }
    }
    #[doc = "0x0a - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout10_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut DOUT) }
    }
    #[doc = "0x0b - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout11(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(11usize) as *const DOUT) }
    }
    #[doc = "0x0b - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout11_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(11usize) as *mut DOUT) }
    }
    #[doc = "0x0c - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout12(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const DOUT) }
    }
    #[doc = "0x0c - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout12_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut DOUT) }
    }
    #[doc = "0x0d - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout13(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(13usize) as *const DOUT) }
    }
    #[doc = "0x0d - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout13_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(13usize) as *mut DOUT) }
    }
    #[doc = "0x0e - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout14(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const DOUT) }
    }
    #[doc = "0x0e - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout14_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut DOUT) }
    }
    #[doc = "0x0f - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout15(&self) -> &DOUT {
        unsafe { &*(((self as *const Self) as *const u8).add(15usize) as *const DOUT) }
    }
    #[doc = "0x0f - GPIO Port Pin I/O Bit Output Control"]
    #[inline(always)]
    pub fn dout15_mut(&self) -> &mut DOUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(15usize) as *mut DOUT) }
    }
}
#[doc = "GPIO Port Pin I/O Bit Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout](dout) module"]
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
#[doc = "`read()` method returns [dout::R](dout::R) reader structure"]
impl crate::Readable for DOUT {}
#[doc = "`write(|w| ..)` method takes [dout::W](dout::W) writer structure"]
impl crate::Writable for DOUT {}
#[doc = "GPIO Port Pin I/O Bit Output Control"]
pub mod dout;
