#[doc = "Reader of register APBCLK"]
pub type R = crate::R<u32, super::APBCLK>;
#[doc = "Writer for register APBCLK"]
pub type W = crate::W<u32, super::APBCLK>;
#[doc = "Register APBCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::APBCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_EN`"]
pub type WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_EN`"]
pub struct WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RTC_EN`"]
pub type RTC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_EN`"]
pub struct RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_EN_W<'a> {
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
#[doc = "Reader of field `TMR0_EN`"]
pub type TMR0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR0_EN`"]
pub struct TMR0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_EN_W<'a> {
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
#[doc = "Reader of field `TMR1_EN`"]
pub type TMR1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR1_EN`"]
pub struct TMR1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_EN_W<'a> {
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
#[doc = "Reader of field `TMR2_EN`"]
pub type TMR2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR2_EN`"]
pub struct TMR2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_EN_W<'a> {
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
#[doc = "Reader of field `TMR3_EN`"]
pub type TMR3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR3_EN`"]
pub struct TMR3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_EN_W<'a> {
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
#[doc = "Reader of field `FDIV_EN`"]
pub type FDIV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDIV_EN`"]
pub struct FDIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2C0_EN`"]
pub type I2C0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_EN`"]
pub struct I2C0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_EN_W<'a> {
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
#[doc = "Reader of field `I2C1_EN`"]
pub type I2C1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_EN`"]
pub struct I2C1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_EN_W<'a> {
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
#[doc = "Reader of field `SPI0_EN`"]
pub type SPI0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_EN`"]
pub struct SPI0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_EN_W<'a> {
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
#[doc = "Reader of field `SPI1_EN`"]
pub type SPI1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1_EN`"]
pub struct SPI1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_EN_W<'a> {
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
#[doc = "Reader of field `SPI2_EN`"]
pub type SPI2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2_EN`"]
pub struct SPI2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_EN_W<'a> {
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
#[doc = "Reader of field `SPI3_EN`"]
pub type SPI3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI3_EN`"]
pub struct SPI3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_EN_W<'a> {
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
#[doc = "Reader of field `UART0_EN`"]
pub type UART0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_EN`"]
pub struct UART0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_EN_W<'a> {
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
#[doc = "Reader of field `UART1_EN`"]
pub type UART1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_EN`"]
pub struct UART1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_EN_W<'a> {
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
#[doc = "Reader of field `UART2_EN`"]
pub type UART2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART2_EN`"]
pub struct UART2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_EN_W<'a> {
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
#[doc = "Reader of field `PWM01_EN`"]
pub type PWM01_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM01_EN`"]
pub struct PWM01_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM01_EN_W<'a> {
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
#[doc = "Reader of field `PWM23_EN`"]
pub type PWM23_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM23_EN`"]
pub struct PWM23_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM23_EN_W<'a> {
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
#[doc = "Reader of field `PWM45_EN`"]
pub type PWM45_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM45_EN`"]
pub struct PWM45_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM45_EN_W<'a> {
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
#[doc = "Reader of field `PWM67_EN`"]
pub type PWM67_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM67_EN`"]
pub struct PWM67_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM67_EN_W<'a> {
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
#[doc = "Reader of field `CAN0_EN`"]
pub type CAN0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0_EN`"]
pub struct CAN0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_EN_W<'a> {
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
#[doc = "Reader of field `USBD_EN`"]
pub type USBD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBD_EN`"]
pub struct USBD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBD_EN_W<'a> {
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
#[doc = "Reader of field `ADC_EN`"]
pub type ADC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_EN`"]
pub struct ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EN_W<'a> {
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
#[doc = "Reader of field `I2S_EN`"]
pub type I2S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_EN`"]
pub struct I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_EN_W<'a> {
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
#[doc = "Reader of field `ACMP_EN`"]
pub type ACMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP_EN`"]
pub struct ACMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PS2_EN`"]
pub type PS2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PS2_EN`"]
pub struct PS2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watch Dog Timer Clock Enable (write-protection bit) This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. The bit default value is set by flash controller. User configuration register config0 bit\\[31\\]
1 = Enable Watchdog Timer Clock 0 = Disable Watchdog Timer Clock"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real-Time-Clock APB interface Clock Enable. This bit is used to control the RTC APB clock only, The RTC engine clock source is from the 32.768KHz crystal. 1 = Enable RTC Clock 0 = Disable RTC Clock"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0 Clock Enable 1 = Enable Timer0 Clock 0 = Disable Timer0 Clock"]
    #[inline(always)]
    pub fn tmr0_en(&self) -> TMR0_EN_R {
        TMR0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer1 Clock Enable 1 = Enable Timer1 Clock 0 = Disable Timer1 Clock"]
    #[inline(always)]
    pub fn tmr1_en(&self) -> TMR1_EN_R {
        TMR1_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer2 Clock Enable 1 = Enable Timer2 Clock 0 = Disable Timer2 Clock"]
    #[inline(always)]
    pub fn tmr2_en(&self) -> TMR2_EN_R {
        TMR2_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer3 Clock Enable 1 = Enable Timer3 Clock 0 = Disable Timer3 Clock"]
    #[inline(always)]
    pub fn tmr3_en(&self) -> TMR3_EN_R {
        TMR3_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Frequency Divider Output Clock Enable 1 = Enable FDIV Clock 0 = Disable FDIV Clock"]
    #[inline(always)]
    pub fn fdiv_en(&self) -> FDIV_EN_R {
        FDIV_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 Clock Enable . 1 = Enable I2C0 Clock 0 = Disable I2C0 Clock"]
    #[inline(always)]
    pub fn i2c0_en(&self) -> I2C0_EN_R {
        I2C0_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C1 Clock Enable. 1 = Enable I2C1 Clock 0 = Disable I2C1 Clock"]
    #[inline(always)]
    pub fn i2c1_en(&self) -> I2C1_EN_R {
        I2C1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 Clock Enable. 1 = Enable SPI0 Clock 0 = Disable SPI0 Clock"]
    #[inline(always)]
    pub fn spi0_en(&self) -> SPI0_EN_R {
        SPI0_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI1 Clock Enable. 1 = Enable SPI1 Clock 0 = Disable SPI1 Clock"]
    #[inline(always)]
    pub fn spi1_en(&self) -> SPI1_EN_R {
        SPI1_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 Clock Enable. (Medium Density Only) 1 = Enable SPI2 Clock 0 = Disable SPI2 Clock"]
    #[inline(always)]
    pub fn spi2_en(&self) -> SPI2_EN_R {
        SPI2_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 Clock Enable. (Medium Density Only) 1 = Enable SPI3 Clock 0 = Disable SPI3 Clock"]
    #[inline(always)]
    pub fn spi3_en(&self) -> SPI3_EN_R {
        SPI3_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Clock Enable. 1 = Enable UART0 clock 0 = Disable UART0 clock"]
    #[inline(always)]
    pub fn uart0_en(&self) -> UART0_EN_R {
        UART0_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 Clock Enable. 1 = Enable UART1 clock 0 = Disable UART1 clock"]
    #[inline(always)]
    pub fn uart1_en(&self) -> UART1_EN_R {
        UART1_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 Clock Enable.(Medium Density Only) 1 = Enable UART2 clock 0 = Disable UART2 clock"]
    #[inline(always)]
    pub fn uart2_en(&self) -> UART2_EN_R {
        UART2_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM_01 Clock Enable. 1 = Enable PWM01 clock 0 = Disable PWM01 clock"]
    #[inline(always)]
    pub fn pwm01_en(&self) -> PWM01_EN_R {
        PWM01_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM_23 Clock Enable. 1 = Enable PWM23 clock 0 = Disable PWM23 clock"]
    #[inline(always)]
    pub fn pwm23_en(&self) -> PWM23_EN_R {
        PWM23_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PWM_45 Clock Enable.(Medium Density Only) 1 = Enable PWM45 clock 0 = Disable PWM45 clock"]
    #[inline(always)]
    pub fn pwm45_en(&self) -> PWM45_EN_R {
        PWM45_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PWM_67 Clock Enable.(Medium Density Only) 1 = Enable PWM67 clock 0 = Disable PWM67 clock"]
    #[inline(always)]
    pub fn pwm67_en(&self) -> PWM67_EN_R {
        PWM67_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CAN Bus Controller-0 Clock Enable 1 = Enable CAN0 clock 0 = Disable CAN0 clock"]
    #[inline(always)]
    pub fn can0_en(&self) -> CAN0_EN_R {
        CAN0_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB 2.0 FS Device Controller Clock Enable 1 = Enable USB clock 0 = Disable USB clock"]
    #[inline(always)]
    pub fn usbd_en(&self) -> USBD_EN_R {
        USBD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Analog-Digital-Converter (ADC) Clock Enable. 1 = Enable ADC clock 0 = Disable ADC clock"]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I2S Clock Enable 1 = Enable I2S Clock 0 = Disable I2S Clock"]
    #[inline(always)]
    pub fn i2s_en(&self) -> I2S_EN_R {
        I2S_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Analog Comparator Clock Enable. 1 = Enable the Analog Comparator Clock 0 = Disable the Analog Comparator Clock"]
    #[inline(always)]
    pub fn acmp_en(&self) -> ACMP_EN_R {
        ACMP_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PS2 Clock Enable. 1 = Enable PS2 clock 0 = Disable PS2 clock"]
    #[inline(always)]
    pub fn ps2_en(&self) -> PS2_EN_R {
        PS2_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watch Dog Timer Clock Enable (write-protection bit) This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. The bit default value is set by flash controller. User configuration register config0 bit\\[31\\]
1 = Enable Watchdog Timer Clock 0 = Disable Watchdog Timer Clock"]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Real-Time-Clock APB interface Clock Enable. This bit is used to control the RTC APB clock only, The RTC engine clock source is from the 32.768KHz crystal. 1 = Enable RTC Clock 0 = Disable RTC Clock"]
    #[inline(always)]
    pub fn rtc_en(&mut self) -> RTC_EN_W {
        RTC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Timer0 Clock Enable 1 = Enable Timer0 Clock 0 = Disable Timer0 Clock"]
    #[inline(always)]
    pub fn tmr0_en(&mut self) -> TMR0_EN_W {
        TMR0_EN_W { w: self }
    }
    #[doc = "Bit 3 - Timer1 Clock Enable 1 = Enable Timer1 Clock 0 = Disable Timer1 Clock"]
    #[inline(always)]
    pub fn tmr1_en(&mut self) -> TMR1_EN_W {
        TMR1_EN_W { w: self }
    }
    #[doc = "Bit 4 - Timer2 Clock Enable 1 = Enable Timer2 Clock 0 = Disable Timer2 Clock"]
    #[inline(always)]
    pub fn tmr2_en(&mut self) -> TMR2_EN_W {
        TMR2_EN_W { w: self }
    }
    #[doc = "Bit 5 - Timer3 Clock Enable 1 = Enable Timer3 Clock 0 = Disable Timer3 Clock"]
    #[inline(always)]
    pub fn tmr3_en(&mut self) -> TMR3_EN_W {
        TMR3_EN_W { w: self }
    }
    #[doc = "Bit 6 - Frequency Divider Output Clock Enable 1 = Enable FDIV Clock 0 = Disable FDIV Clock"]
    #[inline(always)]
    pub fn fdiv_en(&mut self) -> FDIV_EN_W {
        FDIV_EN_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 Clock Enable . 1 = Enable I2C0 Clock 0 = Disable I2C0 Clock"]
    #[inline(always)]
    pub fn i2c0_en(&mut self) -> I2C0_EN_W {
        I2C0_EN_W { w: self }
    }
    #[doc = "Bit 9 - I2C1 Clock Enable. 1 = Enable I2C1 Clock 0 = Disable I2C1 Clock"]
    #[inline(always)]
    pub fn i2c1_en(&mut self) -> I2C1_EN_W {
        I2C1_EN_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 Clock Enable. 1 = Enable SPI0 Clock 0 = Disable SPI0 Clock"]
    #[inline(always)]
    pub fn spi0_en(&mut self) -> SPI0_EN_W {
        SPI0_EN_W { w: self }
    }
    #[doc = "Bit 13 - SPI1 Clock Enable. 1 = Enable SPI1 Clock 0 = Disable SPI1 Clock"]
    #[inline(always)]
    pub fn spi1_en(&mut self) -> SPI1_EN_W {
        SPI1_EN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 Clock Enable. (Medium Density Only) 1 = Enable SPI2 Clock 0 = Disable SPI2 Clock"]
    #[inline(always)]
    pub fn spi2_en(&mut self) -> SPI2_EN_W {
        SPI2_EN_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 Clock Enable. (Medium Density Only) 1 = Enable SPI3 Clock 0 = Disable SPI3 Clock"]
    #[inline(always)]
    pub fn spi3_en(&mut self) -> SPI3_EN_W {
        SPI3_EN_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Clock Enable. 1 = Enable UART0 clock 0 = Disable UART0 clock"]
    #[inline(always)]
    pub fn uart0_en(&mut self) -> UART0_EN_W {
        UART0_EN_W { w: self }
    }
    #[doc = "Bit 17 - UART1 Clock Enable. 1 = Enable UART1 clock 0 = Disable UART1 clock"]
    #[inline(always)]
    pub fn uart1_en(&mut self) -> UART1_EN_W {
        UART1_EN_W { w: self }
    }
    #[doc = "Bit 18 - UART2 Clock Enable.(Medium Density Only) 1 = Enable UART2 clock 0 = Disable UART2 clock"]
    #[inline(always)]
    pub fn uart2_en(&mut self) -> UART2_EN_W {
        UART2_EN_W { w: self }
    }
    #[doc = "Bit 20 - PWM_01 Clock Enable. 1 = Enable PWM01 clock 0 = Disable PWM01 clock"]
    #[inline(always)]
    pub fn pwm01_en(&mut self) -> PWM01_EN_W {
        PWM01_EN_W { w: self }
    }
    #[doc = "Bit 21 - PWM_23 Clock Enable. 1 = Enable PWM23 clock 0 = Disable PWM23 clock"]
    #[inline(always)]
    pub fn pwm23_en(&mut self) -> PWM23_EN_W {
        PWM23_EN_W { w: self }
    }
    #[doc = "Bit 22 - PWM_45 Clock Enable.(Medium Density Only) 1 = Enable PWM45 clock 0 = Disable PWM45 clock"]
    #[inline(always)]
    pub fn pwm45_en(&mut self) -> PWM45_EN_W {
        PWM45_EN_W { w: self }
    }
    #[doc = "Bit 23 - PWM_67 Clock Enable.(Medium Density Only) 1 = Enable PWM67 clock 0 = Disable PWM67 clock"]
    #[inline(always)]
    pub fn pwm67_en(&mut self) -> PWM67_EN_W {
        PWM67_EN_W { w: self }
    }
    #[doc = "Bit 24 - CAN Bus Controller-0 Clock Enable 1 = Enable CAN0 clock 0 = Disable CAN0 clock"]
    #[inline(always)]
    pub fn can0_en(&mut self) -> CAN0_EN_W {
        CAN0_EN_W { w: self }
    }
    #[doc = "Bit 27 - USB 2.0 FS Device Controller Clock Enable 1 = Enable USB clock 0 = Disable USB clock"]
    #[inline(always)]
    pub fn usbd_en(&mut self) -> USBD_EN_W {
        USBD_EN_W { w: self }
    }
    #[doc = "Bit 28 - Analog-Digital-Converter (ADC) Clock Enable. 1 = Enable ADC clock 0 = Disable ADC clock"]
    #[inline(always)]
    pub fn adc_en(&mut self) -> ADC_EN_W {
        ADC_EN_W { w: self }
    }
    #[doc = "Bit 29 - I2S Clock Enable 1 = Enable I2S Clock 0 = Disable I2S Clock"]
    #[inline(always)]
    pub fn i2s_en(&mut self) -> I2S_EN_W {
        I2S_EN_W { w: self }
    }
    #[doc = "Bit 30 - Analog Comparator Clock Enable. 1 = Enable the Analog Comparator Clock 0 = Disable the Analog Comparator Clock"]
    #[inline(always)]
    pub fn acmp_en(&mut self) -> ACMP_EN_W {
        ACMP_EN_W { w: self }
    }
    #[doc = "Bit 31 - PS2 Clock Enable. 1 = Enable PS2 clock 0 = Disable PS2 clock"]
    #[inline(always)]
    pub fn ps2_en(&mut self) -> PS2_EN_W {
        PS2_EN_W { w: self }
    }
}
