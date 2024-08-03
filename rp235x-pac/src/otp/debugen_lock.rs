#[doc = "Register `DEBUGEN_LOCK` reader"]
pub type R = crate::R<DEBUGEN_LOCK_SPEC>;
#[doc = "Register `DEBUGEN_LOCK` writer"]
pub type W = crate::W<DEBUGEN_LOCK_SPEC>;
#[doc = "Field `PROC0` reader - Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `PROC0` writer - Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
pub type PROC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC0_SECURE` reader - Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
pub type PROC0_SECURE_R = crate::BitReader;
#[doc = "Field `PROC0_SECURE` writer - Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
pub type PROC0_SECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC1` reader - Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
pub type PROC1_R = crate::BitReader;
#[doc = "Field `PROC1` writer - Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
pub type PROC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC1_SECURE` reader - Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
pub type PROC1_SECURE_R = crate::BitReader;
#[doc = "Field `PROC1_SECURE` writer - Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
pub type PROC1_SECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
pub type MISC_R = crate::BitReader;
#[doc = "Field `MISC` writer - Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
pub type MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn proc0_secure(&self) -> PROC0_SECURE_R {
        PROC0_SECURE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn proc1_secure(&self) -> PROC1_SECURE_R {
        PROC1_SECURE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<DEBUGEN_LOCK_SPEC> {
        PROC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    #[must_use]
    pub fn proc0_secure(&mut self) -> PROC0_SECURE_W<DEBUGEN_LOCK_SPEC> {
        PROC0_SECURE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<DEBUGEN_LOCK_SPEC> {
        PROC1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    #[must_use]
    pub fn proc1_secure(&mut self) -> PROC1_SECURE_W<DEBUGEN_LOCK_SPEC> {
        PROC1_SECURE_W::new(self, 3)
    }
    #[doc = "Bit 8 - Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MISC_W<DEBUGEN_LOCK_SPEC> {
        MISC_W::new(self, 8)
    }
}
#[doc = "Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset.  

You can [`read`](crate::Reg::read) this register and get [`debugen_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugen_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUGEN_LOCK_SPEC;
impl crate::RegisterSpec for DEBUGEN_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugen_lock::R`](R) reader structure"]
impl crate::Readable for DEBUGEN_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debugen_lock::W`](W) writer structure"]
impl crate::Writable for DEBUGEN_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUGEN_LOCK to value 0"]
impl crate::Resettable for DEBUGEN_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
