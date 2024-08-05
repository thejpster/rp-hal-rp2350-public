#[doc = "Register `ID_ISAR1` reader"]
pub type R = crate::R<ID_ISAR1_SPEC>;
#[doc = "Register `ID_ISAR1` writer"]
pub type W = crate::W<ID_ISAR1_SPEC>;
#[doc = "Field `EXTEND` reader - Indicates the implemented Extend instructions"]
pub type EXTEND_R = crate::FieldReader;
#[doc = "Field `IFTHEN` reader - Indicates the implemented If-Then instructions"]
pub type IFTHEN_R = crate::FieldReader;
#[doc = "Field `IMMEDIATE` reader - Indicates the implemented for data-processing instructions with long immediates"]
pub type IMMEDIATE_R = crate::FieldReader;
#[doc = "Field `INTERWORK` reader - Indicates the implemented Interworking instructions"]
pub type INTERWORK_R = crate::FieldReader;
impl R {
    #[doc = "Bits 12:15 - Indicates the implemented Extend instructions"]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the implemented If-Then instructions"]
    #[inline(always)]
    pub fn ifthen(&self) -> IFTHEN_R {
        IFTHEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the implemented for data-processing instructions with long immediates"]
    #[inline(always)]
    pub fn immediate(&self) -> IMMEDIATE_R {
        IMMEDIATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the implemented Interworking instructions"]
    #[inline(always)]
    pub fn interwork(&self) -> INTERWORK_R {
        INTERWORK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR1_SPEC;
impl crate::RegisterSpec for ID_ISAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar1::R`](R) reader structure"]
impl crate::Readable for ID_ISAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_isar1::W`](W) writer structure"]
impl crate::Writable for ID_ISAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR1 to value 0x0572_5000"]
impl crate::Resettable for ID_ISAR1_SPEC {
    const RESET_VALUE: u32 = 0x0572_5000;
}
