#[doc = "Register `PLATFORM` reader"]
pub type R = crate::R<PLATFORM_SPEC>;
#[doc = "Field `FPGA` reader - "]
pub type FPGA_R = crate::BitReader;
#[doc = "Field `ASIC` reader - "]
pub type ASIC_R = crate::BitReader;
#[doc = "Field `HDLSIM` reader - "]
pub type HDLSIM_R = crate::BitReader;
#[doc = "Field `BATCHSIM` reader - "]
pub type BATCHSIM_R = crate::BitReader;
#[doc = "Field `GATESIM` reader - "]
pub type GATESIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hdlsim(&self) -> HDLSIM_R {
        HDLSIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn batchsim(&self) -> BATCHSIM_R {
        BATCHSIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gatesim(&self) -> GATESIM_R {
        GATESIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM.  

You can [`read`](crate::Reg::read) this register and get [`platform::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`platform::R`](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {}
#[doc = "`reset()` method sets PLATFORM to value 0"]
impl crate::Resettable for PLATFORM_SPEC {
    const RESET_VALUE: u32 = 0;
}
