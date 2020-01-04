#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Interrupt De-bounce Control"]
    pub dbncecon: DBNCECON,
}
#[doc = "External Interrupt De-bounce Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbncecon](dbncecon) module"]
pub type DBNCECON = crate::Reg<u32, _DBNCECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBNCECON;
#[doc = "`read()` method returns [dbncecon::R](dbncecon::R) reader structure"]
impl crate::Readable for DBNCECON {}
#[doc = "`write(|w| ..)` method takes [dbncecon::W](dbncecon::W) writer structure"]
impl crate::Writable for DBNCECON {}
#[doc = "External Interrupt De-bounce Control"]
pub mod dbncecon;
