#[doc = "Register `FP_PIDR6` reader"]
pub type R = crate::R<FP_PIDR6_SPEC>;
#[doc = "Register `FP_PIDR6` writer"]
pub type W = crate::W<FP_PIDR6_SPEC>;
#[doc = "Field `FP_PIDR6` reader - "]
pub type FP_PIDR6_R = crate::FieldReader<u32>;
#[doc = "Field `FP_PIDR6` writer - "]
pub type FP_PIDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fp_pidr6(&self) -> FP_PIDR6_R {
        FP_PIDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fp_pidr6(&mut self) -> FP_PIDR6_W<FP_PIDR6_SPEC> {
        FP_PIDR6_W::new(self, 0)
    }
}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR6_SPEC;
impl crate::RegisterSpec for FP_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr6::R`](R) reader structure"]
impl crate::Readable for FP_PIDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_pidr6::W`](W) writer structure"]
impl crate::Writable for FP_PIDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_PIDR6 to value 0"]
impl crate::Resettable for FP_PIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
