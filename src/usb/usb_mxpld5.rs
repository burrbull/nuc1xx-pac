#[doc = "Reader of register USB_MXPLD5"]
pub type R = crate::R<u32, super::USB_MXPLD5>;
#[doc = "Writer for register USB_MXPLD5"]
pub type W = crate::W<u32, super::USB_MXPLD5>;
#[doc = "Register USB_MXPLD5 `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_MXPLD5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MXPLD`"]
pub type MXPLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MXPLD`"]
pub struct MXPLD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXPLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - It is used to define the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted in IN token or received in OUT token. (1). When the register is written by CPU, For IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready. For OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host. (2). When the register is read by CPU, For IN token, the value of MXPLD is indicated the data length be transmitted to host. For OUT token, the value of MXPLD is indicated the actual data length receiving from host. Note that once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
    #[inline(always)]
    pub fn mxpld(&self) -> MXPLD_R {
        MXPLD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - It is used to define the data length which is transmitted to host (IN token) or the actual data length which is received from the host (OUT token). It also used to indicate that the endpoint is ready to be transmitted in IN token or received in OUT token. (1). When the register is written by CPU, For IN token, the value of MXPLD is used to define the data length to be transmitted and indicate the data buffer is ready. For OUT token, it means that the controller is ready to receive data from the host and the value of MXPLD is the maximal data length comes from host. (2). When the register is read by CPU, For IN token, the value of MXPLD is indicated the data length be transmitted to host. For OUT token, the value of MXPLD is indicated the actual data length receiving from host. Note that once MXPLD is written, the data packets will be transmitted/received immediately after IN/OUT token arrived."]
    #[inline(always)]
    pub fn mxpld(&mut self) -> MXPLD_W {
        MXPLD_W { w: self }
    }
}
