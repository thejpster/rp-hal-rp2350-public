#[doc = "Register `SBPI_STATUS` reader"]
pub type R = crate::R<SBPI_STATUS_SPEC>;
#[doc = "Register `SBPI_STATUS` writer"]
pub type W = crate::W<SBPI_STATUS_SPEC>;
#[doc = "Field `RDATA_VLD` reader - Read command has returned data"]
pub type RDATA_VLD_R = crate::BitReader;
#[doc = "Field `RDATA_VLD` writer - Read command has returned data"]
pub type RDATA_VLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INSTR_DONE` reader - Last instruction done"]
pub type INSTR_DONE_R = crate::BitReader;
#[doc = "Field `INSTR_DONE` writer - Last instruction done"]
pub type INSTR_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INSTR_MISS` reader - Last instruction missed (dropped), as the previous has not finished running"]
pub type INSTR_MISS_R = crate::BitReader;
#[doc = "Field `INSTR_MISS` writer - Last instruction missed (dropped), as the previous has not finished running"]
pub type INSTR_MISS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLAG` reader - SBPI flag"]
pub type FLAG_R = crate::BitReader;
#[doc = "Field `MISO` reader - SBPI MISO (master in - slave out): response from SBPI"]
pub type MISO_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Read command has returned data"]
    #[inline(always)]
    pub fn rdata_vld(&self) -> RDATA_VLD_R {
        RDATA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Last instruction done"]
    #[inline(always)]
    pub fn instr_done(&self) -> INSTR_DONE_R {
        INSTR_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Last instruction missed (dropped), as the previous has not finished running"]
    #[inline(always)]
    pub fn instr_miss(&self) -> INSTR_MISS_R {
        INSTR_MISS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SBPI flag"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - SBPI MISO (master in - slave out): response from SBPI"]
    #[inline(always)]
    pub fn miso(&self) -> MISO_R {
        MISO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read command has returned data"]
    #[inline(always)]
    #[must_use]
    pub fn rdata_vld(&mut self) -> RDATA_VLD_W<SBPI_STATUS_SPEC> {
        RDATA_VLD_W::new(self, 0)
    }
    #[doc = "Bit 4 - Last instruction done"]
    #[inline(always)]
    #[must_use]
    pub fn instr_done(&mut self) -> INSTR_DONE_W<SBPI_STATUS_SPEC> {
        INSTR_DONE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Last instruction missed (dropped), as the previous has not finished running"]
    #[inline(always)]
    #[must_use]
    pub fn instr_miss(&mut self) -> INSTR_MISS_W<SBPI_STATUS_SPEC> {
        INSTR_MISS_W::new(self, 8)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`sbpi_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_STATUS_SPEC;
impl crate::RegisterSpec for SBPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_status::R`](R) reader structure"]
impl crate::Readable for SBPI_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbpi_status::W`](W) writer structure"]
impl crate::Writable for SBPI_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0111;
}
#[doc = "`reset()` method sets SBPI_STATUS to value 0"]
impl crate::Resettable for SBPI_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
