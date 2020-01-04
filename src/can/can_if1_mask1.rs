#[doc = "Reader of register CAN_IF1_MASK1"]
pub type R = crate::R<u32, super::CAN_IF1_MASK1>;
#[doc = "Writer for register CAN_IF1_MASK1"]
pub type W = crate::W<u32, super::CAN_IF1_MASK1>;
#[doc = "Register CAN_IF1_MASK1 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CAN_IF1_MASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `Msk_0_15`"]
pub type MSK_0_15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Msk_0_15`"]
pub struct MSK_0_15_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_0_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Identifier Mask 15-0 1 = The corresponding identifier bit is used for acceptance filtering. 0 = The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering."]
    #[inline(always)]
    pub fn msk_0_15(&self) -> MSK_0_15_R {
        MSK_0_15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Identifier Mask 15-0 1 = The corresponding identifier bit is used for acceptance filtering. 0 = The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering."]
    #[inline(always)]
    pub fn msk_0_15(&mut self) -> MSK_0_15_W {
        MSK_0_15_W { w: self }
    }
}
