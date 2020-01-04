#[doc = "Reader of register PDID"]
pub type R = crate::R<u32, super::PDID>;
#[doc = "Reader of field `PDID`"]
pub type PDID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Part Device Identification Number This register reflects device part number code. S/W can read this register to identify which device is used."]
    #[inline(always)]
    pub fn pdid(&self) -> PDID_R {
        PDID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
