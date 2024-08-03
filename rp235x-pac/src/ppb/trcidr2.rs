#[doc = "Register `TRCIDR2` reader"]
pub type R = crate::R<TRCIDR2_SPEC>;
#[doc = "Field `IASIZE` reader - reads as `ImpDef"]
pub type IASIZE_R = crate::FieldReader;
#[doc = "Field `CIDSIZE` reader - reads as `ImpDef"]
pub type CIDSIZE_R = crate::FieldReader;
#[doc = "Field `VMIDSIZE` reader - reads as `ImpDef"]
pub type VMIDSIZE_R = crate::FieldReader;
#[doc = "Field `DASIZE` reader - reads as `ImpDef"]
pub type DASIZE_R = crate::FieldReader;
#[doc = "Field `DVSIZE` reader - reads as `ImpDef"]
pub type DVSIZE_R = crate::FieldReader;
#[doc = "Field `CCSIZE` reader - reads as `ImpDef"]
pub type CCSIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - reads as `ImpDef"]
    #[inline(always)]
    pub fn iasize(&self) -> IASIZE_R {
        IASIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - reads as `ImpDef"]
    #[inline(always)]
    pub fn cidsize(&self) -> CIDSIZE_R {
        CIDSIZE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - reads as `ImpDef"]
    #[inline(always)]
    pub fn vmidsize(&self) -> VMIDSIZE_R {
        VMIDSIZE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - reads as `ImpDef"]
    #[inline(always)]
    pub fn dasize(&self) -> DASIZE_R {
        DASIZE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - reads as `ImpDef"]
    #[inline(always)]
    pub fn dvsize(&self) -> DVSIZE_R {
        DVSIZE_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - reads as `ImpDef"]
    #[inline(always)]
    pub fn ccsize(&self) -> CCSIZE_R {
        CCSIZE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[doc = "TRCIDR2  

You can [`read`](crate::Reg::read) this register and get [`trcidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR2_SPEC;
impl crate::RegisterSpec for TRCIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr2::R`](R) reader structure"]
impl crate::Readable for TRCIDR2_SPEC {}
#[doc = "`reset()` method sets TRCIDR2 to value 0x04"]
impl crate::Resettable for TRCIDR2_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
