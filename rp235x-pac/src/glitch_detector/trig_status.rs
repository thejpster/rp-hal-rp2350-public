#[doc = "Register `TRIG_STATUS` reader"]
pub type R = crate::R<TRIG_STATUS_SPEC>;
#[doc = "Register `TRIG_STATUS` writer"]
pub type W = crate::W<TRIG_STATUS_SPEC>;
#[doc = "Field `DET0` reader - "]
pub type DET0_R = crate::BitReader;
#[doc = "Field `DET0` writer - "]
pub type DET0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DET1` reader - "]
pub type DET1_R = crate::BitReader;
#[doc = "Field `DET1` writer - "]
pub type DET1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DET2` reader - "]
pub type DET2_R = crate::BitReader;
#[doc = "Field `DET2` writer - "]
pub type DET2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DET3` reader - "]
pub type DET3_R = crate::BitReader;
#[doc = "Field `DET3` writer - "]
pub type DET3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn det0(&self) -> DET0_R {
        DET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn det1(&self) -> DET1_R {
        DET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn det2(&self) -> DET2_R {
        DET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn det3(&self) -> DET3_R {
        DET3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn det0(&mut self) -> DET0_W<TRIG_STATUS_SPEC> {
        DET0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn det1(&mut self) -> DET1_W<TRIG_STATUS_SPEC> {
        DET1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn det2(&mut self) -> DET2_W<TRIG_STATUS_SPEC> {
        DET2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn det3(&mut self) -> DET3_W<TRIG_STATUS_SPEC> {
        DET3_W::new(self, 3)
    }
}
#[doc = "Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`trig_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIG_STATUS_SPEC;
impl crate::RegisterSpec for TRIG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig_status::R`](R) reader structure"]
impl crate::Readable for TRIG_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trig_status::W`](W) writer structure"]
impl crate::Writable for TRIG_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets TRIG_STATUS to value 0"]
impl crate::Resettable for TRIG_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
