#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<CIDR1_SPEC>;
#[doc = "Register `CIDR1` writer"]
pub type W = crate::W<CIDR1_SPEC>;
#[doc = "Field `PRMBL_1` reader - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code."]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code."]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "CoreSight Component ID1  

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for CIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cidr1::W`](W) writer structure"]
impl crate::Writable for CIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for CIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
