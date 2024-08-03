#[doc = "Register `FP_PIDR5` reader"]
pub type R = crate::R<FP_PIDR5_SPEC>;
#[doc = "Register `FP_PIDR5` writer"]
pub type W = crate::W<FP_PIDR5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR5_SPEC;
impl crate::RegisterSpec for FP_PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr5::R`](R) reader structure"]
impl crate::Readable for FP_PIDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_pidr5::W`](W) writer structure"]
impl crate::Writable for FP_PIDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_PIDR5 to value 0"]
impl crate::Resettable for FP_PIDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
