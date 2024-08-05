#[doc = "Register `PIDR7` reader"]
pub type R = crate::R<PIDR7_SPEC>;
#[doc = "Register `PIDR7` writer"]
pub type W = crate::W<PIDR7_SPEC>;
#[doc = "Field `PIDR7` reader - "]
pub type PIDR7_R = crate::FieldReader<u32>;
#[doc = "Field `PIDR7` writer - "]
pub type PIDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pidr7(&self) -> PIDR7_R {
        PIDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pidr7(&mut self) -> PIDR7_W<PIDR7_SPEC> {
        PIDR7_W::new(self, 0)
    }
}
#[doc = "CoreSight Peripheral ID7  

You can [`read`](crate::Reg::read) this register and get [`pidr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR7_SPEC;
impl crate::RegisterSpec for PIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr7::R`](R) reader structure"]
impl crate::Readable for PIDR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr7::W`](W) writer structure"]
impl crate::Writable for PIDR7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR7 to value 0"]
impl crate::Resettable for PIDR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
