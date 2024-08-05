#[doc = "Register `WRITE_ONCE0` reader"]
pub type R = crate::R<WRITE_ONCE0_SPEC>;
#[doc = "Register `WRITE_ONCE0` writer"]
pub type W = crate::W<WRITE_ONCE0_SPEC>;
#[doc = "Field `WRITE_ONCE0` reader - "]
pub type WRITE_ONCE0_R = crate::FieldReader<u32>;
#[doc = "Field `WRITE_ONCE0` writer - "]
pub type WRITE_ONCE0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn write_once0(&self) -> WRITE_ONCE0_R {
        WRITE_ONCE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn write_once0(&mut self) -> WRITE_ONCE0_W<WRITE_ONCE0_SPEC> {
        WRITE_ONCE0_W::new(self, 0)
    }
}
#[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset.  

You can [`read`](crate::Reg::read) this register and get [`write_once0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_once0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITE_ONCE0_SPEC;
impl crate::RegisterSpec for WRITE_ONCE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_once0::R`](R) reader structure"]
impl crate::Readable for WRITE_ONCE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`write_once0::W`](W) writer structure"]
impl crate::Writable for WRITE_ONCE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITE_ONCE0 to value 0"]
impl crate::Resettable for WRITE_ONCE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
