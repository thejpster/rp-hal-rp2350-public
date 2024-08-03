#[doc = "Register `DSCSR` reader"]
pub type R = crate::R<DSCSR_SPEC>;
#[doc = "Register `DSCSR` writer"]
pub type W = crate::W<DSCSR_SPEC>;
#[doc = "Field `SBRSELEN` reader - Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SBRSELEN_R = crate::BitReader;
#[doc = "Field `SBRSELEN` writer - Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SBRSELEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBRSEL` reader - If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SBRSEL_R = crate::BitReader;
#[doc = "Field `SBRSEL` writer - If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SBRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDS` reader - This field indicates the current Security state of the processor"]
pub type CDS_R = crate::BitReader;
#[doc = "Field `CDS` writer - This field indicates the current Security state of the processor"]
pub type CDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSKEY` reader - Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CDSKEY_R = crate::BitReader;
#[doc = "Field `CDSKEY` writer - Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CDSKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    pub fn sbrselen(&self) -> SBRSELEN_R {
        SBRSELEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    pub fn sbrsel(&self) -> SBRSEL_R {
        SBRSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - This field indicates the current Security state of the processor"]
    #[inline(always)]
    pub fn cds(&self) -> CDS_R {
        CDS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    pub fn cdskey(&self) -> CDSKEY_R {
        CDSKEY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrselen(&mut self) -> SBRSELEN_W<DSCSR_SPEC> {
        SBRSELEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsel(&mut self) -> SBRSEL_W<DSCSR_SPEC> {
        SBRSEL_W::new(self, 1)
    }
    #[doc = "Bit 16 - This field indicates the current Security state of the processor"]
    #[inline(always)]
    #[must_use]
    pub fn cds(&mut self) -> CDS_W<DSCSR_SPEC> {
        CDS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cdskey(&mut self) -> CDSKEY_W<DSCSR_SPEC> {
        CDSKEY_W::new(self, 17)
    }
}
#[doc = "Provides control and status information for Secure debug  

You can [`read`](crate::Reg::read) this register and get [`dscsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCSR_SPEC;
impl crate::RegisterSpec for DSCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscsr::R`](R) reader structure"]
impl crate::Readable for DSCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dscsr::W`](W) writer structure"]
impl crate::Writable for DSCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCSR to value 0"]
impl crate::Resettable for DSCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
