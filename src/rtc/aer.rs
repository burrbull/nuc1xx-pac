#[doc = "Reader of register AER"]
pub type R = crate::R<u32, super::AER>;
#[doc = "Writer for register AER"]
pub type W = crate::W<u32, super::AER>;
#[doc = "Register AER `reset()`'s with value 0"]
impl crate::ResetValue for super::AER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AER`"]
pub struct AER_W<'a> {
    w: &'a mut W,
}
impl<'a> AER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENF`"]
pub type ENF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - RTC Register Access Enable Flag (Read only) 1 = RTC register read/write enable 0 = RTC register read/write disable This bit will be set after AER\\[15:0\\]
register is load a 0xA965, and be clear automatically 512 RTC clock or AER\\[15:0\\]
is not 0xA965.Register\\AER.ENF 1 0 INIR R/W R/W AER R/W R/W FCR R/W - TLR R/W R CLR R/W R TSSR R/W R/W DWR R/W R TAR R/W - CAR R/W - LIR R R RIER R/W R/W RIIR R/C R/C TTR R/W -"]
    #[inline(always)]
    pub fn enf(&self) -> ENF_R {
        ENF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Register Access Enable Password (Write only) 0xA965 = Enable RTC access Others = Disable RTC access"]
    #[inline(always)]
    pub fn aer(&mut self) -> AER_W {
        AER_W { w: self }
    }
}
