#[doc = "Register `BOOT1` reader"]
pub type R = crate::R<BOOT1_SPEC>;
#[doc = "Register `BOOT1` writer"]
pub type W = crate::W<BOOT1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT1_SPEC;
impl crate::RegisterSpec for BOOT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot1::R`](R) reader structure"]
impl crate::Readable for BOOT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot1::W`](W) writer structure"]
impl crate::Writable for BOOT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT1 to value 0"]
impl crate::Resettable for BOOT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
