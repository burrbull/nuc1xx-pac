#[doc = "Reader of register USB_FLDET"]
pub type R = crate::R<u32, super::USB_FLDET>;
#[doc = "Reader of field `FLDET`"]
pub type FLDET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 1: When the controller is attached into the BUS, this bit will be set as 1 0: The controller didn't attached into the USB host"]
    #[inline(always)]
    pub fn fldet(&self) -> FLDET_R {
        FLDET_R::new((self.bits & 0x01) != 0)
    }
}
