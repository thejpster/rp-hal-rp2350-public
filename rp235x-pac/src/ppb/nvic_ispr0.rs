#[doc = "Register `NVIC_ISPR0` reader"]
pub type R = crate::R<NVIC_ISPR0_SPEC>;
#[doc = "Register `NVIC_ISPR0` writer"]
pub type W = crate::W<NVIC_ISPR0_SPEC>;
#[doc = "Field `SETPEND` reader - For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
pub type SETPEND_R = crate::FieldReader<u32>;
#[doc = "Field `SETPEND` writer - For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
pub type SETPEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<NVIC_ISPR0_SPEC> {
        SETPEND_W::new(self, 0)
    }
}
#[doc = "Enables or reads the pending state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_ispr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ispr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISPR0_SPEC;
impl crate::RegisterSpec for NVIC_ISPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr0::R`](R) reader structure"]
impl crate::Readable for NVIC_ISPR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr0::W`](W) writer structure"]
impl crate::Writable for NVIC_ISPR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR0 to value 0"]
impl crate::Resettable for NVIC_ISPR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
