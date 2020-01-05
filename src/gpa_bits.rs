#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Pin I/O Bit Output Control"]
    pub dout: [DOUT; 16],
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
