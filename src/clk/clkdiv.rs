#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCLK_N`"]
pub type HCLK_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCLK_N`"]
pub struct HCLK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `USB_N`"]
pub type USB_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USB_N`"]
pub struct USB_N_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART_N`"]
pub type UART_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_N`"]
pub struct UART_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAN_N_L`"]
pub type CAN_N_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAN_N_L`"]
pub struct CAN_N_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_N_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC_N`"]
pub type ADC_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_N`"]
pub struct ADC_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CAN_N_H`"]
pub type CAN_N_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAN_N_H`"]
pub struct CAN_N_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_N_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - HCLK clock divide number from HCLK clock source The HCLK clock frequency = (HCLK clock source frequency) / (HCLK_N + 1)"]
    #[inline(always)]
    pub fn hclk_n(&self) -> HCLK_N_R {
        HCLK_N_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - USB clock divide number from PLL clock The USB clock frequency = (PLL frequency ) / (USB_N + 1)"]
    #[inline(always)]
    pub fn usb_n(&self) -> USB_N_R {
        USB_N_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - UART clock divide number from UART clock source The UART clock frequency = (UART clock source frequency ) / (UART_N + 1)"]
    #[inline(always)]
    pub fn uart_n(&self) -> UART_N_R {
        UART_N_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CAN clock divide number from CAN clock source The CAN clock frequency = (CAN clock source frequency ) / (CAN_N + 1) Which CAN_N = 16 * CAN_N_H + CAN_N_L"]
    #[inline(always)]
    pub fn can_n_l(&self) -> CAN_N_L_R {
        CAN_N_L_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - ADC clock divide number from ADC clock source The ADC clock frequency = (ADC engine clock source frequency ) / (ADC_N + 1)"]
    #[inline(always)]
    pub fn adc_n(&self) -> ADC_N_R {
        ADC_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - CAN clock divide number from CAN clock source (Low Density Only) The CAN clock frequency = (CAN clock source frequency ) / (CAN_N + 1) Which CAN_N = 16 * CAN_N_H + CAN_N_L"]
    #[inline(always)]
    pub fn can_n_h(&self) -> CAN_N_H_R {
        CAN_N_H_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - HCLK clock divide number from HCLK clock source The HCLK clock frequency = (HCLK clock source frequency) / (HCLK_N + 1)"]
    #[inline(always)]
    pub fn hclk_n(&mut self) -> HCLK_N_W {
        HCLK_N_W { w: self }
    }
    #[doc = "Bits 4:7 - USB clock divide number from PLL clock The USB clock frequency = (PLL frequency ) / (USB_N + 1)"]
    #[inline(always)]
    pub fn usb_n(&mut self) -> USB_N_W {
        USB_N_W { w: self }
    }
    #[doc = "Bits 8:11 - UART clock divide number from UART clock source The UART clock frequency = (UART clock source frequency ) / (UART_N + 1)"]
    #[inline(always)]
    pub fn uart_n(&mut self) -> UART_N_W {
        UART_N_W { w: self }
    }
    #[doc = "Bits 12:15 - CAN clock divide number from CAN clock source The CAN clock frequency = (CAN clock source frequency ) / (CAN_N + 1) Which CAN_N = 16 * CAN_N_H + CAN_N_L"]
    #[inline(always)]
    pub fn can_n_l(&mut self) -> CAN_N_L_W {
        CAN_N_L_W { w: self }
    }
    #[doc = "Bits 16:23 - ADC clock divide number from ADC clock source The ADC clock frequency = (ADC engine clock source frequency ) / (ADC_N + 1)"]
    #[inline(always)]
    pub fn adc_n(&mut self) -> ADC_N_W {
        ADC_N_W { w: self }
    }
    #[doc = "Bits 24:29 - CAN clock divide number from CAN clock source (Low Density Only) The CAN clock frequency = (CAN clock source frequency ) / (CAN_N + 1) Which CAN_N = 16 * CAN_N_H + CAN_N_L"]
    #[inline(always)]
    pub fn can_n_h(&mut self) -> CAN_N_H_W {
        CAN_N_H_W { w: self }
    }
}
