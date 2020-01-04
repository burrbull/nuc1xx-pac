#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Reader of field `REVESION`"]
pub type REVESION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PARTNO`"]
pub type PARTNO_R = crate::R<u16, u16>;
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u8, u8>;
#[doc = "Reader of field `IMPLEMENTER`"]
pub type IMPLEMENTER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Reads as 0x0"]
    #[inline(always)]
    pub fn revesion(&self) -> REVESION_R {
        REVESION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reads as 0xC20"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Reads as 0xC for ARMv6-M parts"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code assigned by ARM. ( ARM = 0x41)"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
