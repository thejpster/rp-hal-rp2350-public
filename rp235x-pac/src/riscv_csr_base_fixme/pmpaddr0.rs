#[doc = "Register `PMPADDR0` reader"]
pub type R = crate::R<PMPADDR0_SPEC>;
#[doc = "Register `PMPADDR0` writer"]
pub type W = crate::W<PMPADDR0_SPEC>;
#[doc = "Field `PMPADDR0` reader - "]
pub type PMPADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR0` writer - "]
pub type PMPADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr0(&self) -> PMPADDR0_R {
        PMPADDR0_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr0(&mut self) -> PMPADDR0_W<PMPADDR0_SPEC> {
        PMPADDR0_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 0. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR0_SPEC;
impl crate::RegisterSpec for PMPADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr0::R`](R) reader structure"]
impl crate::Readable for PMPADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr0::W`](W) writer structure"]
impl crate::Writable for PMPADDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR0 to value 0"]
impl crate::Resettable for PMPADDR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
