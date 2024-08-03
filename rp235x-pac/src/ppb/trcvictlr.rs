#[doc = "Register `TRCVICTLR` reader"]
pub type R = crate::R<TRCVICTLR_SPEC>;
#[doc = "Register `TRCVICTLR` writer"]
pub type W = crate::W<TRCVICTLR_SPEC>;
#[doc = "Field `SEL0` reader - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_R = crate::FieldReader;
#[doc = "Field `SEL0` writer - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
pub type SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPE0` reader - Selects the resource type for event 0"]
pub type TYPE0_R = crate::BitReader;
#[doc = "Field `TYPE0` writer - Selects the resource type for event 0"]
pub type TYPE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSTATUS` reader - Indicates the current status of the start/stop logic"]
pub type SSSTATUS_R = crate::BitReader;
#[doc = "Field `SSSTATUS` writer - Indicates the current status of the start/stop logic"]
pub type SSSTATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCRESET` reader - Selects whether a reset exception must always be traced"]
pub type TRCRESET_R = crate::BitReader;
#[doc = "Field `TRCRESET` writer - Selects whether a reset exception must always be traced"]
pub type TRCRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCERR` reader - Selects whether a system error exception must always be traced"]
pub type TRCERR_R = crate::BitReader;
#[doc = "Field `TRCERR` writer - Selects whether a system error exception must always be traced"]
pub type TRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXLEVEL_S0` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
pub type EXLEVEL_S0_R = crate::BitReader;
#[doc = "Field `EXLEVEL_S0` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
pub type EXLEVEL_S0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXLEVEL_S3` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
pub type EXLEVEL_S3_R = crate::BitReader;
#[doc = "Field `EXLEVEL_S3` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
pub type EXLEVEL_S3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 9 - Indicates the current status of the start/stop logic"]
    #[inline(always)]
    pub fn ssstatus(&self) -> SSSTATUS_R {
        SSSTATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects whether a reset exception must always be traced"]
    #[inline(always)]
    pub fn trcreset(&self) -> TRCRESET_R {
        TRCRESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects whether a system error exception must always be traced"]
    #[inline(always)]
    pub fn trcerr(&self) -> TRCERR_R {
        TRCERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
    #[inline(always)]
    pub fn exlevel_s0(&self) -> EXLEVEL_S0_R {
        EXLEVEL_S0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
    #[inline(always)]
    pub fn exlevel_s3(&self) -> EXLEVEL_S3_R {
        EXLEVEL_S3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the resource number, based on the value of TYPE0: When TYPE1 is 0, selects a single selected resource from 0-15 defined by SEL0\\[2:0\\]. When TYPE1 is 1, selects a Boolean combined resource pair from 0-7 defined by SEL0\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<TRCVICTLR_SPEC> {
        SEL0_W::new(self, 0)
    }
    #[doc = "Bit 7 - Selects the resource type for event 0"]
    #[inline(always)]
    #[must_use]
    pub fn type0(&mut self) -> TYPE0_W<TRCVICTLR_SPEC> {
        TYPE0_W::new(self, 7)
    }
    #[doc = "Bit 9 - Indicates the current status of the start/stop logic"]
    #[inline(always)]
    #[must_use]
    pub fn ssstatus(&mut self) -> SSSTATUS_W<TRCVICTLR_SPEC> {
        SSSTATUS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects whether a reset exception must always be traced"]
    #[inline(always)]
    #[must_use]
    pub fn trcreset(&mut self) -> TRCRESET_W<TRCVICTLR_SPEC> {
        TRCRESET_W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects whether a system error exception must always be traced"]
    #[inline(always)]
    #[must_use]
    pub fn trcerr(&mut self) -> TRCERR_W<TRCVICTLR_SPEC> {
        TRCERR_W::new(self, 11)
    }
    #[doc = "Bit 16 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_s0(&mut self) -> EXLEVEL_S0_W<TRCVICTLR_SPEC> {
        EXLEVEL_S0_W::new(self, 16)
    }
    #[doc = "Bit 19 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level"]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_s3(&mut self) -> EXLEVEL_S3_W<TRCVICTLR_SPEC> {
        EXLEVEL_S3_W::new(self, 19)
    }
}
#[doc = "The TRCVICTLR controls instruction trace filtering  

You can [`read`](crate::Reg::read) this register and get [`trcvictlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvictlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCVICTLR_SPEC;
impl crate::RegisterSpec for TRCVICTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcvictlr::R`](R) reader structure"]
impl crate::Readable for TRCVICTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcvictlr::W`](W) writer structure"]
impl crate::Writable for TRCVICTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCVICTLR to value 0"]
impl crate::Resettable for TRCVICTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
