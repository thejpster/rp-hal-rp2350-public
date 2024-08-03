#[doc = "Register `ITM_PIDR6` reader"]
pub type R = crate::R<ITM_PIDR6_SPEC>;
#[doc = "Register `ITM_PIDR6` writer"]
pub type W = crate::W<ITM_PIDR6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_PIDR6_SPEC;
impl crate::RegisterSpec for ITM_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_pidr6::R`](R) reader structure"]
impl crate::Readable for ITM_PIDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_pidr6::W`](W) writer structure"]
impl crate::Writable for ITM_PIDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_PIDR6 to value 0"]
impl crate::Resettable for ITM_PIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}