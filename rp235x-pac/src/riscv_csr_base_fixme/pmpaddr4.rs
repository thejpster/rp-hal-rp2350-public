#[doc = "Register `PMPADDR4` reader"]
pub type R = crate::R<PMPADDR4_SPEC>;
#[doc = "Register `PMPADDR4` writer"]
pub type W = crate::W<PMPADDR4_SPEC>;
#[doc = "Field `PMPADDR4` reader - "]
pub type PMPADDR4_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR4` writer - "]
pub type PMPADDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr4(&self) -> PMPADDR4_R {
        PMPADDR4_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr4(&mut self) -> PMPADDR4_W<PMPADDR4_SPEC> {
        PMPADDR4_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 4. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR4_SPEC;
impl crate::RegisterSpec for PMPADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr4::R`](R) reader structure"]
impl crate::Readable for PMPADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr4::W`](W) writer structure"]
impl crate::Writable for PMPADDR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR4 to value 0"]
impl crate::Resettable for PMPADDR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
