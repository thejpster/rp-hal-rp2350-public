#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `SBPI_FLAG_N` reader - "]
pub type SBPI_FLAG_N_R = crate::BitReader;
#[doc = "Field `SBPI_FLAG_N` writer - "]
pub type SBPI_FLAG_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPI_WR_FAIL` reader - "]
pub type SBPI_WR_FAIL_R = crate::BitReader;
#[doc = "Field `SBPI_WR_FAIL` writer - "]
pub type SBPI_WR_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_DCTRL_FAIL` reader - "]
pub type APB_DCTRL_FAIL_R = crate::BitReader;
#[doc = "Field `APB_DCTRL_FAIL` writer - "]
pub type APB_DCTRL_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_RD_SEC_FAIL` reader - "]
pub type APB_RD_SEC_FAIL_R = crate::BitReader;
#[doc = "Field `APB_RD_SEC_FAIL` writer - "]
pub type APB_RD_SEC_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_RD_NSEC_FAIL` reader - "]
pub type APB_RD_NSEC_FAIL_R = crate::BitReader;
#[doc = "Field `APB_RD_NSEC_FAIL` writer - "]
pub type APB_RD_NSEC_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sbpi_flag_n(&mut self) -> SBPI_FLAG_N_W<INTF_SPEC> {
        SBPI_FLAG_N_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sbpi_wr_fail(&mut self) -> SBPI_WR_FAIL_W<INTF_SPEC> {
        SBPI_WR_FAIL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn apb_dctrl_fail(&mut self) -> APB_DCTRL_FAIL_W<INTF_SPEC> {
        APB_DCTRL_FAIL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn apb_rd_sec_fail(&mut self) -> APB_RD_SEC_FAIL_W<INTF_SPEC> {
        APB_RD_SEC_FAIL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn apb_rd_nsec_fail(&mut self) -> APB_RD_NSEC_FAIL_W<INTF_SPEC> {
        APB_RD_NSEC_FAIL_W::new(self, 4)
    }
}
#[doc = "Interrupt Force  

You can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
