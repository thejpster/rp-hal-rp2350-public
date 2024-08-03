#[doc = "Register `FP_COMP5` reader"]
pub type R = crate::R<FP_COMP5_SPEC>;
#[doc = "Register `FP_COMP5` writer"]
pub type W = crate::W<FP_COMP5_SPEC>;
#[doc = "Field `BE` reader - Selects between flashpatch and breakpoint functionality"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Selects between flashpatch and breakpoint functionality"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<FP_COMP5_SPEC> {
        BE_W::new(self, 0)
    }
}
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_COMP5_SPEC;
impl crate::RegisterSpec for FP_COMP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_comp5::R`](R) reader structure"]
impl crate::Readable for FP_COMP5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_comp5::W`](W) writer structure"]
impl crate::Writable for FP_COMP5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_COMP5 to value 0"]
impl crate::Resettable for FP_COMP5_SPEC {
    const RESET_VALUE: u32 = 0;
}
