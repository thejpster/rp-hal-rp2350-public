#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CPUID_SPEC>;
#[doc = "Register `CPUID` writer"]
pub type W = crate::W<CPUID_SPEC>;
#[doc = "Field `REVISION` reader - IMPLEMENTATION DEFINED revision number for the device"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PARTNO` reader - IMPLEMENTATION DEFINED primary part number for the device"]
pub type PARTNO_R = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECTURE` reader - Defines the Architecture implemented by the PE"]
pub type ARCHITECTURE_R = crate::FieldReader;
#[doc = "Field `VARIANT` reader - IMPLEMENTATION DEFINED variant number. Typically, this field is used to distinguish between different product variants, or major revisions of a product"]
pub type VARIANT_R = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` reader - This field must hold an implementer code that has been assigned by ARM"]
pub type IMPLEMENTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - IMPLEMENTATION DEFINED revision number for the device"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - IMPLEMENTATION DEFINED primary part number for the device"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Defines the Architecture implemented by the PE"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - IMPLEMENTATION DEFINED variant number. Typically, this field is used to distinguish between different product variants, or major revisions of a product"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - This field must hold an implementer code that has been assigned by ARM"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides identification information for the PE, including an implementer code for the device and a device ID number  

You can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CPUID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpuid::W`](W) writer structure"]
impl crate::Writable for CPUID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUID to value 0x411f_d210"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: u32 = 0x411f_d210;
}
