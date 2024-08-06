#[doc = "Register `ID_ISAR4` reader"]
pub type R = crate::R<ID_ISAR4_SPEC>;
#[doc = "Register `ID_ISAR4` writer"]
pub type W = crate::W<ID_ISAR4_SPEC>;
#[doc = "Field `UNPRIV` reader - Indicates the implemented unprivileged instructions"]
pub type UNPRIV_R = crate::FieldReader;
#[doc = "Field `WITHSHIFTS` reader - Indicates the support for writeback addressing modes"]
pub type WITHSHIFTS_R = crate::FieldReader;
#[doc = "Field `WRITEBACK` reader - Indicates the support for writeback addressing modes"]
pub type WRITEBACK_R = crate::FieldReader;
#[doc = "Field `BARRIER` reader - Indicates the implemented Barrier instructions"]
pub type BARRIER_R = crate::FieldReader;
#[doc = "Field `SYNCPRIM_FRAC` reader - Used in conjunction with ID_ISAR3.SynchPrim to indicate the implemented Synchronization Primitive instructions"]
pub type SYNCPRIM_FRAC_R = crate::FieldReader;
#[doc = "Field `PSR_M` reader - Indicates the implemented M profile instructions to modify the PSRs"]
pub type PSR_M_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the implemented unprivileged instructions"]
    #[inline(always)]
    pub fn unpriv(&self) -> UNPRIV_R {
        UNPRIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the support for writeback addressing modes"]
    #[inline(always)]
    pub fn withshifts(&self) -> WITHSHIFTS_R {
        WITHSHIFTS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the support for writeback addressing modes"]
    #[inline(always)]
    pub fn writeback(&self) -> WRITEBACK_R {
        WRITEBACK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the implemented Barrier instructions"]
    #[inline(always)]
    pub fn barrier(&self) -> BARRIER_R {
        BARRIER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Used in conjunction with ID_ISAR3.SynchPrim to indicate the implemented Synchronization Primitive instructions"]
    #[inline(always)]
    pub fn syncprim_frac(&self) -> SYNCPRIM_FRAC_R {
        SYNCPRIM_FRAC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the implemented M profile instructions to modify the PSRs"]
    #[inline(always)]
    pub fn psr_m(&self) -> PSR_M_R {
        PSR_M_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR4_SPEC;
impl crate::RegisterSpec for ID_ISAR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar4::R`](R) reader structure"]
impl crate::Readable for ID_ISAR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_isar4::W`](W) writer structure"]
impl crate::Writable for ID_ISAR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR4 to value 0x0131_0132"]
impl crate::Resettable for ID_ISAR4_SPEC {
    const RESET_VALUE: u32 = 0x0131_0132;
}
