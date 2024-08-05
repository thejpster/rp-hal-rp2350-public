#[doc = "Register `MTIMEH` reader"]
pub type R = crate::R<MTIMEH_SPEC>;
#[doc = "Register `MTIMEH` writer"]
pub type W = crate::W<MTIMEH_SPEC>;
#[doc = "Field `MTIMEH` reader - "]
pub type MTIMEH_R = crate::FieldReader<u32>;
#[doc = "Field `MTIMEH` writer - "]
pub type MTIMEH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mtimeh(&self) -> MTIMEH_R {
        MTIMEH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mtimeh(&mut self) -> MTIMEH_W<MTIMEH_SPEC> {
        MTIMEH_W::new(self, 0)
    }
}
#[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.  

You can [`read`](crate::Reg::read) this register and get [`mtimeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMEH_SPEC;
impl crate::RegisterSpec for MTIMEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimeh::R`](R) reader structure"]
impl crate::Readable for MTIMEH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimeh::W`](W) writer structure"]
impl crate::Writable for MTIMEH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIMEH to value 0"]
impl crate::Resettable for MTIMEH_SPEC {
    const RESET_VALUE: u32 = 0;
}
