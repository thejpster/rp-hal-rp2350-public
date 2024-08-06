#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DEVTYPE_SPEC>;
#[doc = "Register `DEVTYPE` writer"]
pub type W = crate::W<DEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - Major classification of the type of the debug component as specified in the ARM Architecture Specification for this debug and trace component."]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - Sub-classification of the type of the debug component as specified in the ARM Architecture Specification within the major classification as specified in the MAJOR field."]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Major classification of the type of the debug component as specified in the ARM Architecture Specification for this debug and trace component."]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-classification of the type of the debug component as specified in the ARM Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Device Type Identifier register  

You can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devtype::W`](W) writer structure"]
impl crate::Writable for DEVTYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVTYPE to value 0x14"]
impl crate::Resettable for DEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
