#[doc = "Register `MCOUNTINHIBIT` reader"]
pub type R = crate::R<MCOUNTINHIBIT_SPEC>;
#[doc = "Register `MCOUNTINHIBIT` writer"]
pub type W = crate::W<MCOUNTINHIBIT_SPEC>;
#[doc = "Field `CY` reader - Inhibit counting of the `mcycle` and `mcycleh` registers. Set by default to save power."]
pub type CY_R = crate::BitReader;
#[doc = "Field `CY` writer - Inhibit counting of the `mcycle` and `mcycleh` registers. Set by default to save power."]
pub type CY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR` reader - Inhibit counting of the `minstret` and `minstreth` registers. Set by default to save power."]
pub type IR_R = crate::BitReader;
#[doc = "Field `IR` writer - Inhibit counting of the `minstret` and `minstreth` registers. Set by default to save power."]
pub type IR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Inhibit counting of the `mcycle` and `mcycleh` registers. Set by default to save power."]
    #[inline(always)]
    pub fn cy(&self) -> CY_R {
        CY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inhibit counting of the `minstret` and `minstreth` registers. Set by default to save power."]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inhibit counting of the `mcycle` and `mcycleh` registers. Set by default to save power."]
    #[inline(always)]
    #[must_use]
    pub fn cy(&mut self) -> CY_W<MCOUNTINHIBIT_SPEC> {
        CY_W::new(self, 0)
    }
    #[doc = "Bit 2 - Inhibit counting of the `minstret` and `minstreth` registers. Set by default to save power."]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IR_W<MCOUNTINHIBIT_SPEC> {
        IR_W::new(self, 2)
    }
}
#[doc = "Count inhibit register for `mcycle`/`minstret`  

You can [`read`](crate::Reg::read) this register and get [`mcountinhibit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcountinhibit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCOUNTINHIBIT_SPEC;
impl crate::RegisterSpec for MCOUNTINHIBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcountinhibit::R`](R) reader structure"]
impl crate::Readable for MCOUNTINHIBIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcountinhibit::W`](W) writer structure"]
impl crate::Writable for MCOUNTINHIBIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCOUNTINHIBIT to value 0x05"]
impl crate::Resettable for MCOUNTINHIBIT_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
