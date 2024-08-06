#[doc = "Register `VREG_STS` reader"]
pub type R = crate::R<VREG_STS_SPEC>;
#[doc = "Register `VREG_STS` writer"]
pub type W = crate::W<VREG_STS_SPEC>;
#[doc = "Field `STARTUP` reader - startup status 0=startup complete, 1=starting up"]
pub type STARTUP_R = crate::BitReader;
#[doc = "Field `VOUT_OK` reader - output regulation status 0=not in regulation, 1=in regulation"]
pub type VOUT_OK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - startup status 0=startup complete, 1=starting up"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - output regulation status 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn vout_ok(&self) -> VOUT_OK_R {
        VOUT_OK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {}
#[doc = "Voltage Regulator Status  

You can [`read`](crate::Reg::read) this register and get [`vreg_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_STS_SPEC;
impl crate::RegisterSpec for VREG_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg_sts::R`](R) reader structure"]
impl crate::Readable for VREG_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg_sts::W`](W) writer structure"]
impl crate::Writable for VREG_STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREG_STS to value 0"]
impl crate::Resettable for VREG_STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
