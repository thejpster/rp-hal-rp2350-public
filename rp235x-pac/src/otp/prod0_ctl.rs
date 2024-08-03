#[doc = "Register `PROD0_CTL` reader"]
pub type R = crate::R<PROD0_CTL_SPEC>;
#[doc = "Register `PROD0_CTL` writer"]
pub type W = crate::W<PROD0_CTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Controls availale only during production_mode = 0  

You can [`read`](crate::Reg::read) this register and get [`prod0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prod0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROD0_CTL_SPEC;
impl crate::RegisterSpec for PROD0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prod0_ctl::R`](R) reader structure"]
impl crate::Readable for PROD0_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prod0_ctl::W`](W) writer structure"]
impl crate::Writable for PROD0_CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROD0_CTL to value 0x3000"]
impl crate::Resettable for PROD0_CTL_SPEC {
    const RESET_VALUE: u32 = 0x3000;
}
