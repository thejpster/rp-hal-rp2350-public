#[doc = "Register `SAU_RLAR` reader"]
pub type R = crate::R<SAU_RLAR_SPEC>;
#[doc = "Register `SAU_RLAR` writer"]
pub type W = crate::W<SAU_RLAR_SPEC>;
#[doc = "Field `ENABLE` reader - SAU region enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - SAU region enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSC` reader - Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NSC_R = crate::BitReader;
#[doc = "Field `NSC` writer - Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LADDR` reader - Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LADDR` writer - Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - SAU region enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:31 - Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - SAU region enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<SAU_RLAR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    #[must_use]
    pub fn nsc(&mut self) -> NSC_W<SAU_RLAR_SPEC> {
        NSC_W::new(self, 1)
    }
    #[doc = "Bits 5:31 - Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn laddr(&mut self) -> LADDR_W<SAU_RLAR_SPEC> {
        LADDR_W::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region  

You can [`read`](crate::Reg::read) this register and get [`sau_rlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAU_RLAR_SPEC;
impl crate::RegisterSpec for SAU_RLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sau_rlar::R`](R) reader structure"]
impl crate::Readable for SAU_RLAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sau_rlar::W`](W) writer structure"]
impl crate::Writable for SAU_RLAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAU_RLAR to value 0"]
impl crate::Resettable for SAU_RLAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
