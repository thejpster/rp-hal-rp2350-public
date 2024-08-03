#[doc = "Register `CTICONTROL` reader"]
pub type R = crate::R<CTICONTROL_SPEC>;
#[doc = "Register `CTICONTROL` writer"]
pub type W = crate::W<CTICONTROL_SPEC>;
#[doc = "Field `GLBEN` reader - Enables or disables the CTI"]
pub type GLBEN_R = crate::BitReader;
#[doc = "Field `GLBEN` writer - Enables or disables the CTI"]
pub type GLBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables or disables the CTI"]
    #[inline(always)]
    pub fn glben(&self) -> GLBEN_R {
        GLBEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the CTI"]
    #[inline(always)]
    #[must_use]
    pub fn glben(&mut self) -> GLBEN_W<CTICONTROL_SPEC> {
        GLBEN_W::new(self, 0)
    }
}
#[doc = "CTI Control Register  

You can [`read`](crate::Reg::read) this register and get [`cticontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cticontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTICONTROL_SPEC;
impl crate::RegisterSpec for CTICONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cticontrol::R`](R) reader structure"]
impl crate::Readable for CTICONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cticontrol::W`](W) writer structure"]
impl crate::Writable for CTICONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTICONTROL to value 0"]
impl crate::Resettable for CTICONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
