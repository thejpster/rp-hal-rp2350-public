#[doc = "Register `LOCKED` reader"]
pub type R = crate::R<LOCKED_SPEC>;
#[doc = "Register `LOCKED` writer"]
pub type W = crate::W<LOCKED_SPEC>;
#[doc = "Field `LOCKED` reader - "]
pub type LOCKED_R = crate::BitReader;
#[doc = "Field `LOCKED` writer - "]
pub type LOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn locked(&mut self) -> LOCKED_W<LOCKED_SPEC> {
        LOCKED_W::new(self, 0)
    }
}
#[doc = "Set locked bit to disable write access to timer Once set, cannot be cleared (without a reset)  

You can [`read`](crate::Reg::read) this register and get [`locked::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locked::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKED_SPEC;
impl crate::RegisterSpec for LOCKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`locked::R`](R) reader structure"]
impl crate::Readable for LOCKED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`locked::W`](W) writer structure"]
impl crate::Writable for LOCKED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKED to value 0"]
impl crate::Resettable for LOCKED_SPEC {
    const RESET_VALUE: u32 = 0;
}
