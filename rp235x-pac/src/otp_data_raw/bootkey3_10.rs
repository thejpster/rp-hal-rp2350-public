#[doc = "Register `BOOTKEY3_10` reader"]
pub type R = crate::R<BOOTKEY3_10_SPEC>;
#[doc = "Register `BOOTKEY3_10` writer"]
pub type W = crate::W<BOOTKEY3_10_SPEC>;
#[doc = "Field `BOOTKEY3_10` reader - "]
pub type BOOTKEY3_10_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn bootkey3_10(&self) -> BOOTKEY3_10_R {
        BOOTKEY3_10_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_10_SPEC;
impl crate::RegisterSpec for BOOTKEY3_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey3_10::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootkey3_10::W`](W) writer structure"]
impl crate::Writable for BOOTKEY3_10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTKEY3_10 to value 0"]
impl crate::Resettable for BOOTKEY3_10_SPEC {
    const RESET_VALUE: u32 = 0;
}
