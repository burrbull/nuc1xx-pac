#[doc = "Reader of register OFFD"]
pub type R = crate::R<u32, super::OFFD>;
#[doc = "Writer for register OFFD"]
pub type W = crate::W<u32, super::OFFD>;
#[doc = "Register OFFD `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin%s is disabled. If input is analog signal, users can OFF digital input path to avoid creepage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFD_A {
    #[doc = "0: Enable IO digital input path"]
    ENABLE = 0,
    #[doc = "1: Disable IO digital input path (digital input tied to low)"]
    DISABLE = 1,
}
impl From<OFFD_A> for bool {
    #[inline(always)]
    fn from(variant: OFFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFD[%s]`"]
pub type OFFD_R = crate::R<bool, OFFD_A>;
impl OFFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFD_A {
        match self.bits {
            false => OFFD_A::ENABLE,
            true => OFFD_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OFFD_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OFFD_A::DISABLE
    }
}
#[doc = "Write proxy for fields `OFFD(0-15)`"]
pub struct OFFD_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> OFFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO digital input path"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OFFD_A::ENABLE)
    }
    #[doc = "Disable IO digital input path (digital input tied to low)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OFFD_A::DISABLE)
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
OFF digital input path Enable. Control if the input path of Pin%s is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub unsafe fn offd(&self, n: usize) -> OFFD_R {
        OFFD_R::new(((self.bits >> n + 16) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin0 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd0(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin1 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd1(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin2 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd2(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin3 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd3(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin4 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd4(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin5 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd5(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin6 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd6(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin7 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd7(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin8 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd8(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin9 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd9(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin10 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd10(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin11 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd11(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin12 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd12(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin13 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd13(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin14 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd14(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin15 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd15(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin%s is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub unsafe fn offd(&mut self, n: usize) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin0 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd0(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bit 17 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin1 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd1(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 17,
        }
    }
    #[doc = "Bit 18 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin2 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd2(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 18,
        }
    }
    #[doc = "Bit 19 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin3 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd3(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 19,
        }
    }
    #[doc = "Bit 20 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin4 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd4(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 20,
        }
    }
    #[doc = "Bit 21 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin5 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd5(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 21,
        }
    }
    #[doc = "Bit 22 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin6 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd6(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 22,
        }
    }
    #[doc = "Bit 23 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin7 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd7(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 23,
        }
    }
    #[doc = "Bit 24 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin8 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd8(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 24,
        }
    }
    #[doc = "Bit 25 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin9 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd9(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 25,
        }
    }
    #[doc = "Bit 26 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin10 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd10(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 26,
        }
    }
    #[doc = "Bit 27 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin11 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd11(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 27,
        }
    }
    #[doc = "Bit 28 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin12 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd12(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 28,
        }
    }
    #[doc = "Bit 29 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin13 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd13(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 29,
        }
    }
    #[doc = "Bit 30 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin14 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd14(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 30,
        }
    }
    #[doc = "Bit 31 - GPIOx Pin\\[n\\]
OFF digital input path Enable. Control if the input path of Pin15 is disabled. If input is analog signal, users can OFF digital input path to avoid creepage"]
    #[inline(always)]
    pub fn offd15(&mut self) -> OFFD_W {
        OFFD_W {
            w: self,
            offset: 31,
        }
    }
}
