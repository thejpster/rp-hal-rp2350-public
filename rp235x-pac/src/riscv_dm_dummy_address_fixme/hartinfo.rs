#[doc = "Register `HARTINFO` reader"]
pub type R = crate::R<HARTINFO_SPEC>;
#[doc = "Register `HARTINFO` writer"]
pub type W = crate::W<HARTINFO_SPEC>;
#[doc = "Field `DATAADDR` reader - If dataaccess is 0: The number of the first CSR dedicated to shadowing the data registers.  

 If dataaccess is 1: Signed address of RAM where the data registers are shadowed, to be used to access relative to zero  

 On Hazard3 this indicates the single data register mapped as dmdata0. There is actually only a single shared register, internal to the Debug Module, and mirrored in each core's CSR space."]
pub type DATAADDR_R = crate::FieldReader<u16>;
#[doc = "Field `DATAADDR` writer - If dataaccess is 0: The number of the first CSR dedicated to shadowing the data registers.  

 If dataaccess is 1: Signed address of RAM where the data registers are shadowed, to be used to access relative to zero  

 On Hazard3 this indicates the single data register mapped as dmdata0. There is actually only a single shared register, internal to the Debug Module, and mirrored in each core's CSR space."]
pub type DATAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DATASIZE` reader - If dataaccess is 0: Number of CSRs dedicated to shadowing the data registers.  

 If dataaccess is 1: Number of 32-bit words in the memory map dedicated to shadowing the data registers."]
pub type DATASIZE_R = crate::FieldReader;
#[doc = "Field `DATASIZE` writer - If dataaccess is 0: Number of CSRs dedicated to shadowing the data registers.  

 If dataaccess is 1: Number of 32-bit words in the memory map dedicated to shadowing the data registers."]
pub type DATASIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAACCESS` reader - 0: The data registers are shadowed in the hart by CSRs. Each CSR is DXLEN bits in size, and corresponds to a single argument.  

 1: The data registers are shadowed in the hart’s memory map. Each register takes up 4 bytes in the memory map."]
pub type DATAACCESS_R = crate::BitReader;
#[doc = "Field `NSCRATCH` reader - Number of dscratch registers available for the debugger to use during program buffer execution, starting from dscratch0. The debugger can make no assumptions about the contents of these registers between commands."]
pub type NSCRATCH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - If dataaccess is 0: The number of the first CSR dedicated to shadowing the data registers.  

 If dataaccess is 1: Signed address of RAM where the data registers are shadowed, to be used to access relative to zero  

 On Hazard3 this indicates the single data register mapped as dmdata0. There is actually only a single shared register, internal to the Debug Module, and mirrored in each core's CSR space."]
    #[inline(always)]
    pub fn dataaddr(&self) -> DATAADDR_R {
        DATAADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - If dataaccess is 0: Number of CSRs dedicated to shadowing the data registers.  

 If dataaccess is 1: Number of 32-bit words in the memory map dedicated to shadowing the data registers."]
    #[inline(always)]
    pub fn datasize(&self) -> DATASIZE_R {
        DATASIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 0: The data registers are shadowed in the hart by CSRs. Each CSR is DXLEN bits in size, and corresponds to a single argument.  

 1: The data registers are shadowed in the hart’s memory map. Each register takes up 4 bytes in the memory map."]
    #[inline(always)]
    pub fn dataaccess(&self) -> DATAACCESS_R {
        DATAACCESS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of dscratch registers available for the debugger to use during program buffer execution, starting from dscratch0. The debugger can make no assumptions about the contents of these registers between commands."]
    #[inline(always)]
    pub fn nscratch(&self) -> NSCRATCH_R {
        NSCRATCH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - If dataaccess is 0: The number of the first CSR dedicated to shadowing the data registers.  

 If dataaccess is 1: Signed address of RAM where the data registers are shadowed, to be used to access relative to zero  

 On Hazard3 this indicates the single data register mapped as dmdata0. There is actually only a single shared register, internal to the Debug Module, and mirrored in each core's CSR space."]
    #[inline(always)]
    #[must_use]
    pub fn dataaddr(&mut self) -> DATAADDR_W<HARTINFO_SPEC> {
        DATAADDR_W::new(self, 0)
    }
    #[doc = "Bits 12:15 - If dataaccess is 0: Number of CSRs dedicated to shadowing the data registers.  

 If dataaccess is 1: Number of 32-bit words in the memory map dedicated to shadowing the data registers."]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DATASIZE_W<HARTINFO_SPEC> {
        DATASIZE_W::new(self, 12)
    }
}
#[doc = "This register gives information about the hart currently selected by hartsel.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`hartinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hartinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HARTINFO_SPEC;
impl crate::RegisterSpec for HARTINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hartinfo::R`](R) reader structure"]
impl crate::Readable for HARTINFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hartinfo::W`](W) writer structure"]
impl crate::Writable for HARTINFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HARTINFO to value 0x1bff"]
impl crate::Resettable for HARTINFO_SPEC {
    const RESET_VALUE: u32 = 0x1bff;
}
