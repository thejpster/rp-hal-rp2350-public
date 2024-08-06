#[doc = "Register `MTIME` reader"]
pub type R = crate::R<MTIME_SPEC>;
#[doc = "Register `MTIME` writer"]
pub type W = crate::W<MTIME_SPEC>;
#[doc = "Field `MTIME` reader - "]
pub type MTIME_R = crate::FieldReader<u32>;
#[doc = "Field `MTIME` writer - "]
pub type MTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mtime(&mut self) -> MTIME_W<MTIME_SPEC> {
        MTIME_W::new(self, 0)
    }
}
#[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.  

You can [`read`](crate::Reg::read) this register and get [`mtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtime::W`](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIME to value 0"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
