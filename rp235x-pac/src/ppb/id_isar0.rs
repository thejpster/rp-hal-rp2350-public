#[doc = "Register `ID_ISAR0` reader"]
pub type R = crate::R<ID_ISAR0_SPEC>;
#[doc = "Register `ID_ISAR0` writer"]
pub type W = crate::W<ID_ISAR0_SPEC>;
#[doc = "Field `BITCOUNT` reader - Indicates the supported bit count instructions"]
pub type BITCOUNT_R = crate::FieldReader;
#[doc = "Field `BITFIELD` reader - Indicates the supported bit field instructions"]
pub type BITFIELD_R = crate::FieldReader;
#[doc = "Field `CMPBRANCH` reader - Indicates the supported combined Compare and Branch instructions"]
pub type CMPBRANCH_R = crate::FieldReader;
#[doc = "Field `COPROC` reader - Indicates the supported Coprocessor instructions"]
pub type COPROC_R = crate::FieldReader;
#[doc = "Field `DEBUG` reader - Indicates the implemented Debug instructions"]
pub type DEBUG_R = crate::FieldReader;
#[doc = "Field `DIVIDE` reader - Indicates the supported Divide instructions"]
pub type DIVIDE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Indicates the supported bit count instructions"]
    #[inline(always)]
    pub fn bitcount(&self) -> BITCOUNT_R {
        BITCOUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the supported bit field instructions"]
    #[inline(always)]
    pub fn bitfield(&self) -> BITFIELD_R {
        BITFIELD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the supported combined Compare and Branch instructions"]
    #[inline(always)]
    pub fn cmpbranch(&self) -> CMPBRANCH_R {
        CMPBRANCH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported Coprocessor instructions"]
    #[inline(always)]
    pub fn coproc(&self) -> COPROC_R {
        COPROC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the implemented Debug instructions"]
    #[inline(always)]
    pub fn debug(&self) -> DEBUG_R {
        DEBUG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported Divide instructions"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR0_SPEC;
impl crate::RegisterSpec for ID_ISAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar0::R`](R) reader structure"]
impl crate::Readable for ID_ISAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_isar0::W`](W) writer structure"]
impl crate::Writable for ID_ISAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR0 to value 0x0809_2300"]
impl crate::Resettable for ID_ISAR0_SPEC {
    const RESET_VALUE: u32 = 0x0809_2300;
}
