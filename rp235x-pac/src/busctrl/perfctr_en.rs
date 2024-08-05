#[doc = "Register `PERFCTR_EN` reader"]
pub type R = crate::R<PERFCTR_EN_SPEC>;
#[doc = "Register `PERFCTR_EN` writer"]
pub type W = crate::W<PERFCTR_EN_SPEC>;
#[doc = "Field `PERFCTR_EN` reader - "]
pub type PERFCTR_EN_R = crate::BitReader;
#[doc = "Field `PERFCTR_EN` writer - "]
pub type PERFCTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn perfctr_en(&self) -> PERFCTR_EN_R {
        PERFCTR_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr_en(&mut self) -> PERFCTR_EN_W<PERFCTR_EN_SPEC> {
        PERFCTR_EN_W::new(self, 0)
    }
}
#[doc = "Enable the performance counters. If 0, the performance counters do not increment. This can be used to precisely start/stop event sampling around the profiled section of code. The performance counters are initially disabled, to save energy.  

You can [`read`](crate::Reg::read) this register and get [`perfctr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfctr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFCTR_EN_SPEC;
impl crate::RegisterSpec for PERFCTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfctr_en::R`](R) reader structure"]
impl crate::Readable for PERFCTR_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfctr_en::W`](W) writer structure"]
impl crate::Writable for PERFCTR_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFCTR_EN to value 0"]
impl crate::Resettable for PERFCTR_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
