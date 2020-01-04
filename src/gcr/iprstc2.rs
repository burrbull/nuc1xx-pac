#[doc = "Reader of register IPRSTC2"]
pub type R = crate::R<u32, super::IPRSTC2>;
#[doc = "Writer for register IPRSTC2"]
pub type W = crate::W<u32, super::IPRSTC2>;
#[doc = "Register IPRSTC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPRSTC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_RST`"]
pub type GPIO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_RST`"]
pub struct GPIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TMR0_RST`"]
pub type TMR0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR0_RST`"]
pub struct TMR0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TMR1_RST`"]
pub type TMR1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR1_RST`"]
pub struct TMR1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TMR2_RST`"]
pub type TMR2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR2_RST`"]
pub struct TMR2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TMR3_RST`"]
pub type TMR3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR3_RST`"]
pub struct TMR3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2C0_RST`"]
pub type I2C0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_RST`"]
pub struct I2C0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C1_RST`"]
pub type I2C1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_RST`"]
pub struct I2C1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPI0_RST`"]
pub type SPI0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_RST`"]
pub struct SPI0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI1_RST`"]
pub type SPI1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_RST`"]
pub struct SPI1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPI2_RST`"]
pub type SPI2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2_RST`"]
pub struct SPI2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI3_RST`"]
pub type SPI3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI3_RST`"]
pub struct SPI3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UART0_RST`"]
pub type UART0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_RST`"]
pub struct UART0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART1_RST`"]
pub type UART1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_RST`"]
pub struct UART1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `UART2_RST`"]
pub type UART2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART2_RST`"]
pub struct UART2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PWM03_RST`"]
pub type PWM03_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM03_RST`"]
pub struct PWM03_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM03_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PWM47_RST`"]
pub type PWM47_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM47_RST`"]
pub struct PWM47_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM47_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ACMP_RST`"]
pub type ACMP_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP_RST`"]
pub struct ACMP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PS2_RST`"]
pub type PS2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PS2_RST`"]
pub struct PS2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CAN0_RST`"]
pub type CAN0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0_RST`"]
pub struct CAN0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USBD_RST`"]
pub type USBD_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBD_RST`"]
pub struct USBD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBD_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ADC_RST`"]
pub type ADC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_RST`"]
pub struct ADC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2S_RST`"]
pub type I2S_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RST`"]
pub struct I2S_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - GPIO controller Reset 1 = GPIO controller reset 0 = GPIO controller normal operation"]
    #[inline(always)]
    pub fn gpio_rst(&self) -> GPIO_RST_R {
        GPIO_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0 controller Reset 1 = Timer0 controller reset 0 = Timer0 controller normal operation"]
    #[inline(always)]
    pub fn tmr0_rst(&self) -> TMR0_RST_R {
        TMR0_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer1 controller Reset 1 = Timer1 controller reset 0 = Timer1 controller normal operation"]
    #[inline(always)]
    pub fn tmr1_rst(&self) -> TMR1_RST_R {
        TMR1_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer2 controller Reset 1 = Timer2 controller reset 0 = Timer2 controller normal operation"]
    #[inline(always)]
    pub fn tmr2_rst(&self) -> TMR2_RST_R {
        TMR2_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer3 controller Reset 1 = Timer3 controller reset 0 = Timer3 controller normal operation"]
    #[inline(always)]
    pub fn tmr3_rst(&self) -> TMR3_RST_R {
        TMR3_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 controller Reset 1 = I2C0 controller reset 0 = I2C0 controller normal operation"]
    #[inline(always)]
    pub fn i2c0_rst(&self) -> I2C0_RST_R {
        I2C0_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C1 controller Reset 1 = I2C1 controller reset 0 = I2C1 controller normal operation"]
    #[inline(always)]
    pub fn i2c1_rst(&self) -> I2C1_RST_R {
        I2C1_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 controller Reset 1 = SPI0 controller reset 0 = SPI0 controller normal operation"]
    #[inline(always)]
    pub fn spi0_rst(&self) -> SPI0_RST_R {
        SPI0_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI1 controller Reset 1 = SPI1 controller reset 0 = SPI1 controller normal operation"]
    #[inline(always)]
    pub fn spi1_rst(&self) -> SPI1_RST_R {
        SPI1_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 controller Reset (Medium Density Only) 1 = SPI2 controller reset 0 = SPI2 controller normal operation"]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 controller Reset (Medium Density Only) 1 = SPI3 controller reset 0 = SPI3 controller normal operation"]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 controller Reset 1 = UART0 controller reset 0 = UART0 controller normal operation"]
    #[inline(always)]
    pub fn uart0_rst(&self) -> UART0_RST_R {
        UART0_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 controller Reset 1 = UART1 controller reset 0 = UART1 controller normal operation"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 controller Reset (Medium Density Only) 1 = UART2 controller reset 0 = UART2 controller normal operation"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM03 controller Reset 1 = PWM03 controller reset 0 = PWM03 controller normal operation"]
    #[inline(always)]
    pub fn pwm03_rst(&self) -> PWM03_RST_R {
        PWM03_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM47 controller Reset (Medium Density Only) 1 = PWM47 controller reset 0 = PWM47 controller normal operation"]
    #[inline(always)]
    pub fn pwm47_rst(&self) -> PWM47_RST_R {
        PWM47_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Analog Comparator Controller Reset 1 = Analog Comparator controller reset 0 = Analog Comparator controller normal operation"]
    #[inline(always)]
    pub fn acmp_rst(&self) -> ACMP_RST_R {
        ACMP_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PS2 Controller Reset 1 = PS2 controller reset 0 = PS2 controller normal operation"]
    #[inline(always)]
    pub fn ps2_rst(&self) -> PS2_RST_R {
        PS2_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CAN0 Controller Reset 1 = CAN0 controller reset 0 = CAN0 controller normal operation"]
    #[inline(always)]
    pub fn can0_rst(&self) -> CAN0_RST_R {
        CAN0_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB Device Controller Reset 1 = USB device controller reset 0 = USB devide controller normal operation"]
    #[inline(always)]
    pub fn usbd_rst(&self) -> USBD_RST_R {
        USBD_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC Controller Reset 1 = ADC controller reset 0 = ADC controller normal operation"]
    #[inline(always)]
    pub fn adc_rst(&self) -> ADC_RST_R {
        ADC_RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I2S Controller Reset 1 = I2S controller reset 0 = I2S controller normal operation"]
    #[inline(always)]
    pub fn i2s_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - GPIO controller Reset 1 = GPIO controller reset 0 = GPIO controller normal operation"]
    #[inline(always)]
    pub fn gpio_rst(&mut self) -> GPIO_RST_W {
        GPIO_RST_W { w: self }
    }
    #[doc = "Bit 2 - Timer0 controller Reset 1 = Timer0 controller reset 0 = Timer0 controller normal operation"]
    #[inline(always)]
    pub fn tmr0_rst(&mut self) -> TMR0_RST_W {
        TMR0_RST_W { w: self }
    }
    #[doc = "Bit 3 - Timer1 controller Reset 1 = Timer1 controller reset 0 = Timer1 controller normal operation"]
    #[inline(always)]
    pub fn tmr1_rst(&mut self) -> TMR1_RST_W {
        TMR1_RST_W { w: self }
    }
    #[doc = "Bit 4 - Timer2 controller Reset 1 = Timer2 controller reset 0 = Timer2 controller normal operation"]
    #[inline(always)]
    pub fn tmr2_rst(&mut self) -> TMR2_RST_W {
        TMR2_RST_W { w: self }
    }
    #[doc = "Bit 5 - Timer3 controller Reset 1 = Timer3 controller reset 0 = Timer3 controller normal operation"]
    #[inline(always)]
    pub fn tmr3_rst(&mut self) -> TMR3_RST_W {
        TMR3_RST_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 controller Reset 1 = I2C0 controller reset 0 = I2C0 controller normal operation"]
    #[inline(always)]
    pub fn i2c0_rst(&mut self) -> I2C0_RST_W {
        I2C0_RST_W { w: self }
    }
    #[doc = "Bit 9 - I2C1 controller Reset 1 = I2C1 controller reset 0 = I2C1 controller normal operation"]
    #[inline(always)]
    pub fn i2c1_rst(&mut self) -> I2C1_RST_W {
        I2C1_RST_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 controller Reset 1 = SPI0 controller reset 0 = SPI0 controller normal operation"]
    #[inline(always)]
    pub fn spi0_rst(&mut self) -> SPI0_RST_W {
        SPI0_RST_W { w: self }
    }
    #[doc = "Bit 13 - SPI1 controller Reset 1 = SPI1 controller reset 0 = SPI1 controller normal operation"]
    #[inline(always)]
    pub fn spi1_rst(&mut self) -> SPI1_RST_W {
        SPI1_RST_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 controller Reset (Medium Density Only) 1 = SPI2 controller reset 0 = SPI2 controller normal operation"]
    #[inline(always)]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W {
        SPI2_RST_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 controller Reset (Medium Density Only) 1 = SPI3 controller reset 0 = SPI3 controller normal operation"]
    #[inline(always)]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W {
        SPI3_RST_W { w: self }
    }
    #[doc = "Bit 16 - UART0 controller Reset 1 = UART0 controller reset 0 = UART0 controller normal operation"]
    #[inline(always)]
    pub fn uart0_rst(&mut self) -> UART0_RST_W {
        UART0_RST_W { w: self }
    }
    #[doc = "Bit 17 - UART1 controller Reset 1 = UART1 controller reset 0 = UART1 controller normal operation"]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART1_RST_W {
        UART1_RST_W { w: self }
    }
    #[doc = "Bit 18 - UART2 controller Reset (Medium Density Only) 1 = UART2 controller reset 0 = UART2 controller normal operation"]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> UART2_RST_W {
        UART2_RST_W { w: self }
    }
    #[doc = "Bit 20 - PWM03 controller Reset 1 = PWM03 controller reset 0 = PWM03 controller normal operation"]
    #[inline(always)]
    pub fn pwm03_rst(&mut self) -> PWM03_RST_W {
        PWM03_RST_W { w: self }
    }
    #[doc = "Bit 21 - PWM47 controller Reset (Medium Density Only) 1 = PWM47 controller reset 0 = PWM47 controller normal operation"]
    #[inline(always)]
    pub fn pwm47_rst(&mut self) -> PWM47_RST_W {
        PWM47_RST_W { w: self }
    }
    #[doc = "Bit 22 - Analog Comparator Controller Reset 1 = Analog Comparator controller reset 0 = Analog Comparator controller normal operation"]
    #[inline(always)]
    pub fn acmp_rst(&mut self) -> ACMP_RST_W {
        ACMP_RST_W { w: self }
    }
    #[doc = "Bit 23 - PS2 Controller Reset 1 = PS2 controller reset 0 = PS2 controller normal operation"]
    #[inline(always)]
    pub fn ps2_rst(&mut self) -> PS2_RST_W {
        PS2_RST_W { w: self }
    }
    #[doc = "Bit 24 - CAN0 Controller Reset 1 = CAN0 controller reset 0 = CAN0 controller normal operation"]
    #[inline(always)]
    pub fn can0_rst(&mut self) -> CAN0_RST_W {
        CAN0_RST_W { w: self }
    }
    #[doc = "Bit 27 - USB Device Controller Reset 1 = USB device controller reset 0 = USB devide controller normal operation"]
    #[inline(always)]
    pub fn usbd_rst(&mut self) -> USBD_RST_W {
        USBD_RST_W { w: self }
    }
    #[doc = "Bit 28 - ADC Controller Reset 1 = ADC controller reset 0 = ADC controller normal operation"]
    #[inline(always)]
    pub fn adc_rst(&mut self) -> ADC_RST_W {
        ADC_RST_W { w: self }
    }
    #[doc = "Bit 29 - I2S Controller Reset 1 = I2S controller reset 0 = I2S controller normal operation"]
    #[inline(always)]
    pub fn i2s_rst(&mut self) -> I2S_RST_W {
        I2S_RST_W { w: self }
    }
}
