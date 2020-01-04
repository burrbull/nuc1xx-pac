#[doc = "Reader of register PMD"]
pub type R = crate::R<u32, super::PMD>;
#[doc = "Writer for register PMD"]
pub type W = crate::W<u32, super::PMD>;
#[doc = "Register PMD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMD_A {
    #[doc = "0: Pin\\[n\\]
is in INPUT mode"]
    INPUT = 0,
    #[doc = "1: Pin\\[n\\]
is in OUTPUT mode"]
    OUTPUT = 1,
    #[doc = "2: Pin\\[n\\]
is in Open-Drain mode"]
    OPEN_DRAIN = 2,
    #[doc = "3: Pin\\[n\\]
is in Quasi-bidirectional mode"]
    QUASI_BIDIRECTIONAL = 3,
}
impl From<PMD_A> for u8 {
    #[inline(always)]
    fn from(variant: PMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PMD[%s]`"]
pub type PMD_R = crate::R<u8, PMD_A>;
impl PMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMD_A {
        match self.bits {
            0 => PMD_A::INPUT,
            1 => PMD_A::OUTPUT,
            2 => PMD_A::OPEN_DRAIN,
            3 => PMD_A::QUASI_BIDIRECTIONAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PMD_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PMD_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PMD_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `QUASI_BIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_quasi_bidirectional(&self) -> bool {
        *self == PMD_A::QUASI_BIDIRECTIONAL
    }
}
#[doc = "Write proxy for fields `PMD(0-15)`"]
pub struct PMD_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin\\[n\\]
is in INPUT mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PMD_A::INPUT)
    }
    #[doc = "Pin\\[n\\]
is in OUTPUT mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PMD_A::OUTPUT)
    }
    #[doc = "Pin\\[n\\]
is in Open-Drain mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(PMD_A::OPEN_DRAIN)
    }
    #[doc = "Pin\\[n\\]
is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn quasi_bidirectional(self) -> &'a mut W {
        self.variant(PMD_A::QUASI_BIDIRECTIONAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << self.offset)) | (((value as u32) & 0x03) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub unsafe fn pmd(&self, n: usize) -> PMD_R {
        PMD_R::new(((self.bits >> n * 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd0(&self) -> PMD_R {
        PMD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd1(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd2(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd3(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd4(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd5(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd6(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd7(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd8(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd9(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd10(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd11(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd12(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd13(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd14(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd15(&self) -> PMD_R {
        PMD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub unsafe fn pmd(&mut self, n: usize) -> PMD_W {
        PMD_W {
            w: self,
            offset: n * 2,
        }
    }
    #[doc = "Bits 0:1 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd0(&mut self) -> PMD_W {
        PMD_W { w: self, offset: 0 }
    }
    #[doc = "Bits 2:3 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd1(&mut self) -> PMD_W {
        PMD_W { w: self, offset: 2 }
    }
    #[doc = "Bits 4:5 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd2(&mut self) -> PMD_W {
        PMD_W { w: self, offset: 4 }
    }
    #[doc = "Bits 6:7 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd3(&mut self) -> PMD_W {
        PMD_W { w: self, offset: 6 }
    }
    #[doc = "Bits 8:9 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd4(&mut self) -> PMD_W {
        PMD_W { w: self, offset: 8 }
    }
    #[doc = "Bits 10:11 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd5(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bits 12:13 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd6(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bits 14:15 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd7(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bits 16:17 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd8(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bits 18:19 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd9(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 18,
        }
    }
    #[doc = "Bits 20:21 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd10(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 20,
        }
    }
    #[doc = "Bits 22:23 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd11(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 22,
        }
    }
    #[doc = "Bits 24:25 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd12(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 24,
        }
    }
    #[doc = "Bits 26:27 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd13(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 26,
        }
    }
    #[doc = "Bits 28:29 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd14(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 28,
        }
    }
    #[doc = "Bits 30:31 - GPIOX I/O Pin\\[n\\]
Mode Control. Determine each I/O type of GPIOx pins"]
    #[inline(always)]
    pub fn pmd15(&mut self) -> PMD_W {
        PMD_W {
            w: self,
            offset: 30,
        }
    }
}
