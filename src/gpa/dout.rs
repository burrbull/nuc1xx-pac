#[doc = "Reader of register DOUT"]
pub type R = crate::R<u32, super::DOUT>;
#[doc = "Writer for register DOUT"]
pub type W = crate::W<u32, super::DOUT>;
#[doc = "Register DOUT `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::DOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT_A {
    #[doc = "0: Pin\\[n\\]
will drive Low if the GPIO pin is configures as output, open-drain and quasi-mode"]
    LOW = 0,
    #[doc = "1: Pin\\[n\\]
will drive High if the GPIO pin is configures as output, open-drain and quasi-mode"]
    HIGH = 1,
}
impl From<DOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOUT[%s]`"]
pub type DOUT_R = crate::R<bool, DOUT_A>;
impl DOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_A {
        match self.bits {
            false => DOUT_A::LOW,
            true => DOUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DOUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DOUT_A::HIGH
    }
}
#[doc = "Write proxy for fields `DOUT(0-15)`"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> DOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin\\[n\\]
will drive Low if the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DOUT_A::LOW)
    }
    #[doc = "Pin\\[n\\]
will drive High if the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DOUT_A::HIGH)
    }
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | (((value as u32) & 0x01) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub unsafe fn dout(&self, n: usize) -> DOUT_R {
        DOUT_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout6(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout9(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout10(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout11(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout12(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub unsafe fn dout(&mut self, n: usize) -> DOUT_W {
        DOUT_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout0(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout1(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout2(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout3(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout4(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout6(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout7(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout8(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout9(&mut self) -> DOUT_W {
        DOUT_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout10(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout11(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout12(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout13(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout14(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - GPIOx Pin\\[n\\]
Output Value. Control the status of a GPIO pin when the GPIO pin is configures as output, open-drain and quasi-mode"]
    #[inline(always)]
    pub fn dout15(&mut self) -> DOUT_W {
        DOUT_W {
            w: self,
            offset: 15,
        }
    }
}
