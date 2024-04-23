// Copyright 2024 Rivos, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use crate::types::*;
use crate::{simple_smbios_structure, Sink, SmbiosStructure, StringIndex};
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::size_of;
use paste::paste;
use zerocopy::{byteorder, byteorder::LE, AsBytes};

// SMBIOS structures little-endian
type U16 = byteorder::U16<LE>;
type U32 = byteorder::U32<LE>;
type U64 = byteorder::U64<LE>;
type U128 = byteorder::U128<LE>;

// A handle to another structure is always 16 bits wide
type StructureHandle = U16;

// Current spec revision
const SMBIOS_MAJOR: u8 = 3;
const SMBIOS_MINOR: u8 = 7;
const SMBIOS_DOCREV: u8 = 0;

// For some reason this is used to indicate SMBIOS 3+ in the entry point structure
const SMBIOS_REVISION: u8 = 1;

fn to_mb(n: u64) -> u64 {
    n >> 20
}

fn mb(n: u64) -> u64 {
    n << 20
}

fn gb(n: u64) -> u64 {
    n << 30
}

fn tb(n: u64) -> u64 {
    n << 40
}

// SMBIOS 3.0 64-bit Entry Point structure
#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug, AsBytes)]
pub struct EntryPoint {
    anchor: [u8; 5],
    checksum: u8,
    length: u8,
    major_version: u8,
    minor_version: u8,
    docrev: u8,
    revision: u8,
    _reserved: u8,
    structure_max_size: U32,
    table_address: U64,
}
static_assertions::const_assert!(size_of::<EntryPoint>() == 0x18);

impl EntryPoint {
    pub fn new(structure_max_size: u32, table_address: u64) -> Self {
        let mut s = Self {
            anchor: *b"_SM3_",
            checksum: 0,
            length: size_of::<Self>() as u8,
            major_version: SMBIOS_MAJOR,
            minor_version: SMBIOS_MINOR,
            docrev: SMBIOS_DOCREV,
            revision: SMBIOS_REVISION,
            _reserved: 0,
            structure_max_size: structure_max_size.into(),
            table_address: table_address.into(),
        };

        // Calculate checksum as the value that makes the sum of the structure zero.
        let mut sum: u8 = 0;
        for b in s.as_bytes() {
            sum = sum.wrapping_add(*b);
        }
        s.checksum = 0u8.wrapping_sub(sum);
        s
    }
}

impl SmbiosStructure for EntryPoint {
    fn serialize(&self, sink: &mut dyn Sink) {
        sink.vec(self.as_bytes());
    }
}

// Type 0 SMBIOS table (BIOS Information)
simple_smbios_structure! {
    0,
    struct BiosInformation {
        data: struct Type0Data {
            vendor: StringIndex,
            bios_version: StringIndex,
            bios_starting_address_segment: U16,
            bios_release_date: StringIndex,
            bios_rom_size: u8,
            bios_characteristics: U64,
            bios_characteristics_ex1: u8,
            bios_characteristics_ex2: u8,
        }
    }
}
static_assertions::const_assert!(size_of::<Type0Data>() == 0x14);

// Type 1 SMBIOS table (System Information)
simple_smbios_structure! {
    1,
    struct SystemInformation {
        data: struct Type1Data {
            manufacturer: StringIndex,
            product_name: StringIndex,
            version: StringIndex,
            serial_number: StringIndex,
            uuid: [u8; 16],
            wakeup_type: WakeupType,
            sku_number: StringIndex,
            family: StringIndex,
        }
    }
}
static_assertions::const_assert!(size_of::<Type1Data>() == 0x1b);

