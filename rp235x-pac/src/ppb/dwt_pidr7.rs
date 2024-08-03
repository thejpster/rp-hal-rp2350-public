#[doc = "Register `DWT_PIDR7` reader"]
pub type R = crate::R<DWT_PIDR7_SPEC>;
#[doc = "Register `DWT_PIDR7` writer"]
pub type W = crate::W<DWT_PIDR7_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_PIDR7_SPEC;
impl crate::RegisterSpec for DWT_PIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_pidr7::R`](R) reader structure"]
impl crate::Readable for DWT_PIDR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_pidr7::W`](W) writer structure"]
impl crate::Writable for DWT_PIDR7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_PIDR7 to value 0"]
impl crate::Resettable for DWT_PIDR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
