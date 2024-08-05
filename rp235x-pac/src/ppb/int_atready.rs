#[doc = "Register `INT_ATREADY` reader"]
pub type R = crate::R<INT_ATREADY_SPEC>;
#[doc = "Register `INT_ATREADY` writer"]
pub type W = crate::W<INT_ATREADY_SPEC>;
#[doc = "Field `ATREADY` reader - A read of this bit returns the value of ATREADY"]
pub type ATREADY_R = crate::BitReader;
#[doc = "Field `AFVALID` reader - A read of this bit returns the value of AFVALID"]
pub type AFVALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - A read of this bit returns the value of ATREADY"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A read of this bit returns the value of AFVALID"]
    #[inline(always)]
    pub fn afvalid(&self) -> AFVALID_R {
        AFVALID_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Integration Mode: Read ATB Ready  

You can [`read`](crate::Reg::read) this register and get [`int_atready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_atready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ATREADY_SPEC;
impl crate::RegisterSpec for INT_ATREADY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_atready::R`](R) reader structure"]
impl crate::Readable for INT_ATREADY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_atready::W`](W) writer structure"]
impl crate::Writable for INT_ATREADY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ATREADY to value 0"]
impl crate::Resettable for INT_ATREADY_SPEC {
    const RESET_VALUE: u32 = 0;
}