// Type 4 SMBIOS table (Processor Information)
simple_smbios_structure! {
    4,
    struct ProcessorInformation {
        data: struct Type4Data {
            socket_designation: StringIndex,
            processor_type: ProcessorType,
            processor_family: ProcessorFamily,
            processor_manufacturer: StringIndex,
            processor_id: U64, // mvendorid for risc-v
            processor_version: StringIndex,
            voltage: u8,
            external_clock: U16,
            max_speed: U16,
            current_speed: U16,
            status: u8,
            processor_ugprade: ProcessorUpgrade,
            l1_cache_handle: StructureHandle,
            l2_cache_handle: StructureHandle,
            l3_cache_handle: StructureHandle,
            serial_number: StringIndex,
            asset_tag: StringIndex,
            part_number: StringIndex,
            core_count: u8,
            core_enabled: u8,
            thread_count: u8,
            processor_characteristics: U16,
            processor_family2: ProcessorFamily2,
            core_count2: U16,
            core_enabled2: U16,
            thread_count2: U16,
            thread_enabled: U16,
        }
    }
}
static_assertions::const_assert!(size_of::<Type4Data>() == 0x32);

// Type 7 SMBIOS table (Cache Information)
simple_smbios_structure! {
    7,
    struct CacheInformation {
        data: struct Type7Data {
            socket_designation: StringIndex,
            cache_configuration: CacheConfiguration,
            maximum_cache_size: U16,
            installed_size: U16,
            supported_sram_type: SramType,
            current_sram_type: SramType,
            cache_speed: u8,
            error_correction_type: EccType,
            system_cache_type: SystemCacheType,
            associativity: Associativity,
            maximum_cache_size2: U32,
            installed_cache_size2: U32,
        }
    }
}
static_assertions::const_assert!(size_of::<Type7Data>() == 0x1b);

// Type 9 SMBIOS table (System Slots)
// Note: This does not include support for peer groups
simple_smbios_structure! {
    9,
    struct SystemSlots {
        data: struct Type9Data {
            slot_designation: StringIndex,
            slot_type: SlotType,
            slot_data_bus_width: SlotWidth,
            current_usage: CurrentUsage,
            slot_length: SlotLength,
            slot_id: U16,
            slot_characteristics1: u8,
            slot_characteristics2: u8,
            segment_group_number: U16,
            bus_number: u8,
            devfn: u8,
            data_bus_width: u8,
            peer_group_count: u8,
        }
    }
}
static_assertions::const_assert!(size_of::<Type9Data>() == 0x13);

// Type 11 SMBIOS table (OEM Strings)
pub struct OemStrings {
    handle: u16,
    strings: Vec<String>,
}

impl OemStrings {
    pub fn new(handle: u16) -> Self {
        Self {
            handle,
            strings: Vec::new(),
        }
    }

    pub fn add_string(&mut self, s: &str) {
        self.strings.push(s.into());
    }
}

impl SmbiosStructure for OemStrings {
    fn serialize(&self, sink: &mut dyn Sink) {
        sink.byte(11);
        sink.byte(5);
        sink.word(self.handle);
        sink.byte(self.strings.len().try_into().unwrap());
        for s in &self.strings {
            sink.vec(s.as_bytes());
            sink.byte(0);
        }
        sink.byte(0);
    }
}

// Type 16 SMBIOS table (Physical Memory Array)
simple_smbios_structure! {
    16,
    struct PhysicalMemoryArray {
        data: struct Type16Data {
            location: ArrayLocation,
            array_use: ArrayUse,
            memory_error_correction: ErrorCorrectionType,
            maximum_capacity: U32,
            error_information_handle: StructureHandle,
            number_of_memory_devices: U16,
            extended_maximum_capacity: U64,
        }
    }
}
static_assertions::const_assert!(size_of::<Type16Data>() == 0x17);

impl PhysicalMemoryArray {
    pub fn set_memory_capacity(&mut self, memory_capacity_bytes: u64) {
        if memory_capacity_bytes >= tb(2) {
            self.data.maximum_capacity = 0x8000_0000.into();
            self.data.extended_maximum_capacity = memory_capacity_bytes.into();
        } else {
            let cap_kb: u32 = (memory_capacity_bytes / 1024).try_into().unwrap();
            self.data.maximum_capacity = cap_kb.into();
            self.data.extended_maximum_capacity = 0.into();
        }
    }
}

