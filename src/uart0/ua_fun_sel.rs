#[doc = "Reader of register UA_FUN_SEL"]
pub type R = crate::R<u32, super::UA_FUN_SEL>;
#[doc = "Writer for register UA_FUN_SEL"]
pub type W = crate::W<u32, super::UA_FUN_SEL>;
#[doc = "Register UA_FUN_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_FUN_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FUN_SEL`"]
pub type FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUN_SEL`"]
pub struct FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Function Select Enable 00 = UART Function 01 = Enable LIN Function 10 = Enable IrDA Function 11 = Enable RS-485 Function (Low Density Only)"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function Select Enable 00 = UART Function 01 = Enable LIN Function 10 = Enable IrDA Function 11 = Enable RS-485 Function (Low Density Only)"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W {
        FUN_SEL_W { w: self }
    }
}
