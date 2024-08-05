#[doc = "Register `TRCIDR6` reader"]
pub type R = crate::R<TRCIDR6_SPEC>;
#[doc = "Register `TRCIDR6` writer"]
pub type W = crate::W<TRCIDR6_SPEC>;
#[doc = "Field `TRCIDR6` reader - "]
pub type TRCIDR6_R = crate::FieldReader<u32>;
#[doc = "Field `TRCIDR6` writer - "]
pub type TRCIDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trcidr6(&self) -> TRCIDR6_R {
        TRCIDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn trcidr6(&mut self) -> TRCIDR6_W<TRCIDR6_SPEC> {
        TRCIDR6_W::new(self, 0)
    }
}
#[doc = "TRCIDR6  

You can [`read`](crate::Reg::read) this register and get [`trcidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR6_SPEC;
impl crate::RegisterSpec for TRCIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr6::R`](R) reader structure"]
impl crate::Readable for TRCIDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr6::W`](W) writer structure"]
impl crate::Writable for TRCIDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR6 to value 0"]
impl crate::Resettable for TRCIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
