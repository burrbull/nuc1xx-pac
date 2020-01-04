#[doc = "Reader of register DFBADR"]
pub type R = crate::R<u32, super::DFBADR>;
#[doc = "Reader of field `DFBADR`"]
pub type DFBADR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Flash Base Address This register indicates data flash start address. It is a read only register. For 128kB flash memory device, the data flash size is defined by user configuration, register content is loaded from Config1 when chip power on but for 64/32kB device, it is fixed at 0x01_f000"]
    #[inline(always)]
    pub fn dfbadr(&self) -> DFBADR_R {
        DFBADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
