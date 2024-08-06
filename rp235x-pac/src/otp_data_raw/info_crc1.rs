#[doc = "Register `INFO_CRC1` reader"]
pub type R = crate::R<INFO_CRC1_SPEC>;
#[doc = "Register `INFO_CRC1` writer"]
pub type W = crate::W<INFO_CRC1_SPEC>;
#[doc = "Field `INFO_CRC1` reader - "]
pub type INFO_CRC1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn info_crc1(&self) -> INFO_CRC1_R {
        INFO_CRC1_R::new(self.bits)
    }
}
impl W {}
#[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_crc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_CRC1_SPEC;
impl crate::RegisterSpec for INFO_CRC1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`info_crc1::R`](R) reader structure"]
impl crate::Readable for INFO_CRC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`info_crc1::W`](W) writer structure"]
impl crate::Writable for INFO_CRC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INFO_CRC1 to value 0"]
impl crate::Resettable for INFO_CRC1_SPEC {
    const RESET_VALUE: u16 = 0;
}
