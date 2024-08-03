#[doc = "Register `TRCIDR5` reader"]
pub type R = crate::R<TRCIDR5_SPEC>;
#[doc = "Field `NUMEXTIN` reader - reads as `ImpDef"]
pub type NUMEXTIN_R = crate::FieldReader<u16>;
#[doc = "Field `NUMEXTINSEL` reader - reads as `ImpDef"]
pub type NUMEXTINSEL_R = crate::FieldReader;
#[doc = "Field `TRACEIDSIZE` reader - reads as 0x07"]
pub type TRACEIDSIZE_R = crate::FieldReader;
#[doc = "Field `ATBTRIG` reader - reads as `ImpDef"]
pub type ATBTRIG_R = crate::BitReader;
#[doc = "Field `LPOVERRIDE` reader - reads as `ImpDef"]
pub type LPOVERRIDE_R = crate::BitReader;
#[doc = "Field `NUMSEQSTATE` reader - reads as `ImpDef"]
pub type NUMSEQSTATE_R = crate::FieldReader;
#[doc = "Field `NUMCNTR` reader - reads as `ImpDef"]
pub type NUMCNTR_R = crate::FieldReader;
#[doc = "Field `REDFUNCNTR` reader - reads as `ImpDef"]
pub type REDFUNCNTR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numextin(&self) -> NUMEXTIN_R {
        NUMEXTIN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numextinsel(&self) -> NUMEXTINSEL_R {
        NUMEXTINSEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 16:21 - reads as 0x07"]
    #[inline(always)]
    pub fn traceidsize(&self) -> TRACEIDSIZE_R {
        TRACEIDSIZE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - reads as `ImpDef"]
    #[inline(always)]
    pub fn atbtrig(&self) -> ATBTRIG_R {
        ATBTRIG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reads as `ImpDef"]
    #[inline(always)]
    pub fn lpoverride(&self) -> LPOVERRIDE_R {
        LPOVERRIDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:27 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numseqstate(&self) -> NUMSEQSTATE_R {
        NUMSEQSTATE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcntr(&self) -> NUMCNTR_R {
        NUMCNTR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn redfuncntr(&self) -> REDFUNCNTR_R {
        REDFUNCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "TRCIDR5  

You can [`read`](crate::Reg::read) this register and get [`trcidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR5_SPEC;
impl crate::RegisterSpec for TRCIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr5::R`](R) reader structure"]
impl crate::Readable for TRCIDR5_SPEC {}
#[doc = "`reset()` method sets TRCIDR5 to value 0x90c7_0004"]
impl crate::Resettable for TRCIDR5_SPEC {
    const RESET_VALUE: u32 = 0x90c7_0004;
}
