#[doc = "Register `SCRATCH5` reader"]
pub type R = crate::R<SCRATCH5_SPEC>;
#[doc = "Register `SCRATCH5` writer"]
pub type W = crate::W<SCRATCH5_SPEC>;
#[doc = "Field `SCRATCH5` reader - "]
pub type SCRATCH5_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH5` writer - "]
pub type SCRATCH5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch5(&self) -> SCRATCH5_R {
        SCRATCH5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn scratch5(&mut self) -> SCRATCH5_W<SCRATCH5_SPEC> {
        SCRATCH5_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH5_SPEC;
impl crate::RegisterSpec for SCRATCH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch5::R`](R) reader structure"]
impl crate::Readable for SCRATCH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch5::W`](W) writer structure"]
impl crate::Writable for SCRATCH5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH5 to value 0"]
impl crate::Resettable for SCRATCH5_SPEC {
    const RESET_VALUE: u32 = 0;
}
