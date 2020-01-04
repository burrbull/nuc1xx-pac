#[doc = "Reader of register WTCR"]
pub type R = crate::R<u32, super::WTCR>;
#[doc = "Writer for register WTCR"]
pub type W = crate::W<u32, super::WTCR>;
#[doc = "Register WTCR `reset()`'s with value 0x0700"]
impl crate::ResetValue for super::WTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700
    }
}
#[doc = "Write proxy for field `WTR`"]
pub struct WTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WTR_W<'a> {
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
#[doc = "Reader of field `WTRE`"]
pub type WTRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTRE`"]
pub struct WTRE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTRE_W<'a> {
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
#[doc = "Reader of field `WTRF`"]
pub type WTRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTRF`"]
pub struct WTRF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTRF_W<'a> {
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
#[doc = "Reader of field `WTIF`"]
pub type WTIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIF`"]
pub struct WTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIF_W<'a> {
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
#[doc = "Reader of field `WTWKE`"]
pub type WTWKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTWKE`"]
pub struct WTWKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTWKE_W<'a> {
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
#[doc = "Reader of field `WTWKF`"]
pub type WTWKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTWKF`"]
pub struct WTWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTWKF_W<'a> {
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
#[doc = "Reader of field `WTIE`"]
pub type WTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIE`"]
pub struct WTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIE_W<'a> {
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
#[doc = "Reader of field `WTE`"]
pub type WTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTE`"]
pub struct WTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `WTIS`"]
pub type WTIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTIS`"]
pub struct WTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Watchdog Timer Reset Enable (write protection bit) Setting this bit will enable the Watchdog timer reset function. 0 = Disable Watchdog timer reset function 1 = Enable Watchdog timer reset function"]
    #[inline(always)]
    pub fn wtre(&self) -> WTRE_R {
        WTRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog Timer Reset Flag When the Watchdog timer initiates a reset, the hardware will set this bit. This flag can be read by software to determine the source of reset. Software is responsible to clear it manually by writing 1 to it. If WTRE is disabled, then the Watchdog timer has no effect on this bit. 0 = Watchdog timer reset did not occur 1 = Watchdog timer reset occurs NOTE: This bit is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn wtrf(&self) -> WTRF_R {
        WTRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer Interrupt Flag If the Watchdog timer interrupt is enabled, then the hardware will set this bit to indicate that the Watchdog timer interrupt has occurred. 0 = Watchdog timer interrupt does not occur 1 = Watchdog timer interrupt occurs NOTE: This bit is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn wtif(&self) -> WTIF_R {
        WTIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog Timer Wakeup Function Enable bit (write protection bit) 0 : Disable Watchdog timer Wakeup CPU function. 1 : Enable the Wakeup function that Watchdog timer timeout can wake up CPU from power-down mode. Note: CHIP can wakeup by WDT only if WDT clock source select RC10K."]
    #[inline(always)]
    pub fn wtwke(&self) -> WTWKE_R {
        WTWKE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog Timer Wakeup Flag If Watchdog timer causes CPU wakes up from power-down mode, this bit will be set to high. It must be cleared by software with a write 1 to this bit. 0 : Watchdog timer does not cause CPU wakeup. 1 : CPU wake up from sleep or power-down mode by Watchdog timeout."]
    #[inline(always)]
    pub fn wtwkf(&self) -> WTWKF_R {
        WTWKF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Timer Interrupt Enable (write protection bit) 0 = Disable the Watchdog timer interrupt 1 = Enable the Watchdog timer interrupt"]
    #[inline(always)]
    pub fn wtie(&self) -> WTIE_R {
        WTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Watchdog Timer Enable (write protection bit) 0 = Disable the Watchdog timer (This action will reset the internal counter) 1 = Enable the Watchdog timer"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Watchdog Timer Interval Select (write protection bit) These three bits select the timeout interval for the Watchdog timer. WTIS Timeout Interval Selection Interrupt Period WTR Timeout Interval (WDT_CLK=12MHz) 000 2^4 * WDT_CLK (2^4 + 1024) * WDT_CLK 1.33 us ~ 86.67 us 001 2^6 * WDT_CLK (2^6 + 1024) * WDT_CLK 5.33 us ~ 90.67 us 010 2^8 * WDT_CLK (2^8 + 1024) * WDT_CLK 21.33 us ~ 106.67 us 011 2^10 * WDT_CLK (2^10 + 1024) * WDT_CLK 85.33 us ~ 170.67 us 100 2^12 * WDT_CLK (2^12 + 1024) * WDT_CLK 341.33 us ~ 426.67 us 101 2^14 * WDT_CLK (2^14 + 1024) * WDT_CLK 1.36 ms ~ 1.45 ms 110 2^16 * WDT_CLK (2^16 + 1024) * WDT_CLK 5.46 ms ~ 5.55 ms 111 2^18 * WDT_CLK (2^18 + 1024) * WDT_CLK 21.84 ms ~ 21.93 ms"]
    #[inline(always)]
    pub fn wtis(&self) -> WTIS_R {
        WTIS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Watchdog Timer (write protection bit) Set this bit will clear the Watchdog timer. 0 = Writing 0 to this bit has no effect 1 = Reset the contents of the Watchdog timer NOTE: This bit will auto clear after few clock cycle"]
    #[inline(always)]
    pub fn wtr(&mut self) -> WTR_W {
        WTR_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Enable (write protection bit) Setting this bit will enable the Watchdog timer reset function. 0 = Disable Watchdog timer reset function 1 = Enable Watchdog timer reset function"]
    #[inline(always)]
    pub fn wtre(&mut self) -> WTRE_W {
        WTRE_W { w: self }
    }
    #[doc = "Bit 2 - Watchdog Timer Reset Flag When the Watchdog timer initiates a reset, the hardware will set this bit. This flag can be read by software to determine the source of reset. Software is responsible to clear it manually by writing 1 to it. If WTRE is disabled, then the Watchdog timer has no effect on this bit. 0 = Watchdog timer reset did not occur 1 = Watchdog timer reset occurs NOTE: This bit is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn wtrf(&mut self) -> WTRF_W {
        WTRF_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog Timer Interrupt Flag If the Watchdog timer interrupt is enabled, then the hardware will set this bit to indicate that the Watchdog timer interrupt has occurred. 0 = Watchdog timer interrupt does not occur 1 = Watchdog timer interrupt occurs NOTE: This bit is cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn wtif(&mut self) -> WTIF_W {
        WTIF_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog Timer Wakeup Function Enable bit (write protection bit) 0 : Disable Watchdog timer Wakeup CPU function. 1 : Enable the Wakeup function that Watchdog timer timeout can wake up CPU from power-down mode. Note: CHIP can wakeup by WDT only if WDT clock source select RC10K."]
    #[inline(always)]
    pub fn wtwke(&mut self) -> WTWKE_W {
        WTWKE_W { w: self }
    }
    #[doc = "Bit 5 - Watchdog Timer Wakeup Flag If Watchdog timer causes CPU wakes up from power-down mode, this bit will be set to high. It must be cleared by software with a write 1 to this bit. 0 : Watchdog timer does not cause CPU wakeup. 1 : CPU wake up from sleep or power-down mode by Watchdog timeout."]
    #[inline(always)]
    pub fn wtwkf(&mut self) -> WTWKF_W {
        WTWKF_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog Timer Interrupt Enable (write protection bit) 0 = Disable the Watchdog timer interrupt 1 = Enable the Watchdog timer interrupt"]
    #[inline(always)]
    pub fn wtie(&mut self) -> WTIE_W {
        WTIE_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog Timer Enable (write protection bit) 0 = Disable the Watchdog timer (This action will reset the internal counter) 1 = Enable the Watchdog timer"]
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W {
        WTE_W { w: self }
    }
    #[doc = "Bits 8:10 - Watchdog Timer Interval Select (write protection bit) These three bits select the timeout interval for the Watchdog timer. WTIS Timeout Interval Selection Interrupt Period WTR Timeout Interval (WDT_CLK=12MHz) 000 2^4 * WDT_CLK (2^4 + 1024) * WDT_CLK 1.33 us ~ 86.67 us 001 2^6 * WDT_CLK (2^6 + 1024) * WDT_CLK 5.33 us ~ 90.67 us 010 2^8 * WDT_CLK (2^8 + 1024) * WDT_CLK 21.33 us ~ 106.67 us 011 2^10 * WDT_CLK (2^10 + 1024) * WDT_CLK 85.33 us ~ 170.67 us 100 2^12 * WDT_CLK (2^12 + 1024) * WDT_CLK 341.33 us ~ 426.67 us 101 2^14 * WDT_CLK (2^14 + 1024) * WDT_CLK 1.36 ms ~ 1.45 ms 110 2^16 * WDT_CLK (2^16 + 1024) * WDT_CLK 5.46 ms ~ 5.55 ms 111 2^18 * WDT_CLK (2^18 + 1024) * WDT_CLK 21.84 ms ~ 21.93 ms"]
    #[inline(always)]
    pub fn wtis(&mut self) -> WTIS_W {
        WTIS_W { w: self }
    }
}
