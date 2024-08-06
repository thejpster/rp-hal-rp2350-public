#[doc = "Register `TRCIDR3` reader"]
pub type R = crate::R<TRCIDR3_SPEC>;
#[doc = "Register `TRCIDR3` writer"]
pub type W = crate::W<TRCIDR3_SPEC>;
#[doc = "Field `CCITMIN` reader - reads as `ImpDef"]
pub type CCITMIN_R = crate::FieldReader<u16>;
#[doc = "Field `EXLEVEL_S` reader - reads as `ImpDef"]
pub type EXLEVEL_S_R = crate::FieldReader;
#[doc = "Field `EXLEVEL_NS` reader - reads as `ImpDef"]
pub type EXLEVEL_NS_R = crate::FieldReader;
#[doc = "Field `TRCERR` reader - reads as `ImpDef"]
pub type TRCERR_R = crate::BitReader;
#[doc = "Field `SYNCPR` reader - reads as `ImpDef"]
pub type SYNCPR_R = crate::BitReader;
#[doc = "Field `STALLCTL` reader - reads as `ImpDef"]
pub type STALLCTL_R = crate::BitReader;
#[doc = "Field `SYSSTALL` reader - reads as `ImpDef"]
pub type SYSSTALL_R = crate::BitReader;
#[doc = "Field `NUMPROC` reader - reads as `ImpDef"]
pub type NUMPROC_R = crate::FieldReader;
#[doc = "Field `NOOVERFLOW` reader - reads as `ImpDef"]
pub type NOOVERFLOW_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - reads as `ImpDef"]
    #[inline(always)]
    pub fn ccitmin(&self) -> CCITMIN_R {
        CCITMIN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - reads as `ImpDef"]
    #[inline(always)]
    pub fn exlevel_s(&self) -> EXLEVEL_S_R {
        EXLEVEL_S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - reads as `ImpDef"]
    #[inline(always)]
    pub fn exlevel_ns(&self) -> EXLEVEL_NS_R {
        EXLEVEL_NS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trcerr(&self) -> TRCERR_R {
        TRCERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reads as `ImpDef"]
    #[inline(always)]
    pub fn syncpr(&self) -> SYNCPR_R {
        SYNCPR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reads as `ImpDef"]
    #[inline(always)]
    pub fn stallctl(&self) -> STALLCTL_R {
        STALLCTL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reads as `ImpDef"]
    #[inline(always)]
    pub fn sysstall(&self) -> SYSSTALL_R {
        SYSSTALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numproc(&self) -> NUMPROC_R {
        NUMPROC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nooverflow(&self) -> NOOVERFLOW_R {
        NOOVERFLOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "TRCIDR3  

You can [`read`](crate::Reg::read) this register and get [`trcidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR3_SPEC;
impl crate::RegisterSpec for TRCIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr3::R`](R) reader structure"]
impl crate::Readable for TRCIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr3::W`](W) writer structure"]
impl crate::Writable for TRCIDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR3 to value 0x0f09_0004"]
impl crate::Resettable for TRCIDR3_SPEC {
    const RESET_VALUE: u32 = 0x0f09_0004;
}
