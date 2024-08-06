#[doc = "Register `BOOTKEY3_8` reader"]
pub type R = crate::R<BOOTKEY3_8_SPEC>;
#[doc = "Register `BOOTKEY3_8` writer"]
pub type W = crate::W<BOOTKEY3_8_SPEC>;
#[doc = "Field `BOOTKEY3_8` reader - "]
pub type BOOTKEY3_8_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey3_8(&self) -> BOOTKEY3_8_R {
        BOOTKEY3_8_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_8_SPEC;
impl crate::RegisterSpec for BOOTKEY3_8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey3_8::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootkey3_8::W`](W) writer structure"]
impl crate::Writable for BOOTKEY3_8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BOOTKEY3_8 to value 0"]
impl crate::Resettable for BOOTKEY3_8_SPEC {
    const RESET_VALUE: u16 = 0;
}
