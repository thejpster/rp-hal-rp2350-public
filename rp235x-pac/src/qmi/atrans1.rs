#[doc = "Register `ATRANS1` reader"]
pub type R = crate::R<ATRANS1_SPEC>;
#[doc = "Register `ATRANS1` writer"]
pub type W = crate::W<ATRANS1_SPEC>;
#[doc = "Field `BASE` reader - Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
pub type BASE_R = crate::FieldReader<u16>;
#[doc = "Field `BASE` writer - Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SIZE` reader - Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
pub type SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:11 - Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:26 - Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<ATRANS1_SPEC> {
        BASE_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<ATRANS1_SPEC> {
        SIZE_W::new(self, 16)
    }
}
#[doc = "Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.  

You can [`read`](crate::Reg::read) this register and get [`atrans1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atrans1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATRANS1_SPEC;
impl crate::RegisterSpec for ATRANS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atrans1::R`](R) reader structure"]
impl crate::Readable for ATRANS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atrans1::W`](W) writer structure"]
impl crate::Writable for ATRANS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATRANS1 to value 0x0400_0400"]
impl crate::Resettable for ATRANS1_SPEC {
    const RESET_VALUE: u32 = 0x0400_0400;
}
