#[doc = "Register `INT_ATVALID` reader"]
pub type R = crate::R<INT_ATVALID_SPEC>;
#[doc = "Register `INT_ATVALID` writer"]
pub type W = crate::W<INT_ATVALID_SPEC>;
#[doc = "Field `ATREADY` reader - A write to this bit gives the value of ATVALID"]
pub type ATREADY_R = crate::BitReader;
#[doc = "Field `ATREADY` writer - A write to this bit gives the value of ATVALID"]
pub type ATREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - A write to this bit gives the value of AFREADY"]
pub type AFREADY_R = crate::BitReader;
#[doc = "Field `AFREADY` writer - A write to this bit gives the value of AFREADY"]
pub type AFREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A write to this bit gives the value of ATVALID"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A write to this bit gives the value of AFREADY"]
    #[inline(always)]
    pub fn afready(&self) -> AFREADY_R {
        AFREADY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A write to this bit gives the value of ATVALID"]
    #[inline(always)]
    #[must_use]
    pub fn atready(&mut self) -> ATREADY_W<INT_ATVALID_SPEC> {
        ATREADY_W::new(self, 0)
    }
    #[doc = "Bit 1 - A write to this bit gives the value of AFREADY"]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AFREADY_W<INT_ATVALID_SPEC> {
        AFREADY_W::new(self, 1)
    }
}
#[doc = "Integration Mode: Write ATB Valid  

You can [`read`](crate::Reg::read) this register and get [`int_atvalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_atvalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ATVALID_SPEC;
impl crate::RegisterSpec for INT_ATVALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_atvalid::R`](R) reader structure"]
impl crate::Readable for INT_ATVALID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_atvalid::W`](W) writer structure"]
impl crate::Writable for INT_ATVALID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ATVALID to value 0"]
impl crate::Resettable for INT_ATVALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
