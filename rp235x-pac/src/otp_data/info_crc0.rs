#[doc = "Register `INFO_CRC0` reader"]
pub type R = crate::R<INFO_CRC0_SPEC>;
#[doc = "Field `INFO_CRC0` reader - "]
pub type INFO_CRC0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn info_crc0(&self) -> INFO_CRC0_R {
        INFO_CRC0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_CRC0_SPEC;
impl crate::RegisterSpec for INFO_CRC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info_crc0::R`](R) reader structure"]
impl crate::Readable for INFO_CRC0_SPEC {}
#[doc = "`reset()` method sets INFO_CRC0 to value 0"]
impl crate::Resettable for INFO_CRC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
