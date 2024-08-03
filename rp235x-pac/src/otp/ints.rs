#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
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
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
