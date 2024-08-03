#[doc = "Register `PIDR6` reader"]
pub type R = crate::R<PIDR6_SPEC>;
#[doc = "Register `PIDR6` writer"]
pub type W = crate::W<PIDR6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CoreSight Periperal ID6  

You can [`read`](crate::Reg::read) this register and get [`pidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR6_SPEC;
impl crate::RegisterSpec for PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr6::R`](R) reader structure"]
impl crate::Readable for PIDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr6::W`](W) writer structure"]
impl crate::Writable for PIDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR6 to value 0"]
impl crate::Resettable for PIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
