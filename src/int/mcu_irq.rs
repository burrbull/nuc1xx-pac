#[doc = "Reader of register MCU_IRQ"]
pub type R = crate::R<u32, super::MCU_IRQ>;
#[doc = "Writer for register MCU_IRQ"]
pub type W = crate::W<u32, super::MCU_IRQ>;
#[doc = "Register MCU_IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_IRQ`"]
pub type MCU_IRQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCU_IRQ`"]
pub struct MCU_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MCU IRQ Source Register The MCU_IRQ collect all the interrupts from the peripherals and generate the synchronous interrupt to MCU Cortex-M0. There are two modes to generate interrupt to MCU Cortex-M0, the normal mode and test mode. The MCU_IRQ collect all interrupt from each peripheral and synchronize them then interrupt the Cortex-M0. When the MCU_IRQ\\[n\\]
is \"0\": Set MCU_IRQ\\[n\\]
\"1\" will generate a interrupt to Cortex_M0 NVIC\\[n\\]. When the MCU_IRQ\\[n\\]
is \"1\" (mean a interrupt is assert) set \"1\" to the MCU_bit\\[n\\]
will clear the interrupt. Set MCU_IRQ\\[n\\]
\"0\": no any effect"]
    #[inline(always)]
    pub fn mcu_irq(&self) -> MCU_IRQ_R {
        MCU_IRQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MCU IRQ Source Register The MCU_IRQ collect all the interrupts from the peripherals and generate the synchronous interrupt to MCU Cortex-M0. There are two modes to generate interrupt to MCU Cortex-M0, the normal mode and test mode. The MCU_IRQ collect all interrupt from each peripheral and synchronize them then interrupt the Cortex-M0. When the MCU_IRQ\\[n\\]
is \"0\": Set MCU_IRQ\\[n\\]
\"1\" will generate a interrupt to Cortex_M0 NVIC\\[n\\]. When the MCU_IRQ\\[n\\]
is \"1\" (mean a interrupt is assert) set \"1\" to the MCU_bit\\[n\\]
will clear the interrupt. Set MCU_IRQ\\[n\\]
\"0\": no any effect"]
    #[inline(always)]
    pub fn mcu_irq(&mut self) -> MCU_IRQ_W {
        MCU_IRQ_W { w: self }
    }
}
