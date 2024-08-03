#[doc = "Register `PMPADDR2` reader"]
pub type R = crate::R<PMPADDR2_SPEC>;
#[doc = "Register `PMPADDR2` writer"]
pub type W = crate::W<PMPADDR2_SPEC>;
#[doc = "Field `PMPADDR2` reader - "]
pub type PMPADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR2` writer - "]
pub type PMPADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr2(&self) -> PMPADDR2_R {
        PMPADDR2_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr2(&mut self) -> PMPADDR2_W<PMPADDR2_SPEC> {
        PMPADDR2_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 2. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR2_SPEC;
impl crate::RegisterSpec for PMPADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr2::R`](R) reader structure"]
impl crate::Readable for PMPADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr2::W`](W) writer structure"]
impl crate::Writable for PMPADDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR2 to value 0"]
impl crate::Resettable for PMPADDR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
