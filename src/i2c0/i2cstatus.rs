#[doc = "Reader of register I2CSTATUS"]
pub type R = crate::R<u32, super::I2CSTATUS>;
#[doc = "Reader of field `I2CSTATUS`"]
pub type I2CSTATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - I2C Status Register The status register of I2C: The three least significant bits are always 0. The five most significant bits contain the status code. There are 26 possible status codes. When I2STATUS contains F8H, no serial interrupt is requested. All other I2CSTATUS values correspond to defined I2C states. When each of these states is entered, a status interrupt is requested (SI = 1). A valid status code is present in I2CSTATUS one machine cycle after SI is set by hardware and is still present one machine cycle after SI has been reset by software. In addition, states 00H stands for a Bus Error. A Bus Error occurs when a START or STOP condition is present at an illegal position in the formation frame. Example of illegal position are during the serial transfer of an address byte, a data byte or an acknowledge bit."]
    #[inline(always)]
    pub fn i2cstatus(&self) -> I2CSTATUS_R {
        I2CSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
