#[doc = "Register `ID_ISAR3` reader"]
pub type R = crate::R<ID_ISAR3_SPEC>;
#[doc = "Register `ID_ISAR3` writer"]
pub type W = crate::W<ID_ISAR3_SPEC>;
#[doc = "Field `SATURATE` reader - Indicates the implemented saturating instructions"]
pub type SATURATE_R = crate::FieldReader;
#[doc = "Field `SIMD` reader - Indicates the implemented SIMD instructions"]
pub type SIMD_R = crate::FieldReader;
#[doc = "Field `SVC` reader - Indicates the implemented SVC instructions"]
pub type SVC_R = crate::FieldReader;
#[doc = "Field `SYNCHPRIM` reader - Used in conjunction with ID_ISAR4.SynchPrim_frac to indicate the implemented Synchronization Primitive instructions"]
pub type SYNCHPRIM_R = crate::FieldReader;
#[doc = "Field `TABBRANCH` reader - Indicates the implemented Table Branch instructions"]
pub type TABBRANCH_R = crate::FieldReader;
#[doc = "Field `T32COPY` reader - Indicates the support for T32 non flag-setting MOV instructions"]
pub type T32COPY_R = crate::FieldReader;
#[doc = "Field `TRUENOP` reader - Indicates the implemented true NOP instructions"]
pub type TRUENOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the implemented saturating instructions"]
    #[inline(always)]
    pub fn saturate(&self) -> SATURATE_R {
        SATURATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the implemented SIMD instructions"]
    #[inline(always)]
    pub fn simd(&self) -> SIMD_R {
        SIMD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the implemented SVC instructions"]
    #[inline(always)]
    pub fn svc(&self) -> SVC_R {
        SVC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Used in conjunction with ID_ISAR4.SynchPrim_frac to indicate the implemented Synchronization Primitive instructions"]
    #[inline(always)]
    pub fn synchprim(&self) -> SYNCHPRIM_R {
        SYNCHPRIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the implemented Table Branch instructions"]
    #[inline(always)]
    pub fn tabbranch(&self) -> TABBRANCH_R {
        TABBRANCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the support for T32 non flag-setting MOV instructions"]
    #[inline(always)]
    pub fn t32copy(&self) -> T32COPY_R {
        T32COPY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the implemented true NOP instructions"]
    #[inline(always)]
    pub fn truenop(&self) -> TRUENOP_R {
        TRUENOP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR3_SPEC;
impl crate::RegisterSpec for ID_ISAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar3::R`](R) reader structure"]
impl crate::Readable for ID_ISAR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_isar3::W`](W) writer structure"]
impl crate::Writable for ID_ISAR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR3 to value 0x0789_5729"]
impl crate::Resettable for ID_ISAR3_SPEC {
    const RESET_VALUE: u32 = 0x0789_5729;
}
