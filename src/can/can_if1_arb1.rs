#[doc = "Reader of register CAN_IF1_ARB1"]
pub type R = crate::R<u32, super::CAN_IF1_ARB1>;
#[doc = "Writer for register CAN_IF1_ARB1"]
pub type W = crate::W<u32, super::CAN_IF1_ARB1>;
#[doc = "Register CAN_IF1_ARB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF1_ARB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_0_15`"]
pub type ID_0_15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ID_0_15`"]
pub struct ID_0_15_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_0_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Message Identifier 15-0 ID28 - ID0, 29-bit Identifier (\"Extended Frame\"). ID28 - ID18, 11-bit Identifier (\"Standard Frame\")"]
    #[inline(always)]
    pub fn id_0_15(&self) -> ID_0_15_R {
        ID_0_15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Identifier 15-0 ID28 - ID0, 29-bit Identifier (\"Extended Frame\"). ID28 - ID18, 11-bit Identifier (\"Standard Frame\")"]
    #[inline(always)]
    pub fn id_0_15(&mut self) -> ID_0_15_W {
        ID_0_15_W { w: self }
    }
}
