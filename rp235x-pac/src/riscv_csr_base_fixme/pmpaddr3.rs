#[doc = "Register `PMPADDR3` reader"]
pub type R = crate::R<PMPADDR3_SPEC>;
#[doc = "Register `PMPADDR3` writer"]
pub type W = crate::W<PMPADDR3_SPEC>;
#[doc = "Field `PMPADDR3` reader - "]
pub type PMPADDR3_R = crate::FieldReader<u32>;
#[doc = "Field `PMPADDR3` writer - "]
pub type PMPADDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr3(&self) -> PMPADDR3_R {
        PMPADDR3_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn pmpaddr3(&mut self) -> PMPADDR3_W<PMPADDR3_SPEC> {
        PMPADDR3_W::new(self, 0)
    }
}
#[doc = "Physical memory protection address for region 3. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR3_SPEC;
impl crate::RegisterSpec for PMPADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr3::R`](R) reader structure"]
impl crate::Readable for PMPADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpaddr3::W`](W) writer structure"]
impl crate::Writable for PMPADDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPADDR3 to value 0"]
impl crate::Resettable for PMPADDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
