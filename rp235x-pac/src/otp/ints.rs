#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Register `INTS` writer"]
pub type W = crate::W<INTS_SPEC>;
#[doc = "Field `SBPI_FLAG_N` reader - "]
pub type SBPI_FLAG_N_R = crate::BitReader;
#[doc = "Field `SBPI_WR_FAIL` reader - "]
pub type SBPI_WR_FAIL_R = crate::BitReader;
#[doc = "Field `APB_DCTRL_FAIL` reader - "]
pub type APB_DCTRL_FAIL_R = crate::BitReader;
#[doc = "Field `APB_RD_SEC_FAIL` reader - "]
pub type APB_RD_SEC_FAIL_R = crate::BitReader;
#[doc = "Field `APB_RD_NSEC_FAIL` reader - "]
pub type APB_RD_NSEC_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sbpi_flag_n(&self) -> SBPI_FLAG_N_R {
        SBPI_FLAG_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbpi_wr_fail(&self) -> SBPI_WR_FAIL_R {
        SBPI_WR_FAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_dctrl_fail(&self) -> APB_DCTRL_FAIL_R {
        APB_DCTRL_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_rd_sec_fail(&self) -> APB_RD_SEC_FAIL_R {
        APB_RD_SEC_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_rd_nsec_fail(&self) -> APB_RD_NSEC_FAIL_R {
        APB_RD_NSEC_FAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints::W`](W) writer structure"]
impl crate::Writable for INTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
