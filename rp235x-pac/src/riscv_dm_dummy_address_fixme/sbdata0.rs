#[doc = "Register `SBDATA0` reader"]
pub type R = crate::R<SBDATA0_SPEC>;
#[doc = "Register `SBDATA0` writer"]
pub type W = crate::W<SBDATA0_SPEC>;
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SBDATA0_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "System Bus Data 31:0  

 Any successful system bus read updates sbdata. If the width of the read access is less than the width of sbdata, the contents of the remaining high bits may take on any value.  

 If sberror or sbbusyerror both aren’t 0 then accesses do nothing.  

 If the bus master is busy then accesses set sbbusyerror, and don’t do anything else. Writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus write of the new value of sbdata to sbaddress.  

 3. If the write succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

 Reads from this register start the following:  

 1. “Return” the data.  

 2. Set sbbusy.  

 3. If sbreadondata is set, perform a system bus read from the address contained in sbaddress, placing the result in sbdata.  

 4. If the read was successful, and sbautoincrement is set, increment sbaddress.  

 5. Clear sbbusy.  

You can [`read`](crate::Reg::read) this register and get [`sbdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBDATA0_SPEC;
impl crate::RegisterSpec for SBDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbdata0::R`](R) reader structure"]
impl crate::Readable for SBDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbdata0::W`](W) writer structure"]
impl crate::Writable for SBDATA0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBDATA0 to value 0"]
impl crate::Resettable for SBDATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
