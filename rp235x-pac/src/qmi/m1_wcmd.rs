#[doc = "Register `M1_WCMD` reader"]
pub type R = crate::R<M1_WCMD_SPEC>;
#[doc = "Register `M1_WCMD` writer"]
pub type W = crate::W<M1_WCMD_SPEC>;
#[doc = "Field `PREFIX` reader - The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
pub type PREFIX_R = crate::FieldReader;
#[doc = "Field `PREFIX` writer - The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
pub type PREFIX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SUFFIX` reader - The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
pub type SUFFIX_R = crate::FieldReader;
#[doc = "Field `SUFFIX` writer - The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
pub type SUFFIX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn prefix(&self) -> PREFIX_R {
        PREFIX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn suffix(&self) -> SUFFIX_R {
        SUFFIX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    #[must_use]
    pub fn prefix(&mut self) -> PREFIX_W<M1_WCMD_SPEC> {
        PREFIX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    #[must_use]
    pub fn suffix(&mut self) -> SUFFIX_W<M1_WCMD_SPEC> {
        SUFFIX_W::new(self, 8)
    }
}
#[doc = "Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m1_wcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_wcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1_WCMD_SPEC;
impl crate::RegisterSpec for M1_WCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1_wcmd::R`](R) reader structure"]
impl crate::Readable for M1_WCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m1_wcmd::W`](W) writer structure"]
impl crate::Writable for M1_WCMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1_WCMD to value 0xa002"]
impl crate::Resettable for M1_WCMD_SPEC {
    const RESET_VALUE: u32 = 0xa002;
}
