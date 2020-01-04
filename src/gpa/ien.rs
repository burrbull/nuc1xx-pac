#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF_EN_A {
    #[doc = "0: Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    ENABLE = 1,
}
impl From<IF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IF_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IF_EN[%s]`"]
pub type IF_EN_R = crate::R<bool, IF_EN_A>;
impl IF_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF_EN_A {
        match self.bits {
            false => IF_EN_A::DISABLE,
            true => IF_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IF_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IF_EN_A::ENABLE
    }
}
#[doc = "Write proxy for fields `IF_EN(0-15)`"]
pub struct IF_EN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> IF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IF_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IF_EN_A::DISABLE)
    }
    #[doc = "Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IF_EN_A::ENABLE)
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
#[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IR_EN_A {
    #[doc = "0: Disable the PIN\\[n\\]
level-high or low-to-high interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable the PIN\\[n\\]
level-high or low-to-high interrupt"]
    ENABLE = 1,
}
impl From<IR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IR_EN[%s]`"]
pub type IR_EN_R = crate::R<bool, IR_EN_A>;
impl IR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IR_EN_A {
        match self.bits {
            false => IR_EN_A::DISABLE,
            true => IR_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IR_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IR_EN_A::ENABLE
    }
}
#[doc = "Write proxy for fields `IR_EN(0-15)`"]
pub struct IR_EN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> IR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the PIN\\[n\\]
level-high or low-to-high interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IR_EN_A::DISABLE)
    }
    #[doc = "Enable the PIN\\[n\\]
level-high or low-to-high interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IR_EN_A::ENABLE)
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
    #[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub unsafe fn if_en(&self, n: usize) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en0(&self) -> IF_EN_R {
        IF_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en1(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en2(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en3(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en4(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en5(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en6(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en7(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en8(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en9(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en10(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en11(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en12(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en13(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en14(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en15(&self) -> IF_EN_R {
        IF_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub unsafe fn ir_en(&self, n: usize) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> n + 16) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en0(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en1(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en2(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en3(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en4(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en5(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en6(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en7(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en8(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en9(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en10(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en11(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en12(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en13(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en14(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en15(&self) -> IR_EN_R {
        IR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub unsafe fn if_en(&mut self, n: usize) -> IF_EN_W {
        IF_EN_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en0(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en1(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en2(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en3(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en4(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en5(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en6(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en7(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en8(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en9(&mut self) -> IF_EN_W {
        IF_EN_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en10(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en11(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en12(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en13(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en14(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt"]
    #[inline(always)]
    pub fn if_en15(&mut self) -> IF_EN_W {
        IF_EN_W {
            w: self,
            offset: 15,
        }
    }
    #[doc = "Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub unsafe fn ir_en(&mut self, n: usize) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en0(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bit 17 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en1(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 17,
        }
    }
    #[doc = "Bit 18 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en2(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 18,
        }
    }
    #[doc = "Bit 19 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en3(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 19,
        }
    }
    #[doc = "Bit 20 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en4(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 20,
        }
    }
    #[doc = "Bit 21 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en5(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 21,
        }
    }
    #[doc = "Bit 22 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en6(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 22,
        }
    }
    #[doc = "Bit 23 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en7(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 23,
        }
    }
    #[doc = "Bit 24 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en8(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 24,
        }
    }
    #[doc = "Bit 25 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en9(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 25,
        }
    }
    #[doc = "Bit 26 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en10(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 26,
        }
    }
    #[doc = "Bit 27 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en11(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 27,
        }
    }
    #[doc = "Bit 28 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en12(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 28,
        }
    }
    #[doc = "Bit 29 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en13(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 29,
        }
    }
    #[doc = "Bit 30 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en14(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 30,
        }
    }
    #[doc = "Bit 31 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt"]
    #[inline(always)]
    pub fn ir_en15(&mut self) -> IR_EN_W {
        IR_EN_W {
            w: self,
            offset: 31,
        }
    }
}
