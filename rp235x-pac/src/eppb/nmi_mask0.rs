#[doc = "Register `NMI_MASK0` reader"]
pub type R = crate::R<NMI_MASK0_SPEC>;
#[doc = "Register `NMI_MASK0` writer"]
pub type W = crate::W<NMI_MASK0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset.  

You can [`read`](crate::Reg::read) this register and get [`nmi_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMI_MASK0_SPEC;
impl crate::RegisterSpec for NMI_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi_mask0::R`](R) reader structure"]
impl crate::Readable for NMI_MASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmi_mask0::W`](W) writer structure"]
impl crate::Writable for NMI_MASK0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMI_MASK0 to value 0"]
impl crate::Resettable for NMI_MASK0_SPEC {
    const RESET_VALUE: u32 = 0;
}
