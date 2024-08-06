#[doc = "Register `PIDR5` reader"]
pub type R = crate::R<PIDR5_SPEC>;
#[doc = "Register `PIDR5` writer"]
pub type W = crate::W<PIDR5_SPEC>;
#[doc = "Field `PIDR5` reader - "]
pub type PIDR5_R = crate::FieldReader<u32>;
#[doc = "Field `PIDR5` writer - "]
pub type PIDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pidr5(&mut self) -> PIDR5_W<PIDR5_SPEC> {
        PIDR5_W::new(self, 0)
    }
}
#[doc = "CoreSight Peripheral ID5  

You can [`read`](crate::Reg::read) this register and get [`pidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR5_SPEC;
impl crate::RegisterSpec for PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr5::R`](R) reader structure"]
impl crate::Readable for PIDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr5::W`](W) writer structure"]
impl crate::Writable for PIDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR5 to value 0"]
impl crate::Resettable for PIDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
