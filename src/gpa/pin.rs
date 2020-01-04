#[doc = "Reader of register PIN"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Reader of field `PIN0`"]
pub type PIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN1`"]
pub type PIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN2`"]
pub type PIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN3`"]
pub type PIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN4`"]
pub type PIN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN5`"]
pub type PIN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN6`"]
pub type PIN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN7`"]
pub type PIN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN8`"]
pub type PIN8_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN9`"]
pub type PIN9_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN10`"]
pub type PIN10_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN11`"]
pub type PIN11_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN12`"]
pub type PIN12_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN13`"]
pub type PIN13_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN14`"]
pub type PIN14_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN15`"]
pub type PIN15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Pin Values Each bit of the register reflects the actual status of the respective GPIO pin If bit is 1, it indicates the corresponding pin status is high, else the pin status is low"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
