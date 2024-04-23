// Copyright 2024 Rivos, Inc.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

#[macro_use]
mod macros;
pub mod tables;
mod types;

// In SMBIOS structures that contain string values, the strings are located directly after the
// main structure, and the (byte-valued) index for each string goes in the structure itself.
type StringIndex = u8;

// A generic sink for raw data; used by the `SmbiosTable` trait to serialize structures into.
pub trait Sink {
    fn byte(&mut self, byte: u8);
    fn word(&mut self, word: u16) {
        for byte in word.to_le_bytes() {
            self.byte(byte);
        }
    }
    fn dword(&mut self, dword: u32) {
        for byte in dword.to_le_bytes() {
            self.byte(byte);
        }
    }
    fn qword(&mut self, qword: u64) {
        for byte in qword.to_le_bytes() {
            self.byte(byte);
        }
    }
    fn vec(&mut self, v: &[u8]) {
        for byte in v {
            self.byte(*byte);
        }
    }
}

// It may be useful to have a Vec of u8s be a Sink
impl Sink for alloc::vec::Vec<u8> {
    fn byte(&mut self, byte: u8) {
        self.push(byte);
    }
}

// SMBIOS structures can be serialized
pub trait SmbiosStructure {
    fn serialize(&self, sink: &mut dyn Sink);
}
