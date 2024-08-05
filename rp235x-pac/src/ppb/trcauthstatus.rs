#[doc = "Register `TRCAUTHSTATUS` reader"]
pub type R = crate::R<TRCAUTHSTATUS_SPEC>;
#[doc = "Register `TRCAUTHSTATUS` writer"]
pub type W = crate::W<TRCAUTHSTATUS_SPEC>;
#[doc = "Field `NSID` reader - Indicates whether the trace unit supports Non-secure invasive debug:"]
pub type NSID_R = crate::FieldReader;
#[doc = "Field `NSNID` reader - Indicates whether the system enables the trace unit to support Non-secure non-invasive debug:"]
pub type NSNID_R = crate::FieldReader;
#[doc = "Field `SID` reader - Indicates whether the trace unit supports Secure invasive debug:"]
pub type SID_R = crate::FieldReader;
#[doc = "Field `SNID` reader - Indicates whether the system enables the trace unit to support Secure non-invasive debug:"]
pub type SNID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Indicates whether the trace unit supports Non-secure invasive debug:"]
    #[inline(always)]
    pub fn nsid(&self) -> NSID_R {
        NSID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicates whether the system enables the trace unit to support Non-secure non-invasive debug:"]
    #[inline(always)]
    pub fn nsnid(&self) -> NSNID_R {
        NSNID_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Indicates whether the trace unit supports Secure invasive debug:"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Indicates whether the system enables the trace unit to support Secure non-invasive debug:"]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {}
#[doc = "Returns the level of tracing that the trace unit can support  

You can [`read`](crate::Reg::read) this register and get [`trcauthstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcauthstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCAUTHSTATUS_SPEC;
impl crate::RegisterSpec for TRCAUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcauthstatus::R`](R) reader structure"]
impl crate::Readable for TRCAUTHSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcauthstatus::W`](W) writer structure"]
impl crate::Writable for TRCAUTHSTATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCAUTHSTATUS to value 0"]
impl crate::Resettable for TRCAUTHSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