// Type 17 SMBIOS table (Memory Device)
simple_smbios_structure! {
    17,
    struct MemoryDevice {
        data: struct Type17Data {
            physical_memory_array_handle: StructureHandle,
            error_information_handle: StructureHandle,
            total_width: U16,
            data_width: U16,
            size: U16,
            form_factor: FormFactor,
            device_set: u8,
            device_locator: StringIndex,
            bank_locator: StringIndex,
            memory_type: MemoryType,
            type_detail: U16,
            speed: U16,
            manufacturer: StringIndex,
            serial_number: StringIndex,
            asset_tag: StringIndex,
            part_number: StringIndex,
            attributes: u8,
            extended_size: U32,
            configured_memory_speed: U16,
            minimum_voltage: U16,
            maximum_voltage: U16,
            configured_voltage: U16,
            memory_technology: MemoryTechnology,
            memory_operating_mode: U16,
            firmware_version: StringIndex,
            module_manufacturer_id: U16,
            module_product_id: U16,
            memory_subsystem_controller_manufacturer_id: U16,
            memory_subsystem_controller_product_id: U16,
            non_volatile_size: U64,
            volatile_size: U64,
            cache_size: U64,
            logical_size: U64,
            extended_speed: U32,
            extended_configured_memory_speed: U32,
            pmic0_manufacturer_id: U16,
            pmic0_revision_number: U16,
            rcd_manufacturer_id: U16,
            rcd_revision_number: U16,
        }
    }
}
static_assertions::const_assert!(size_of::<Type17Data>() == 0x64);

impl MemoryDevice {
    pub fn set_memory_size(&mut self, size: Option<u64>) {
        match size {
            Some(size) => {
                if size >= gb(32) - mb(1) {
                    self.data.size = 0x7fff.into();
                    // Bits 30:0 represent the size of the memory device in megabytes
                    let size_mb: u32 = to_mb(size).try_into().unwrap();
                    self.data.extended_size = size_mb.into();
                } else {
                    // if bit 15 is 0, the granularity of this field is megabytes,
                    // otherwise (1) it is kilobytes.
                    let size_mb: u16 = to_mb(size).try_into().unwrap();
                    self.data.size = size_mb.into();
                }
            }
            None => self.data.size = 0xffff.into(),
        }
    }
}

// Type 19 SMBIOS table (Memory Array Mapped Address)
simple_smbios_structure! {
    19,
    struct MemoryArrayMappedAddress {
        data: struct Type19Data {
            starting_address: U32,
            ending_address: U32,
            memory_array_handle: StructureHandle,
            partition_width: u8,
            extended_starting_address: U64,
            extended_ending_address: U64,
        }
    }
}
static_assertions::const_assert!(size_of::<Type19Data>() == 0x1f);

impl MemoryArrayMappedAddress {
    pub fn set_address_range(&mut self, starting: u64, ending: u64) {
        self.data.starting_address = u32::MAX.into();
        self.data.ending_address = u32::MAX.into();
        self.data.extended_starting_address = starting.into();
        self.data.extended_ending_address = ending.into();
    }
}

// Type 20 SMBIOS table (Memory Device Mapped Address)
simple_smbios_structure! {
    20,
    struct MemoryDeviceMappedAddress {
        data: struct Type20Data {
            starting_address: U32,
            ending_address: U32,
            memory_device_handle: StructureHandle,
            memory_array_mapped_address_handle: StructureHandle,
            partition_row_position: PartitionRowPosition,
            interleave_position: u8,
            interleaved_data_depth: u8,
            extended_starting_address: U64,
            extended_ending_address: U64,
        }
    }
}
static_assertions::const_assert!(size_of::<Type20Data>() == 0x23);

