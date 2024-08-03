#[doc = "Register `PMPADDR6` reader"]
pub type R = crate::R<PMPADDR6_SPEC>;
#[doc = "Register `PMPADDR6` writer"]
pub type W = crate::W<PMPADDR6_SPEC>;
#[doc = "Field `PMPADDR6` reader - "]
pub type PMPADDR6_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR6` writer - "]
pub type PMPADDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr6(&self) -> PMPADDR6_R {
        PMPADDR6_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr6(&mut self) -> PMPADDR6_W<PMPADDR6_SPEC> {
        PMPADDR6_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 6. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR6_SPEC;
impl crate::RegisterSpec for PMPADDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr6::R`](R) reader structure"]
impl crate::Readable for PMPADDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr6::W`](W) writer structure"]
impl crate::Writable for PMPADDR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR6 to value 0"]
impl crate::Resettable for PMPADDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
