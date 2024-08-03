#[doc = "Register `MPU_RNR` reader"]
pub type R = crate::R<MPU_RNR_SPEC>;
#[doc = "Register `MPU_RNR` writer"]
pub type W = crate::W<MPU_RNR_SPEC>;
#[doc = "Field `REGION` reader - Indicates the memory region accessed by MPU_RBAR and MPU_RLAR"]
pub type REGION_R = crate::FieldReader;
#[doc = "Field `REGION` writer - Indicates the memory region accessed by MPU_RBAR and MPU_RLAR"]
pub type REGION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Indicates the memory region accessed by MPU_RBAR and MPU_RLAR"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Indicates the memory region accessed by MPU_RBAR and MPU_RLAR"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<MPU_RNR_SPEC> {
        REGION_W::new(self, 0)
    }
}
#[doc = "Selects the region currently accessed by MPU_RBAR and MPU_RLAR  

You can [`read`](crate::Reg::read) this register and get [`mpu_rnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RNR_SPEC;
impl crate::RegisterSpec for MPU_RNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rnr::R`](R) reader structure"]
impl crate::Readable for MPU_RNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rnr::W`](W) writer structure"]
impl crate::Writable for MPU_RNR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RNR to value 0"]
impl crate::Resettable for MPU_RNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
