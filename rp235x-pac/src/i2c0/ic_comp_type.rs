#[doc = "Register `IC_COMP_TYPE` reader"]
pub type R = crate::R<IC_COMP_TYPE_SPEC>;
#[doc = "Register `IC_COMP_TYPE` writer"]
pub type W = crate::W<IC_COMP_TYPE_SPEC>;
#[doc = "Field `IC_COMP_TYPE` reader - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
pub type IC_COMP_TYPE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
    #[inline(always)]
    pub fn ic_comp_type(&self) -> IC_COMP_TYPE_R {
        IC_COMP_TYPE_R::new(self.bits)
    }
}
impl W {}
#[doc = "I2C Component Type Register  

You can [`read`](crate::Reg::read) this register and get [`ic_comp_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_comp_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_COMP_TYPE_SPEC;
impl crate::RegisterSpec for IC_COMP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_comp_type::R`](R) reader structure"]
impl crate::Readable for IC_COMP_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_comp_type::W`](W) writer structure"]
impl crate::Writable for IC_COMP_TYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_COMP_TYPE to value 0x4457_0140"]
impl crate::Resettable for IC_COMP_TYPE_SPEC {
    const RESET_VALUE: u32 = 0x4457_0140;
}
