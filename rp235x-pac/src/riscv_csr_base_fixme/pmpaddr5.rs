#[doc = "Register `PMPADDR5` reader"]
pub type R = crate::R<PMPADDR5_SPEC>;
#[doc = "Register `PMPADDR5` writer"]
pub type W = crate::W<PMPADDR5_SPEC>;
#[doc = "Field `PMPADDR5` reader - "]
pub type PMPADDR5_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR5` writer - "]
pub type PMPADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr5(&self) -> PMPADDR5_R {
        PMPADDR5_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr5(&mut self) -> PMPADDR5_W<PMPADDR5_SPEC> {
        PMPADDR5_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 5. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR5_SPEC;
impl crate::RegisterSpec for PMPADDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr5::R`](R) reader structure"]
impl crate::Readable for PMPADDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr5::W`](W) writer structure"]
impl crate::Writable for PMPADDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR5 to value 0"]
impl crate::Resettable for PMPADDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
