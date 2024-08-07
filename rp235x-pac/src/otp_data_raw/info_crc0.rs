#[doc = "Register `INFO_CRC0` reader"]
pub type R = crate::R<INFO_CRC0_SPEC>;
#[doc = "Register `INFO_CRC0` writer"]
pub type W = crate::W<INFO_CRC0_SPEC>;
#[doc = "Field `INFO_CRC0` reader - "]
pub type INFO_CRC0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn info_crc0(&self) -> INFO_CRC0_R {
        INFO_CRC0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_crc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_CRC0_SPEC;
impl crate::RegisterSpec for INFO_CRC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info_crc0::R`](R) reader structure"]
impl crate::Readable for INFO_CRC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`info_crc0::W`](W) writer structure"]
impl crate::Writable for INFO_CRC0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFO_CRC0 to value 0"]
impl crate::Resettable for INFO_CRC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
