#[doc = "Register `DWT_CTRL` reader"]
pub type R = crate::R<DWT_CTRL_SPEC>;
#[doc = "Register `DWT_CTRL` writer"]
pub type W = crate::W<DWT_CTRL_SPEC>;
#[doc = "Field `CYCCNTENA` reader - Enables CYCCNT"]
pub type CYCCNTENA_R = crate::BitReader;
#[doc = "Field `CYCCNTENA` writer - Enables CYCCNT"]
pub type CYCCNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTPRESET` reader - Reload value for the POSTCNT counter"]
pub type POSTPRESET_R = crate::FieldReader;
#[doc = "Field `POSTPRESET` writer - Reload value for the POSTCNT counter"]
pub type POSTPRESET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POSTINIT` reader - Initial value for the POSTCNT counter"]
pub type POSTINIT_R = crate::FieldReader;
#[doc = "Field `POSTINIT` writer - Initial value for the POSTCNT counter"]
pub type POSTINIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CYCTAP` reader - Selects the position of the POSTCNT tap on the CYCCNT counter"]
pub type CYCTAP_R = crate::BitReader;
#[doc = "Field `CYCTAP` writer - Selects the position of the POSTCNT tap on the CYCCNT counter"]
pub type CYCTAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCTAP` reader - Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
pub type SYNCTAP_R = crate::FieldReader;
#[doc = "Field `SYNCTAP` writer - Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
pub type SYNCTAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCSAMPLENA` reader - Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
pub type PCSAMPLENA_R = crate::BitReader;
#[doc = "Field `PCSAMPLENA` writer - Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
pub type PCSAMPLENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRCENA` reader - Enables generation of Exception Trace packets"]
pub type EXTTRCENA_R = crate::BitReader;
#[doc = "Field `EXTTRCENA` writer - Enables generation of Exception Trace packets"]
pub type EXTTRCENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPIEVTENA` reader - Enables DWT_CPICNT counter"]
pub type CPIEVTENA_R = crate::BitReader;
#[doc = "Field `CPIEVTENA` writer - Enables DWT_CPICNT counter"]
pub type CPIEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCEVTENA` reader - Enables DWT_EXCCNT counter"]
pub type EXCEVTENA_R = crate::BitReader;
#[doc = "Field `EXCEVTENA` writer - Enables DWT_EXCCNT counter"]
pub type EXCEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPEVTENA` reader - Enable DWT_SLEEPCNT counter"]
pub type SLEEPEVTENA_R = crate::BitReader;
#[doc = "Field `SLEEPEVTENA` writer - Enable DWT_SLEEPCNT counter"]
pub type SLEEPEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSUEVTENA` reader - Enables DWT_LSUCNT counter"]
pub type LSUEVTENA_R = crate::BitReader;
#[doc = "Field `LSUEVTENA` writer - Enables DWT_LSUCNT counter"]
pub type LSUEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOLDEVTENA` reader - Enables DWT_FOLDCNT counter"]
pub type FOLDEVTENA_R = crate::BitReader;
#[doc = "Field `FOLDEVTENA` writer - Enables DWT_FOLDCNT counter"]
pub type FOLDEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCEVTENA` reader - Enables Event Counter packet generation on POSTCNT underflow"]
pub type CYCEVTENA_R = crate::BitReader;
#[doc = "Field `CYCEVTENA` writer - Enables Event Counter packet generation on POSTCNT underflow"]
pub type CYCEVTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCDISS` reader - Controls whether the cycle counter is disabled in Secure state"]
pub type CYCDISS_R = crate::BitReader;
#[doc = "Field `CYCDISS` writer - Controls whether the cycle counter is disabled in Secure state"]
pub type CYCDISS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOPRFCNT` reader - Indicates whether the implementation does not include the profiling counters"]
pub type NOPRFCNT_R = crate::BitReader;
#[doc = "Field `NOCYCCNT` reader - Indicates whether the implementation does not include a cycle counter"]
pub type NOCYCCNT_R = crate::BitReader;
#[doc = "Field `NOEXTTRIG` reader - Reserved, RAZ"]
pub type NOEXTTRIG_R = crate::BitReader;
#[doc = "Field `NOTRCPKT` reader - Indicates whether the implementation does not support trace"]
pub type NOTRCPKT_R = crate::BitReader;
#[doc = "Field `NUMCOMP` reader - Number of DWT comparators implemented"]
pub type NUMCOMP_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enables CYCCNT"]
    #[inline(always)]
    pub fn cyccntena(&self) -> CYCCNTENA_R {
        CYCCNTENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Reload value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postpreset(&self) -> POSTPRESET_R {
        POSTPRESET_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Initial value for the POSTCNT counter"]
    #[inline(always)]
    pub fn postinit(&self) -> POSTINIT_R {
        POSTINIT_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Selects the position of the POSTCNT tap on the CYCCNT counter"]
    #[inline(always)]
    pub fn cyctap(&self) -> CYCTAP_R {
        CYCTAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
    #[inline(always)]
    pub fn synctap(&self) -> SYNCTAP_R {
        SYNCTAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
    #[inline(always)]
    pub fn pcsamplena(&self) -> PCSAMPLENA_R {
        PCSAMPLENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables generation of Exception Trace packets"]
    #[inline(always)]
    pub fn exttrcena(&self) -> EXTTRCENA_R {
        EXTTRCENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables DWT_CPICNT counter"]
    #[inline(always)]
    pub fn cpievtena(&self) -> CPIEVTENA_R {
        CPIEVTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables DWT_EXCCNT counter"]
    #[inline(always)]
    pub fn excevtena(&self) -> EXCEVTENA_R {
        EXCEVTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable DWT_SLEEPCNT counter"]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SLEEPEVTENA_R {
        SLEEPEVTENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables DWT_LSUCNT counter"]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LSUEVTENA_R {
        LSUEVTENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables DWT_FOLDCNT counter"]
    #[inline(always)]
    pub fn foldevtena(&self) -> FOLDEVTENA_R {
        FOLDEVTENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables Event Counter packet generation on POSTCNT underflow"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CYCEVTENA_R {
        CYCEVTENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls whether the cycle counter is disabled in Secure state"]
    #[inline(always)]
    pub fn cycdiss(&self) -> CYCDISS_R {
        CYCDISS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates whether the implementation does not include the profiling counters"]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NOPRFCNT_R {
        NOPRFCNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates whether the implementation does not include a cycle counter"]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NOCYCCNT_R {
        NOCYCCNT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved, RAZ"]
    #[inline(always)]
    pub fn noexttrig(&self) -> NOEXTTRIG_R {
        NOEXTTRIG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates whether the implementation does not support trace"]
    #[inline(always)]
    pub fn notrcpkt(&self) -> NOTRCPKT_R {
        NOTRCPKT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Number of DWT comparators implemented"]
    #[inline(always)]
    pub fn numcomp(&self) -> NUMCOMP_R {
        NUMCOMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables CYCCNT"]
    #[inline(always)]
    #[must_use]
    pub fn cyccntena(&mut self) -> CYCCNTENA_W<DWT_CTRL_SPEC> {
        CYCCNTENA_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Reload value for the POSTCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn postpreset(&mut self) -> POSTPRESET_W<DWT_CTRL_SPEC> {
        POSTPRESET_W::new(self, 1)
    }
    #[doc = "Bits 5:8 - Initial value for the POSTCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn postinit(&mut self) -> POSTINIT_W<DWT_CTRL_SPEC> {
        POSTINIT_W::new(self, 5)
    }
    #[doc = "Bit 9 - Selects the position of the POSTCNT tap on the CYCCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn cyctap(&mut self) -> CYCTAP_W<DWT_CTRL_SPEC> {
        CYCTAP_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Selects the position of the synchronization packet counter tap on the CYCCNT counter. This determines the Synchronization packet rate"]
    #[inline(always)]
    #[must_use]
    pub fn synctap(&mut self) -> SYNCTAP_W<DWT_CTRL_SPEC> {
        SYNCTAP_W::new(self, 10)
    }
    #[doc = "Bit 12 - Enables use of POSTCNT counter as a timer for Periodic PC Sample packet generation"]
    #[inline(always)]
    #[must_use]
    pub fn pcsamplena(&mut self) -> PCSAMPLENA_W<DWT_CTRL_SPEC> {
        PCSAMPLENA_W::new(self, 12)
    }
    #[doc = "Bit 16 - Enables generation of Exception Trace packets"]
    #[inline(always)]
    #[must_use]
    pub fn exttrcena(&mut self) -> EXTTRCENA_W<DWT_CTRL_SPEC> {
        EXTTRCENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enables DWT_CPICNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn cpievtena(&mut self) -> CPIEVTENA_W<DWT_CTRL_SPEC> {
        CPIEVTENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enables DWT_EXCCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn excevtena(&mut self) -> EXCEVTENA_W<DWT_CTRL_SPEC> {
        EXCEVTENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable DWT_SLEEPCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn sleepevtena(&mut self) -> SLEEPEVTENA_W<DWT_CTRL_SPEC> {
        SLEEPEVTENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - Enables DWT_LSUCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn lsuevtena(&mut self) -> LSUEVTENA_W<DWT_CTRL_SPEC> {
        LSUEVTENA_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enables DWT_FOLDCNT counter"]
    #[inline(always)]
    #[must_use]
    pub fn foldevtena(&mut self) -> FOLDEVTENA_W<DWT_CTRL_SPEC> {
        FOLDEVTENA_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enables Event Counter packet generation on POSTCNT underflow"]
    #[inline(always)]
    #[must_use]
    pub fn cycevtena(&mut self) -> CYCEVTENA_W<DWT_CTRL_SPEC> {
        CYCEVTENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - Controls whether the cycle counter is disabled in Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn cycdiss(&mut self) -> CYCDISS_W<DWT_CTRL_SPEC> {
        CYCDISS_W::new(self, 23)
    }
}
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit  

You can [`read`](crate::Reg::read) this register and get [`dwt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_CTRL_SPEC;
impl crate::RegisterSpec for DWT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_ctrl::R`](R) reader structure"]
impl crate::Readable for DWT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_ctrl::W`](W) writer structure"]
impl crate::Writable for DWT_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_CTRL to value 0x7374_1824"]
impl crate::Resettable for DWT_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x7374_1824;
}
