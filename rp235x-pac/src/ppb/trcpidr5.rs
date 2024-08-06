#[doc = "Register `TRCPIDR5` reader"]
pub type R = crate::R<TRCPIDR5_SPEC>;
#[doc = "Register `TRCPIDR5` writer"]
pub type W = crate::W<TRCPIDR5_SPEC>;
#[doc = "Field `TRCPIDR5` reader - "]
pub type TRCPIDR5_R = crate::FieldReader<u32>;
#[doc = "Field `TRCPIDR5` writer - "]
pub type TRCPIDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trcpidr5(&self) -> TRCPIDR5_R {
        TRCPIDR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn trcpidr5(&mut self) -> TRCPIDR5_W<TRCPIDR5_SPEC> {
        TRCPIDR5_W::new(self, 0)
    }
}
#[doc = "TRCPIDR5  

You can [`read`](crate::Reg::read) this register and get [`trcpidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR5_SPEC;
impl crate::RegisterSpec for TRCPIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr5::R`](R) reader structure"]
impl crate::Readable for TRCPIDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr5::W`](W) writer structure"]
impl crate::Writable for TRCPIDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR5 to value 0"]
impl crate::Resettable for TRCPIDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
