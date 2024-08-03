#[doc = "Register `MEIPA` reader"]
pub type R = crate::R<MEIPA_SPEC>;
#[doc = "Register `MEIPA` writer"]
pub type W = crate::W<MEIPA_SPEC>;
#[doc = "Field `INDEX` writer - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WINDOW` reader - 16-bit read-only window into the external interrupt pending array"]
pub type WINDOW_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:31 - 16-bit read-only window into the external interrupt pending array"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<MEIPA_SPEC> {
        INDEX_W::new(self, 0)
    }
}
#[doc = "External interrupt pending array  

 Contains a read-only bit for each external interrupt request. Similarly to `meiea`, this register is a window into an array of up to 512 external interrupt flags. The status appears in the upper 16 bits of the value read from `meipa`, and the lower 5 bits of the value _written_ by the same CSR instruction (or 0 if no write takes place) select a 16-bit window of the full interrupt pending array.  

 A `1` bit indicates that interrupt is currently asserted. IRQs are assumed to be level-sensitive, and the relevant `meipa` bit is cleared by servicing the requestor so that it deasserts its interrupt request.  

 When any interrupt of sufficient priority is both set in `meipa` and enabled in `meiea`, the standard RISC-V external interrupt pending bit `mip.meip` is asserted. In other words, `meipa` is filtered by `meiea` to generate the standard `mip.meip` flag.  

You can [`read`](crate::Reg::read) this register and get [`meipa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meipa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEIPA_SPEC;
impl crate::RegisterSpec for MEIPA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meipa::R`](R) reader structure"]
impl crate::Readable for MEIPA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meipa::W`](W) writer structure"]
impl crate::Writable for MEIPA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEIPA to value 0"]
impl crate::Resettable for MEIPA_SPEC {
    const RESET_VALUE: u32 = 0;
}
