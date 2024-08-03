#[doc = "Register `MSTATUS` reader"]
pub type R = crate::R<MSTATUS_SPEC>;
#[doc = "Register `MSTATUS` writer"]
pub type W = crate::W<MSTATUS_SPEC>;
#[doc = "Field `MIE` reader - Interrupt enable. Readable and writable. Is set to 0 on trap entry. Is set to the current value of `mstatus.mpie` on trap return."]
pub type MIE_R = crate::BitReader;
#[doc = "Field `MIE` writer - Interrupt enable. Readable and writable. Is set to 0 on trap entry. Is set to the current value of `mstatus.mpie` on trap return."]
pub type MIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPIE` reader - Previous interrupt enable. Readable and writable. Is set to the current value of `mstatus.mie` on trap entry. Is set to 1 on trap return."]
pub type MPIE_R = crate::BitReader;
#[doc = "Field `MPIE` writer - Previous interrupt enable. Readable and writable. Is set to the current value of `mstatus.mie` on trap entry. Is set to 1 on trap return."]
pub type MPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPP` reader - Previous privilege level. Can store the values 3 (M-mode) or 0 (U-mode). If another value is written, hardware rounds to the nearest supported mode."]
pub type MPP_R = crate::FieldReader;
#[doc = "Field `MPP` writer - Previous privilege level. Can store the values 3 (M-mode) or 0 (U-mode). If another value is written, hardware rounds to the nearest supported mode."]
pub type MPP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MPRV` reader - Modify privilege. If 1, loads and stores behave as though the current privilege level were `mpp`. This includes physical memory protection checks, and the privilege level asserted on the system bus alongside the load/store address."]
pub type MPRV_R = crate::BitReader;
#[doc = "Field `MPRV` writer - Modify privilege. If 1, loads and stores behave as though the current privilege level were `mpp`. This includes physical memory protection checks, and the privilege level asserted on the system bus alongside the load/store address."]
pub type MPRV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TW` reader - Timeout wait. When 1, attempting to execute a WFI instruction in U-mode will instantly cause an illegal instruction exception."]
pub type TW_R = crate::BitReader;
#[doc = "Field `TW` writer - Timeout wait. When 1, attempting to execute a WFI instruction in U-mode will instantly cause an illegal instruction exception."]
pub type TW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Interrupt enable. Readable and writable. Is set to 0 on trap entry. Is set to the current value of `mstatus.mpie` on trap return."]
    #[inline(always)]
    pub fn mie(&self) -> MIE_R {
        MIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Previous interrupt enable. Readable and writable. Is set to the current value of `mstatus.mie` on trap entry. Is set to 1 on trap return."]
    #[inline(always)]
    pub fn mpie(&self) -> MPIE_R {
        MPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Previous privilege level. Can store the values 3 (M-mode) or 0 (U-mode). If another value is written, hardware rounds to the nearest supported mode."]
    #[inline(always)]
    pub fn mpp(&self) -> MPP_R {
        MPP_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 17 - Modify privilege. If 1, loads and stores behave as though the current privilege level were `mpp`. This includes physical memory protection checks, and the privilege level asserted on the system bus alongside the load/store address."]
    #[inline(always)]
    pub fn mprv(&self) -> MPRV_R {
        MPRV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Timeout wait. When 1, attempting to execute a WFI instruction in U-mode will instantly cause an illegal instruction exception."]
    #[inline(always)]
    pub fn tw(&self) -> TW_R {
        TW_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Interrupt enable. Readable and writable. Is set to 0 on trap entry. Is set to the current value of `mstatus.mpie` on trap return."]
    #[inline(always)]
    #[must_use]
    pub fn mie(&mut self) -> MIE_W<MSTATUS_SPEC> {
        MIE_W::new(self, 3)
    }
    #[doc = "Bit 7 - Previous interrupt enable. Readable and writable. Is set to the current value of `mstatus.mie` on trap entry. Is set to 1 on trap return."]
    #[inline(always)]
    #[must_use]
    pub fn mpie(&mut self) -> MPIE_W<MSTATUS_SPEC> {
        MPIE_W::new(self, 7)
    }
    #[doc = "Bits 11:12 - Previous privilege level. Can store the values 3 (M-mode) or 0 (U-mode). If another value is written, hardware rounds to the nearest supported mode."]
    #[inline(always)]
    #[must_use]
    pub fn mpp(&mut self) -> MPP_W<MSTATUS_SPEC> {
        MPP_W::new(self, 11)
    }
    #[doc = "Bit 17 - Modify privilege. If 1, loads and stores behave as though the current privilege level were `mpp`. This includes physical memory protection checks, and the privilege level asserted on the system bus alongside the load/store address."]
    #[inline(always)]
    #[must_use]
    pub fn mprv(&mut self) -> MPRV_W<MSTATUS_SPEC> {
        MPRV_W::new(self, 17)
    }
    #[doc = "Bit 21 - Timeout wait. When 1, attempting to execute a WFI instruction in U-mode will instantly cause an illegal instruction exception."]
    #[inline(always)]
    #[must_use]
    pub fn tw(&mut self) -> TW_W<MSTATUS_SPEC> {
        TW_W::new(self, 21)
    }
}
#[doc = "Machine status register  

You can [`read`](crate::Reg::read) this register and get [`mstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTATUS_SPEC;
impl crate::RegisterSpec for MSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstatus::R`](R) reader structure"]
impl crate::Readable for MSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mstatus::W`](W) writer structure"]
impl crate::Writable for MSTATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTATUS to value 0x1800"]
impl crate::Resettable for MSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x1800;
}
