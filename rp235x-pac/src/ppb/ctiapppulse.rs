#[doc = "Register `CTIAPPPULSE` reader"]
pub type R = crate::R<CTIAPPPULSE_SPEC>;
#[doc = "Register `CTIAPPPULSE` writer"]
pub type W = crate::W<CTIAPPPULSE_SPEC>;
#[doc = "Field `APPULSE` reader - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type APPULSE_R = crate::FieldReader;
#[doc = "Field `APPULSE` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type APPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse(&self) -> APPULSE_R {
        APPULSE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    #[must_use]
    pub fn appulse(&mut self) -> APPULSE_W<CTIAPPPULSE_SPEC> {
        APPULSE_W::new(self, 0)
    }
}
#[doc = "CTI Application Pulse Register  

You can [`read`](crate::Reg::read) this register and get [`ctiapppulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiapppulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIAPPPULSE_SPEC;
impl crate::RegisterSpec for CTIAPPPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiapppulse::R`](R) reader structure"]
impl crate::Readable for CTIAPPPULSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiapppulse::W`](W) writer structure"]
impl crate::Writable for CTIAPPPULSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIAPPPULSE to value 0"]
impl crate::Resettable for CTIAPPPULSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
