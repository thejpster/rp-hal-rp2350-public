#[doc = "Register `TRCEVENTCTL0R` reader"]
pub type R = crate::R<TRCEVENTCTL0R_SPEC>;
#[doc = "Register `TRCEVENTCTL0R` writer"]
pub type W = crate::W<TRCEVENTCTL0R_SPEC>;
#[doc = "Field `SEL0` reader - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_R = crate::FieldReader;
#[doc = "Field `SEL0` writer - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TYPE0` reader - Selects the resource type for event 0"]
pub type TYPE0_R = crate::BitReader;
#[doc = "Field `TYPE0` writer - Selects the resource type for event 0"]
pub type TYPE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL1` reader - Selects the resource number, based on the value of TYPE1: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL1\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL1\\[2:0\\]"]
pub type SEL1_R = crate::FieldReader;
#[doc = "Field `SEL1` writer - Selects the resource number, based on the value of TYPE1: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL1\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL1\\[2:0\\]"]
pub type SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TYPE1` reader - Selects the resource type for event 1"]
pub type TYPE1_R = crate::BitReader;
#[doc = "Field `TYPE1` writer - Selects the resource type for event 1"]
pub type TYPE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Selects the resource type for event 0"]
    #[inline(always)]
    pub fn type0(&self) -> TYPE0_R {
        TYPE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Selects the resource number, based on the value of TYPE1: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL1\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL1\\[2:0\\]"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Selects the resource type for event 1"]
    #[inline(always)]
    pub fn type1(&self) -> TYPE1_R {
        TYPE1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<TRCEVENTCTL0R_SPEC> {
        SEL0_W::new(self, 0)
    }
    #[doc = "Bit 7 - Selects the resource type for event 0"]
    #[inline(always)]
    #[must_use]
    pub fn type0(&mut self) -> TYPE0_W<TRCEVENTCTL0R_SPEC> {
        TYPE0_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Selects the resource number, based on the value of TYPE1: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL1\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL1\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<TRCEVENTCTL0R_SPEC> {
        SEL1_W::new(self, 8)
    }
    #[doc = "Bit 15 - Selects the resource type for event 1"]
    #[inline(always)]
    #[must_use]
    pub fn type1(&mut self) -> TYPE1_W<TRCEVENTCTL0R_SPEC> {
        TYPE1_W::new(self, 15)
    }
}
#[doc = "The TRCEVENTCTL0R controls the tracing of events in the trace stream. The events also drive the ETM-Teal external outputs.  

You can [`read`](crate::Reg::read) this register and get [`trceventctl0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCEVENTCTL0R_SPEC;
impl crate::RegisterSpec for TRCEVENTCTL0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trceventctl0r::R`](R) reader structure"]
impl crate::Readable for TRCEVENTCTL0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trceventctl0r::W`](W) writer structure"]
impl crate::Writable for TRCEVENTCTL0R_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCEVENTCTL0R to value 0"]
impl crate::Resettable for TRCEVENTCTL0R_SPEC {
    const RESET_VALUE: u32 = 0;
}