impl MemoryDeviceMappedAddress {
    pub fn set_address_range(&mut self, starting: u64, ending: u64) {
        self.data.starting_address = u32::MAX.into();
        self.data.ending_address = u32::MAX.into();
        self.data.extended_starting_address = starting.into();
        self.data.extended_ending_address = ending.into();
    }
}

// Type 32 SMBIOS table (System Boot Information)
#[derive(Debug, Default)]
pub struct SystemBootInformation<'a> {
    handle: u16,
    status: BootStatus<'a>,
}

impl<'a> SystemBootInformation<'a> {
    pub fn new(handle: u16, status: BootStatus<'a>) -> Self {
        Self { handle, status }
    }
}

impl SmbiosStructure for SystemBootInformation<'_> {
    fn serialize(&self, sink: &mut dyn Sink) {
        let mut output = Vec::new();

        output.byte(32);
        output.byte(0); // length will be fixed up at the end
        output.word(self.handle);

        // 6 reserved bytes, all zero
        for _ in 0..6 {
            output.byte(0);
        }
        match &self.status {
            BootStatus::NoErrorsDetected => output.byte(0),
            BootStatus::NoBootableMedia => output.byte(1),
            BootStatus::NormalOperatingSystemFailedToLoad => output.byte(2),
            BootStatus::FirmwareDetectedHardwareFailure => output.byte(3),
            BootStatus::OperatingSystemDetectedHardwareFailure => output.byte(4),
            BootStatus::UserRequestedBoot => output.byte(5),
            BootStatus::SystemSecurityViolation => output.byte(6),
            BootStatus::PreviouslyRequestedImage(extra) => {
                output.byte(7);
                output.vec(extra);
            }
            BootStatus::SystemWatchdogTimer => output.byte(8),
            BootStatus::VendorSpecific(code, extra) => {
                assert!(*code >= 128 && *code <= 191);
                output.byte(*code);
                output.vec(extra);
            }
            BootStatus::ProductSpecific(code, extra) => {
                assert!(*code >= 192);
                output.byte(*code);
                output.vec(extra);
            }
        }

        // Fix up the length byte
        output[1] = output.len().try_into().unwrap();
        output.byte(0);
        output.byte(0);

        sink.vec(&output);
    }
}

// Type 43 SMBIOS table (TPM Device)
simple_smbios_structure! {
    43,
    struct TpmDevice {
        data: struct Type43Data {
            vendor_id: [u8; 4],
            major_spec_version: u8,
            minor_spec_version: u8,
            firmware_version1: U32,
            firmware_version2: U32,
            description: StringIndex,
            characteristics: U64,
            oem_defined: U32,
        }
    }
}

// Type 44 SMBIOS table (Processor Additional Information)
// See https://github.com/riscv/riscv-smbios/blob/main/riscv-smbios.adoc
simple_smbios_structure! {
    44,
    struct RiscvProcessorAdditionalInformation {
        data: struct RiscvType44Data {
            referenced_handle: StructureHandle, // type 4
            revision: U16,
            structure_length: u8,
            hart_id: U128,
            boot_hart: u8,
            mvendorid: U128,
            marchid: U128,
            mimplid: U128,
            isa_supported: U32,
            privilege_level_supported: u8,
            m_exception_delegation: U128,
            m_interrupt_delegation: U128,
            xlen: Xlen,
            mxlen: Xlen,
            reserved: u8,
            sxlen: Xlen,
            uxlen: Xlen,
        }
    }
}
static_assertions::const_assert!(size_of::<RiscvType44Data>() == 0x74);

