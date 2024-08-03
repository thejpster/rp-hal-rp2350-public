#[doc = "Register `SBADDRESS0` reader"]
pub type R = crate::R<SBADDRESS0_SPEC>;
#[doc = "Register `SBADDRESS0` writer"]
pub type W = crate::W<SBADDRESS0_SPEC>;
#[doc = "Field `ADDRESS` reader - Accesses bits 31:0 of the physical address in sbaddress."]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Accesses bits 31:0 of the physical address in sbaddress."]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 31:0 of the physical address in sbaddress."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Accesses bits 31:0 of the physical address in sbaddress."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<SBADDRESS0_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "System Bus Address 31:0  

 When the system bus master is busy, writes to this register will set sbbusyerror and don’t do anything else.  

 If sberror is 0, sbbusyerror is 0, and sbreadonaddr is set then writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus read from the new value of sbaddress.  

 3. If the read succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

You can [`read`](crate::Reg::read) this register and get [`sbaddress0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBADDRESS0_SPEC;
impl crate::RegisterSpec for SBADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbaddress0::R`](R) reader structure"]
impl crate::Readable for SBADDRESS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbaddress0::W`](W) writer structure"]
impl crate::Writable for SBADDRESS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBADDRESS0 to value 0"]
impl crate::Resettable for SBADDRESS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
