#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Bus Interface General Control Register"]
    pub ebicon: EBICON,
    #[doc = "0x04 - External Bus Interface Timing Control Register"]
    pub extime: EXTIME,
}
#[doc = "External Bus Interface General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebicon](ebicon) module"]
pub type EBICON = crate::Reg<u32, _EBICON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBICON;
#[doc = "`read()` method returns [ebicon::R](ebicon::R) reader structure"]
impl crate::Readable for EBICON {}
#[doc = "`write(|w| ..)` method takes [ebicon::W](ebicon::W) writer structure"]
impl crate::Writable for EBICON {}
#[doc = "External Bus Interface General Control Register"]
pub mod ebicon;
#[doc = "External Bus Interface Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extime](extime) module"]
pub type EXTIME = crate::Reg<u32, _EXTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIME;
#[doc = "`read()` method returns [extime::R](extime::R) reader structure"]
impl crate::Readable for EXTIME {}
#[doc = "`write(|w| ..)` method takes [extime::W](extime::W) writer structure"]
impl crate::Writable for EXTIME {}
#[doc = "External Bus Interface Timing Control Register"]
pub mod extime;
