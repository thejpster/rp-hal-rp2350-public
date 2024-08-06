#[doc = "Register `ID_MMFR3` reader"]
pub type R = crate::R<ID_MMFR3_SPEC>;
#[doc = "Register `ID_MMFR3` writer"]
pub type W = crate::W<ID_MMFR3_SPEC>;
#[doc = "Field `CMAINTVA` reader - Indicates the supported cache maintenance operations by address"]
pub type CMAINTVA_R = crate::FieldReader;
#[doc = "Field `CMAINTSW` reader - Indicates the supported cache maintenance operations by set/way"]
pub type CMAINTSW_R = crate::FieldReader;
#[doc = "Field `BPMAINT` reader - Indicates the supported branch predictor maintenance"]
pub type BPMAINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the supported cache maintenance operations by address"]
    #[inline(always)]
    pub fn cmaintva(&self) -> CMAINTVA_R {
        CMAINTVA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the supported cache maintenance operations by set/way"]
    #[inline(always)]
    pub fn cmaintsw(&self) -> CMAINTSW_R {
        CMAINTSW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the supported branch predictor maintenance"]
    #[inline(always)]
    pub fn bpmaint(&self) -> BPMAINT_R {
        BPMAINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_MMFR3_SPEC;
impl crate::RegisterSpec for ID_MMFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr3::R`](R) reader structure"]
impl crate::Readable for ID_MMFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_mmfr3::W`](W) writer structure"]
impl crate::Writable for ID_MMFR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_MMFR3 to value 0"]
impl crate::Resettable for ID_MMFR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
