#[doc = "Register `TRCTSCTLR` reader"]
pub type R = crate::R<TRCTSCTLR_SPEC>;
#[doc = "Register `TRCTSCTLR` writer"]
pub type W = crate::W<TRCTSCTLR_SPEC>;
#[doc = "Field `SEL0` reader - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_R = crate::FieldReader;
#[doc = "Field `SEL0` writer - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPE0` reader - Selects the resource type for event 0"]
pub type TYPE0_R = crate::BitReader;
#[doc = "Field `TYPE0` writer - Selects the resource type for event 0"]
pub type TYPE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Selects the resource type for event 0"]
    #[inline(always)]
    pub fn type0(&self) -> TYPE0_R {
        TYPE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<TRCTSCTLR_SPEC> {
        SEL0_W::new(self, 0)
    }
    #[doc = "Bit 7 - Selects the resource type for event 0"]
    #[inline(always)]
    #[must_use]
    pub fn type0(&mut self) -> TYPE0_W<TRCTSCTLR_SPEC> {
        TYPE0_W::new(self, 7)
    }
}
#[doc = "The TRCTSCTLR controls the insertion of global timestamps into the trace stream. A timestamp is always inserted into the instruction trace stream  

You can [`read`](crate::Reg::read) this register and get [`trctsctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctsctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCTSCTLR_SPEC;
impl crate::RegisterSpec for TRCTSCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trctsctlr::R`](R) reader structure"]
impl crate::Readable for TRCTSCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trctsctlr::W`](W) writer structure"]
impl crate::Writable for TRCTSCTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCTSCTLR to value 0"]
impl crate::Resettable for TRCTSCTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
