#[doc = "Register `ID_AFR0` reader"]
pub type R = crate::R<ID_AFR0_SPEC>;
#[doc = "Register `ID_AFR0` writer"]
pub type W = crate::W<ID_AFR0_SPEC>;
#[doc = "Field `IMPDEF0` reader - IMPLEMENTATION DEFINED meaning"]
pub type IMPDEF0_R = crate::FieldReader;
#[doc = "Field `IMPDEF1` reader - IMPLEMENTATION DEFINED meaning"]
pub type IMPDEF1_R = crate::FieldReader;
#[doc = "Field `IMPDEF2` reader - IMPLEMENTATION DEFINED meaning"]
pub type IMPDEF2_R = crate::FieldReader;
#[doc = "Field `IMPDEF3` reader - IMPLEMENTATION DEFINED meaning"]
pub type IMPDEF3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - IMPLEMENTATION DEFINED meaning"]
    #[inline(always)]
    pub fn impdef0(&self) -> IMPDEF0_R {
        IMPDEF0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - IMPLEMENTATION DEFINED meaning"]
    #[inline(always)]
    pub fn impdef1(&self) -> IMPDEF1_R {
        IMPDEF1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - IMPLEMENTATION DEFINED meaning"]
    #[inline(always)]
    pub fn impdef2(&self) -> IMPDEF2_R {
        IMPDEF2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IMPLEMENTATION DEFINED meaning"]
    #[inline(always)]
    pub fn impdef3(&self) -> IMPDEF3_R {
        IMPDEF3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the IMPLEMENTATION DEFINED features of the PE  

You can [`read`](crate::Reg::read) this register and get [`id_afr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_afr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_AFR0_SPEC;
impl crate::RegisterSpec for ID_AFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_afr0::R`](R) reader structure"]
impl crate::Readable for ID_AFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_afr0::W`](W) writer structure"]
impl crate::Writable for ID_AFR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_AFR0 to value 0"]
impl crate::Resettable for ID_AFR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
