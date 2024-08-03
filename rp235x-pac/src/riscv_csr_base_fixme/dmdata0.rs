#[doc = "Register `DMDATA0` reader"]
pub type R = crate::R<DMDATA0_SPEC>;
#[doc = "Register `DMDATA0` writer"]
pub type W = crate::W<DMDATA0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The Debug Module's DATA0 register is mapped into Hazard3's CSR space so that the Debug Module can exchange data with the core by executing CSR access instructions (this is used to implement the Abstract Access Register command). Only accessible in Debug Mode.  

You can [`read`](crate::Reg::read) this register and get [`dmdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMDATA0_SPEC;
impl crate::RegisterSpec for DMDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmdata0::R`](R) reader structure"]
impl crate::Readable for DMDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmdata0::W`](W) writer structure"]
impl crate::Writable for DMDATA0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMDATA0 to value 0"]
impl crate::Resettable for DMDATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
