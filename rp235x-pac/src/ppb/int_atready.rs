#[doc = "Register `INT_ATREADY` reader"]
pub type R = crate::R<INT_ATREADY_SPEC>;
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
#[doc = "Integration Mode: Read ATB Ready  

You can [`read`](crate::Reg::read) this register and get [`int_atready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ATREADY_SPEC;
impl crate::RegisterSpec for INT_ATREADY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_atready::R`](R) reader structure"]
impl crate::Readable for INT_ATREADY_SPEC {}
#[doc = "`reset()` method sets INT_ATREADY to value 0"]
impl crate::Resettable for INT_ATREADY_SPEC {
    const RESET_VALUE: u32 = 0;
}
