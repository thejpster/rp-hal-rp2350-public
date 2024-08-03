#[doc = "Register `CTIAPPSET` reader"]
pub type R = crate::R<CTIAPPSET_SPEC>;
#[doc = "Register `CTIAPPSET` writer"]
pub type W = crate::W<CTIAPPSET_SPEC>;
#[doc = "Field `APPSET` reader - Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel"]
pub type APPSET_R = crate::FieldReader;
#[doc = "Field `APPSET` writer - Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel"]
pub type APPSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel"]
    #[inline(always)]
    pub fn appset(&self) -> APPSET_R {
        APPSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Setting a bit HIGH generates a channel event for the selected channel. There is one bit of the register for each channel"]
    #[inline(always)]
    #[must_use]
    pub fn appset(&mut self) -> APPSET_W<CTIAPPSET_SPEC> {
        APPSET_W::new(self, 0)
    }
}
#[doc = "CTI Application Trigger Set Register  

You can [`read`](crate::Reg::read) this register and get [`ctiappset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIAPPSET_SPEC;
impl crate::RegisterSpec for CTIAPPSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiappset::R`](R) reader structure"]
impl crate::Readable for CTIAPPSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiappset::W`](W) writer structure"]
impl crate::Writable for CTIAPPSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIAPPSET to value 0"]
impl crate::Resettable for CTIAPPSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
