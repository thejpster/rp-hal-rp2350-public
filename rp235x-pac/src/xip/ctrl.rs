#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN_SECURE` reader - When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit. There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled."]
pub type EN_SECURE_R = crate::BitReader;
#[doc = "Field `EN_SECURE` writer - When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit. There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled."]
pub type EN_SECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_NONSECURE` reader - When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit."]
pub type EN_NONSECURE_R = crate::BitReader;
#[doc = "Field `EN_NONSECURE` writer - When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit."]
pub type EN_NONSECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWER_DOWN` reader - When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down."]
pub type POWER_DOWN_R = crate::BitReader;
#[doc = "Field `POWER_DOWN` writer - When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down."]
pub type POWER_DOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_UNCACHED_SEC` reader - When 1, Secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
pub type NO_UNCACHED_SEC_R = crate::BitReader;
#[doc = "Field `NO_UNCACHED_SEC` writer - When 1, Secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
pub type NO_UNCACHED_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_UNCACHED_NONSEC` reader - When 1, Non-secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
pub type NO_UNCACHED_NONSEC_R = crate::BitReader;
#[doc = "Field `NO_UNCACHED_NONSEC` writer - When 1, Non-secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
pub type NO_UNCACHED_NONSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_UNTRANSLATED_SEC` reader - When 1, Secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
pub type NO_UNTRANSLATED_SEC_R = crate::BitReader;
#[doc = "Field `NO_UNTRANSLATED_SEC` writer - When 1, Secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
pub type NO_UNTRANSLATED_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_UNTRANSLATED_NONSEC` reader - When 1, Non-secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
pub type NO_UNTRANSLATED_NONSEC_R = crate::BitReader;
#[doc = "Field `NO_UNTRANSLATED_NONSEC` writer - When 1, Non-secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
pub type NO_UNTRANSLATED_NONSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAINT_NONSEC` reader - When 0, Non-secure accesses to the cache maintenance address window (addr\\[27\\]
== 1, addr\\[26\\]
== 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window. Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache. Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code."]
pub type MAINT_NONSEC_R = crate::BitReader;
#[doc = "Field `MAINT_NONSEC` writer - When 0, Non-secure accesses to the cache maintenance address window (addr\\[27\\]
== 1, addr\\[26\\]
== 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window. Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache. Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code."]
pub type MAINT_NONSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLIT_WAYS` reader - When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache. This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution. A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation."]
pub type SPLIT_WAYS_R = crate::BitReader;
#[doc = "Field `SPLIT_WAYS` writer - When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache. This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution. A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation."]
pub type SPLIT_WAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITABLE_M0` reader - If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
pub type WRITABLE_M0_R = crate::BitReader;
#[doc = "Field `WRITABLE_M0` writer - If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
pub type WRITABLE_M0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITABLE_M1` reader - If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
pub type WRITABLE_M1_R = crate::BitReader;
#[doc = "Field `WRITABLE_M1` writer - If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
pub type WRITABLE_M1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit. There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled."]
    #[inline(always)]
    pub fn en_secure(&self) -> EN_SECURE_R {
        EN_SECURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit."]
    #[inline(always)]
    pub fn en_nonsecure(&self) -> EN_NONSECURE_R {
        EN_NONSECURE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, Secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
    #[inline(always)]
    pub fn no_uncached_sec(&self) -> NO_UNCACHED_SEC_R {
        NO_UNCACHED_SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, Non-secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
    #[inline(always)]
    pub fn no_uncached_nonsec(&self) -> NO_UNCACHED_NONSEC_R {
        NO_UNCACHED_NONSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, Secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
    #[inline(always)]
    pub fn no_untranslated_sec(&self) -> NO_UNTRANSLATED_SEC_R {
        NO_UNTRANSLATED_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, Non-secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
    #[inline(always)]
    pub fn no_untranslated_nonsec(&self) -> NO_UNTRANSLATED_NONSEC_R {
        NO_UNTRANSLATED_NONSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 0, Non-secure accesses to the cache maintenance address window (addr\\[27\\]
== 1, addr\\[26\\]
== 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window. Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache. Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code."]
    #[inline(always)]
    pub fn maint_nonsec(&self) -> MAINT_NONSEC_R {
        MAINT_NONSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache. This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution. A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation."]
    #[inline(always)]
    pub fn split_ways(&self) -> SPLIT_WAYS_R {
        SPLIT_WAYS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
    #[inline(always)]
    pub fn writable_m0(&self) -> WRITABLE_M0_R {
        WRITABLE_M0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
    #[inline(always)]
    pub fn writable_m1(&self) -> WRITABLE_M1_R {
        WRITABLE_M1_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit. There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en_secure(&mut self) -> EN_SECURE_W<CTRL_SPEC> {
        EN_SECURE_W::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr\\[26\\]
== 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface. Accesses to the uncached (addr\\[26\\]
== 1) window will never query the cache, irrespective of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn en_nonsecure(&mut self) -> EN_NONSECURE_W<CTRL_SPEC> {
        EN_NONSECURE_W::new(self, 1)
    }
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down."]
    #[inline(always)]
    #[must_use]
    pub fn power_down(&mut self) -> POWER_DOWN_W<CTRL_SPEC> {
        POWER_DOWN_W::new(self, 3)
    }
    #[doc = "Bit 4 - When 1, Secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
    #[inline(always)]
    #[must_use]
    pub fn no_uncached_sec(&mut self) -> NO_UNCACHED_SEC_W<CTRL_SPEC> {
        NO_UNCACHED_SEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - When 1, Non-secure accesses to the uncached window (addr\\[27:26\\]
== 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents. Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC."]
    #[inline(always)]
    #[must_use]
    pub fn no_uncached_nonsec(&mut self) -> NO_UNCACHED_NONSEC_W<CTRL_SPEC> {
        NO_UNCACHED_NONSEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - When 1, Secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
    #[inline(always)]
    #[must_use]
    pub fn no_untranslated_sec(&mut self) -> NO_UNTRANSLATED_SEC_W<CTRL_SPEC> {
        NO_UNTRANSLATED_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - When 1, Non-secure accesses to the uncached, untranslated window (addr\\[27:26\\]
== 3) will generate a bus error."]
    #[inline(always)]
    #[must_use]
    pub fn no_untranslated_nonsec(&mut self) -> NO_UNTRANSLATED_NONSEC_W<CTRL_SPEC> {
        NO_UNTRANSLATED_NONSEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - When 0, Non-secure accesses to the cache maintenance address window (addr\\[27\\]
== 1, addr\\[26\\]
== 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window. Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache. Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code."]
    #[inline(always)]
    #[must_use]
    pub fn maint_nonsec(&mut self) -> MAINT_NONSEC_W<CTRL_SPEC> {
        MAINT_NONSEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache. This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution. A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation."]
    #[inline(always)]
    #[must_use]
    pub fn split_ways(&mut self) -> SPLIT_WAYS_W<CTRL_SPEC> {
        SPLIT_WAYS_W::new(self, 9)
    }
    #[doc = "Bit 10 - If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
    #[inline(always)]
    #[must_use]
    pub fn writable_m0(&mut self) -> WRITABLE_M0_W<CTRL_SPEC> {
        WRITABLE_M0_W::new(self, 10)
    }
    #[doc = "Bit 11 - If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only. XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1. The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour. Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address. Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect."]
    #[inline(always)]
    #[must_use]
    pub fn writable_m1(&mut self) -> WRITABLE_M1_W<CTRL_SPEC> {
        WRITABLE_M1_W::new(self, 11)
    }
}
#[doc = "Cache control register. Read-only from a Non-secure context.  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x83"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
