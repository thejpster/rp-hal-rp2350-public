#[doc = "Register `IRQSUMMARY_DORMANT_WAKE_NONSECURE1` reader"]
pub type R = crate::R<IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC>;
#[doc = "Register `IRQSUMMARY_DORMANT_WAKE_NONSECURE1` writer"]
pub type W = crate::W<IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC>;
#[doc = "Field `GPIO32` reader - "]
pub type GPIO32_R = crate::BitReader;
#[doc = "Field `GPIO33` reader - "]
pub type GPIO33_R = crate::BitReader;
#[doc = "Field `GPIO34` reader - "]
pub type GPIO34_R = crate::BitReader;
#[doc = "Field `GPIO35` reader - "]
pub type GPIO35_R = crate::BitReader;
#[doc = "Field `GPIO36` reader - "]
pub type GPIO36_R = crate::BitReader;
#[doc = "Field `GPIO37` reader - "]
pub type GPIO37_R = crate::BitReader;
#[doc = "Field `GPIO38` reader - "]
pub type GPIO38_R = crate::BitReader;
#[doc = "Field `GPIO39` reader - "]
pub type GPIO39_R = crate::BitReader;
#[doc = "Field `GPIO40` reader - "]
pub type GPIO40_R = crate::BitReader;
#[doc = "Field `GPIO41` reader - "]
pub type GPIO41_R = crate::BitReader;
#[doc = "Field `GPIO42` reader - "]
pub type GPIO42_R = crate::BitReader;
#[doc = "Field `GPIO43` reader - "]
pub type GPIO43_R = crate::BitReader;
#[doc = "Field `GPIO44` reader - "]
pub type GPIO44_R = crate::BitReader;
#[doc = "Field `GPIO45` reader - "]
pub type GPIO45_R = crate::BitReader;
#[doc = "Field `GPIO46` reader - "]
pub type GPIO46_R = crate::BitReader;
#[doc = "Field `GPIO47` reader - "]
pub type GPIO47_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio32(&self) -> GPIO32_R {
        GPIO32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio33(&self) -> GPIO33_R {
        GPIO33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio34(&self) -> GPIO34_R {
        GPIO34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio35(&self) -> GPIO35_R {
        GPIO35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio36(&self) -> GPIO36_R {
        GPIO36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio37(&self) -> GPIO37_R {
        GPIO37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio38(&self) -> GPIO38_R {
        GPIO38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio39(&self) -> GPIO39_R {
        GPIO39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio40(&self) -> GPIO40_R {
        GPIO40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio41(&self) -> GPIO41_R {
        GPIO41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio42(&self) -> GPIO42_R {
        GPIO42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio43(&self) -> GPIO43_R {
        GPIO43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio44(&self) -> GPIO44_R {
        GPIO44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio45(&self) -> GPIO45_R {
        GPIO45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio46(&self) -> GPIO46_R {
        GPIO46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio47(&self) -> GPIO47_R {
        GPIO47_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`irqsummary_dormant_wake_nonsecure1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_dormant_wake_nonsecure1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC;
impl crate::RegisterSpec for IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqsummary_dormant_wake_nonsecure1::R`](R) reader structure"]
impl crate::Readable for IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqsummary_dormant_wake_nonsecure1::W`](W) writer structure"]
impl crate::Writable for IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSUMMARY_DORMANT_WAKE_NONSECURE1 to value 0"]
impl crate::Resettable for IRQSUMMARY_DORMANT_WAKE_NONSECURE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
