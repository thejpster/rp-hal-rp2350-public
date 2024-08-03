#[doc = "Register `PMPADDR1` reader"]
pub type R = crate::R<PMPADDR1_SPEC>;
#[doc = "Register `PMPADDR1` writer"]
pub type W = crate::W<PMPADDR1_SPEC>;
#[doc = "Field `PMPADDR1` reader - "]
pub type PMPADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR1` writer - "]
pub type PMPADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr1(&self) -> PMPADDR1_R {
        PMPADDR1_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr1(&mut self) -> PMPADDR1_W<PMPADDR1_SPEC> {
        PMPADDR1_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 1. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR1_SPEC;
impl crate::RegisterSpec for PMPADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr1::R`](R) reader structure"]
impl crate::Readable for PMPADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr1::W`](W) writer structure"]
impl crate::Writable for PMPADDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR1 to value 0"]
impl crate::Resettable for PMPADDR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