simple_smbios_structure! {
    127,
    struct EndOfTable {
        data: struct Type127Data {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_type0() {
        let expected = [
            0u8, 0x14, 0x1, 0x1, 0x1, 0x2, 0, 0, 0x3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'S', b'y',
            b's', b't', b'e', b'm', b' ', b'B', b'I', b'O', b'S', b' ', b'V', b'e', b'n', b'd',
            b'o', b'r', 0, b'4', b'.', b'0', b'4', 0, b'0', b'0', b'/', b'0', b'0', b'/', b'0',
            b'0', b'0', b'0', 0, 0,
        ];

        let mut output = vec![];
        let mut b = BiosInformation::new(257);
        b.set_vendor("System BIOS Vendor");
        b.set_bios_version("4.04");
        b.set_bios_release_date("00/00/0000");

        b.serialize(&mut output);
        assert_eq!(expected.as_slice(), &output);
    }

    #[test]
    fn test_type1() {
        let expected = [
            1u8, 0x1b, 0xff, 0x0, 0x1, 0x2, 0x3, 0x4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 8, 0x5, 0x6, b'O', b'E', b'M', b'1', 0, b'R', b'i', b'v', b'o', b's', b' ', b's',
            b'y', b's', b't', b'e', b'm', 0, b'G', b'e', b'n', b'0', 0, b'0', b'1', b'2', b'3',
            b'4', b'5', 0, b'S', b'K', b'U', b'1', 0, b'F', b'a', b'm', b'i', b'l', b'y', 0, 0,
        ];

        let mut output = vec![];
        let mut b = SystemInformation::new(255);
        b.set_manufacturer("OEM1");
        b.set_product_name("Rivos system");
        b.set_version("Gen0");
        b.set_serial_number("012345");
        b.set_sku_number("SKU1");
        b.set_family("Family");
        b.set_wakeup_type(WakeupType::AcPowerRestored);

        b.serialize(&mut output);
        assert_eq!(expected.as_slice(), output);
    }

    #[test]
    fn test_type4() {
        let expected = [
            4u8, 0x32, 5, 0, 1, 3, 0xfe, 2, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12, 3, 0,
            1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0,
            0, 0, 0, b'S', b'o', b'c', b'k', b'e', b't', 0, b'M', b'a', b'n', b'u', b'f', 0, b'V',
            b'e', b'r', b's', b'i', b'o', b'n', 0, 0,
        ];

        let mut output = vec![];
        let mut p = ProcessorInformation::new(5);
        p.set_socket_designation("Socket");
        p.set_processor_type(ProcessorType::CentralProcessor);
        p.set_processor_manufacturer("Manuf");
        p.set_processor_family(ProcessorFamily::ObtainFrom2);
        p.set_processor_id(0x1234_5678_90ab_cdef.into());
        p.set_processor_version("Version");
        p.set_external_clock(1.into());
        p.set_processor_family2(ProcessorFamily2::RiscvRv64);

        p.serialize(&mut output);
        assert_eq!(expected.as_slice(), output);
    }

    #[test]
    fn test_type11() {
        let expected = [
            11, 5, 1, 0, 2, b'M', b'y', b' ', b'O', b'E', b'M', b' ', b's', b't', b'r', b'i', b'n',
            b'g', 0, b'f', b'o', b'o', 0, 0,
        ];

        let mut output = vec![];
        let mut o = OemStrings::new(1);
        o.add_string("My OEM string");
        o.add_string("foo");
        o.serialize(&mut output);

        assert_eq!(expected.as_slice(), output);
    }

    #[test]
    fn test_type16() {
        let expected = [
            16u8, 0x17, 10, 0, 3, 3, 2, 0, 0, 0, 128, 0, 0, 16, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
        ];

        let mut output = vec![];
        let mut p = PhysicalMemoryArray::new(10);
        p.set_location(ArrayLocation::SystemBoard);
        p.set_array_use(ArrayUse::SystemMemory);
        p.set_number_of_memory_devices(16.into());
        p.set_memory_capacity(tb(3));
        p.serialize(&mut output);

        assert_eq!(expected.as_slice(), output);
    }
}
