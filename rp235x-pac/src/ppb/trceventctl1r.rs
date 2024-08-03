#[doc = "Register `TRCEVENTCTL1R` reader"]
pub type R = crate::R<TRCEVENTCTL1R_SPEC>;
#[doc = "Register `TRCEVENTCTL1R` writer"]
pub type W = crate::W<TRCEVENTCTL1R_SPEC>;
#[doc = "Field `INSTEN0` reader - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type INSTEN0_R = crate::BitReader;
#[doc = "Field `INSTEN0` writer - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type INSTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTEN1` reader - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type INSTEN1_R = crate::BitReader;
#[doc = "Field `INSTEN1` writer - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type INSTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATB` reader - ATB enabled"]
pub type ATB_R = crate::BitReader;
#[doc = "Field `ATB` writer - ATB enabled"]
pub type ATB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOVERRIDE` reader - Low power state behavior override"]
pub type LPOVERRIDE_R = crate::BitReader;
#[doc = "Field `LPOVERRIDE` writer - Low power state behavior override"]
pub type LPOVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    pub fn insten0(&self) -> INSTEN0_R {
        INSTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    pub fn insten1(&self) -> INSTEN1_R {
        INSTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - ATB enabled"]
    #[inline(always)]
    pub fn atb(&self) -> ATB_R {
        ATB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low power state behavior override"]
    #[inline(always)]
    pub fn lpoverride(&self) -> LPOVERRIDE_R {
        LPOVERRIDE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    #[must_use]
    pub fn insten0(&mut self) -> INSTEN0_W<TRCEVENTCTL1R_SPEC> {
        INSTEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - One bit per event, to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    #[must_use]
    pub fn insten1(&mut self) -> INSTEN1_W<TRCEVENTCTL1R_SPEC> {
        INSTEN1_W::new(self, 1)
    }
    #[doc = "Bit 11 - ATB enabled"]
    #[inline(always)]
    #[must_use]
    pub fn atb(&mut self) -> ATB_W<TRCEVENTCTL1R_SPEC> {
        ATB_W::new(self, 11)
    }
    #[doc = "Bit 12 - Low power state behavior override"]
    #[inline(always)]
    #[must_use]
    pub fn lpoverride(&mut self) -> LPOVERRIDE_W<TRCEVENTCTL1R_SPEC> {
        LPOVERRIDE_W::new(self, 12)
    }
}
#[doc = "The TRCEVENTCTL1R controls how the events selected by TRCEVENTCTL0R behave  

You can [`read`](crate::Reg::read) this register and get [`trceventctl1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCEVENTCTL1R_SPEC;
impl crate::RegisterSpec for TRCEVENTCTL1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trceventctl1r::R`](R) reader structure"]
impl crate::Readable for TRCEVENTCTL1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trceventctl1r::W`](W) writer structure"]
impl crate::Writable for TRCEVENTCTL1R_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCEVENTCTL1R to value 0"]
impl crate::Resettable for TRCEVENTCTL1R_SPEC {
    const RESET_VALUE: u32 = 0;
}
