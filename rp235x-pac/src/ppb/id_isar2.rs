#[doc = "Register `ID_ISAR2` reader"]
pub type R = crate::R<ID_ISAR2_SPEC>;
#[doc = "Field `LOADSTORE` reader - Indicates the implemented additional load/store instructions"]
pub type LOADSTORE_R = crate::FieldReader;
#[doc = "Field `MEMHINT` reader - Indicates the implemented Memory Hint instructions"]
pub type MEMHINT_R = crate::FieldReader;
#[doc = "Field `MULTIACCESSINT` reader - Indicates the support for interruptible multi-access instructions"]
pub type MULTIACCESSINT_R = crate::FieldReader;
#[doc = "Field `MULT` reader - Indicates the implemented additional Multiply instructions"]
pub type MULT_R = crate::FieldReader;
#[doc = "Field `MULTS` reader - Indicates the implemented advanced signed Multiply instructions"]
pub type MULTS_R = crate::FieldReader;
#[doc = "Field `MULTU` reader - Indicates the implemented advanced unsigned Multiply instructions"]
pub type MULTU_R = crate::FieldReader;
#[doc = "Field `REVERSAL` reader - Indicates the implemented Reversal instructions"]
pub type REVERSAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the implemented additional load/store instructions"]
    #[inline(always)]
    pub fn loadstore(&self) -> LOADSTORE_R {
        LOADSTORE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the implemented Memory Hint instructions"]
    #[inline(always)]
    pub fn memhint(&self) -> MEMHINT_R {
        MEMHINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the support for interruptible multi-access instructions"]
    #[inline(always)]
    pub fn multiaccessint(&self) -> MULTIACCESSINT_R {
        MULTIACCESSINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the implemented additional Multiply instructions"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the implemented advanced signed Multiply instructions"]
    #[inline(always)]
    pub fn mults(&self) -> MULTS_R {
        MULTS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the implemented advanced unsigned Multiply instructions"]
    #[inline(always)]
    pub fn multu(&self) -> MULTU_R {
        MULTU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates the implemented Reversal instructions"]
    #[inline(always)]
    pub fn reversal(&self) -> REVERSAL_R {
        REVERSAL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR2_SPEC;
impl crate::RegisterSpec for ID_ISAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar2::R`](R) reader structure"]
impl crate::Readable for ID_ISAR2_SPEC {}
#[doc = "`reset()` method sets ID_ISAR2 to value 0x3017_3426"]
impl crate::Resettable for ID_ISAR2_SPEC {
    const RESET_VALUE: u32 = 0x3017_3426;
}
