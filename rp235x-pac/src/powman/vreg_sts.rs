#[doc = "Register `VREG_STS` reader"]
pub type R = crate::R<VREG_STS_SPEC>;
#[doc = "Field `STARTUP` reader - startup status  
 0=startup complete, 1=starting up"]
pub type STARTUP_R = crate::BitReader;
#[doc = "Field `VOUT_OK` reader - output regulation status  
 0=not in regulation, 1=in regulation"]
pub type VOUT_OK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - startup status  
 0=startup complete, 1=starting up"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - output regulation status  
 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn vout_ok(&self) -> VOUT_OK_R {
        VOUT_OK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Voltage Regulator Status  

You can [`read`](crate::Reg::read) this register and get [`vreg_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_STS_SPEC;
impl crate::RegisterSpec for VREG_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg_sts::R`](R) reader structure"]
impl crate::Readable for VREG_STS_SPEC {}
#[doc = "`reset()` method sets VREG_STS to value 0"]
impl crate::Resettable for VREG_STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
