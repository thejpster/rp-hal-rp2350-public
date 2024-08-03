#[doc = "Register `PMPADDR7` reader"]
pub type R = crate::R<PMPADDR7_SPEC>;
#[doc = "Register `PMPADDR7` writer"]
pub type W = crate::W<PMPADDR7_SPEC>;
#[doc = "Field `PMPADDR7` reader - "]
pub type PMPADDR7_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR7` writer - "]
pub type PMPADDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr7(&self) -> PMPADDR7_R {
        PMPADDR7_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr7(&mut self) -> PMPADDR7_W<PMPADDR7_SPEC> {
        PMPADDR7_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 7. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR7_SPEC;
impl crate::RegisterSpec for PMPADDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr7::R`](R) reader structure"]
impl crate::Readable for PMPADDR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr7::W`](W) writer structure"]
impl crate::Writable for PMPADDR7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR7 to value 0"]
impl crate::Resettable for PMPADDR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
