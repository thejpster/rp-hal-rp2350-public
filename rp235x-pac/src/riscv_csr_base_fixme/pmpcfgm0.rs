#[doc = "Register `PMPCFGM0` reader"]
pub type R = crate::R<PMPCFGM0_SPEC>;
#[doc = "Register `PMPCFGM0` writer"]
pub type W = crate::W<PMPCFGM0_SPEC>;
#[doc = "Field `PMPCFGM0` reader - "]
pub type PMPCFGM0_R = crate::FieldReader<u16>;
#[doc = "Field `PMPCFGM0` writer - "]
pub type PMPCFGM0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pmpcfgm0(&self) -> PMPCFGM0_R {
        PMPCFGM0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pmpcfgm0(&mut self) -> PMPCFGM0_W<PMPCFGM0_SPEC> {
        PMPCFGM0_W::new(self, 0)
    }
}
#[doc = "PMP M-mode configuration. One bit per PMP region. Setting a bit makes the corresponding region apply to M-mode (like the `pmpcfg.L` bit) but does not lock the region.  

 PMP is useful for non-security-related purposes, such as stack guarding and peripheral emulation. This extension allows M-mode to freely use any currently unlocked regions for its own purposes, without the inconvenience of having to lock them.  

 Note that this does not grant any new capabilities to M-mode, since in the base standard it is already possible to apply unlocked regions to M-mode by locking them. In general, PMP regions should be locked in ascending region number order so they can't be subsequently overridden by currently unlocked regions.  

 Note also that this is not the same as the rule locking bypass bit in the ePMP extension, which does not permit locked and unlocked M-mode regions to coexist.  

 This is a Hazard3 custom CSR.  

You can [`read`](crate::Reg::read) this register and get [`pmpcfgm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfgm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPCFGM0_SPEC;
impl crate::RegisterSpec for PMPCFGM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpcfgm0::R`](R) reader structure"]
impl crate::Readable for PMPCFGM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpcfgm0::W`](W) writer structure"]
impl crate::Writable for PMPCFGM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPCFGM0 to value 0"]
impl crate::Resettable for PMPCFGM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
