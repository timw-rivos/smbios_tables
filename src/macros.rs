// Copyright 2024 Rivos, Inc.
// SPDX-License-Identifier: Apache-2.0

//! These macros support brief definitions of SMBIOS structures. If a structure only contains
//! integer data types and strings, then it can be defined using `simple_smbios_structure`. The
//! SMBIOS header (4 bytes) is automatically prepended to each such structure. The macro
//! searches for `StringIndex` types and adds a setter function which accepts a string and adds
//! it to the list of strings for that structure. Other data types come with generic
//! setters. `SmbiosStructure` will be implemented for the structure.

#[macro_export]
macro_rules! inner_impl {
    // Dump everything out
    (@munch () -> {impl $name:ident $innername:ident $($output:tt)*}) => {
        impl $name {
            $($output)*

            pub fn new(handle: u16) -> Self {
                Self {
                    data: $innername::new(handle),
                    strings: Vec::new(),
                }
            }

            pub fn get_handle(&self) -> u16 {
                self.data.handle.into()
            }
        }
    };

    // Create a special setter for each StringIndex
    (@munch ($ident:ident : StringIndex, $($next:tt)*) -> {$($output:tt)*}) => {
        inner_impl!(@munch ($($next)*) -> {
            $($output)*
                paste! {
                    pub fn [<set_ $ident>](&mut self, s: &str) {
                        self.data.$ident = self.add_string(s);
                    }
                }
        });
    };

    // Create a generic setter for all other types
    (@munch ($ident:ident : $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        inner_impl!(@munch ($($next)*) -> {
            $($output)*
                paste! {
                    pub fn [<set_ $ident>](&mut self, t: $ty) {
                        self.data.$ident = t;
                    }
                }
        });
    };

    // Initial match
    (impl $name:ident $innername:ident { $($input:tt)* }) => {
        inner_impl!(@munch ($($input)*) -> {impl $name $innername});
    }
}

// Generate the data struct with the header automatically included
#[macro_export]
macro_rules! inner_struct {
    // Final call, dump out the struct definition
    (@munch () -> {struct $name:ident $(($id:ident: $ty:ty))*}) => {
        #[repr(C, packed)]
        #[derive(Copy, Clone, Default, Debug, AsBytes)]
        struct $name {
            r#type: u8,
            length: u8,
            handle: U16,
            $($id: $ty),*
        }
    };

    // Handle the fields
    (@munch ($id:ident : $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        inner_struct!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };

    // Initial call to macro, setup the field muncher
    (struct $name:ident { $($input:tt)* }) => {
        inner_struct!(@munch ($($input)*) -> {struct $name});
    }
}

#[macro_export]
macro_rules! inner_new {
    ($name:ident, $n:expr) => {
        impl $name {
            fn new(handle: u16) -> Self {
                Self {
                    r#type: $n,
                    length: size_of::<Self>() as u8,
                    handle: handle.into(),
                    ..Default::default()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! simple_smbios_structure {
    // No more input to consume, so the final output is dumped
    (@outer () -> {struct $name:ident $(($id:ident: $ty:ty))*}) => {
        #[derive(Debug, Default)]
        pub struct $name {
            $($id: $ty),*,
            strings: Vec<String>,
        }

        serialize_structure_with_strings!{$name}

        impl $name {
            #[allow(dead_code)]
            fn add_string(&mut self, s:&str) -> u8 {
                self.strings.push(s.into());
                self.strings.len().try_into().unwrap()
            }
        }
    };

    // Found a struct definition inside, break it into a separate call
    (@outer ($id:ident: struct $name:ident {$($inner:tt)*} $($next:tt)*) -> {struct $outername:ident $n:expr, $($output:tt)*}) => {
        inner_struct!(struct $name { $($inner)* });
        inner_impl!(impl $outername $name { $($inner)* });
        inner_new!($name, $n);
        simple_smbios_structure!(@outer ($($next)*) -> {struct $outername $($output)* ($id: $name)});
    };

    // Handle other fields
    (@outer ($id:ident : $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        simple_smbios_structure!(@outer ($($next)*) -> {$($output)* ($id: $ty)});
    };

    // Initial expansion
    ($n:expr, struct $name:ident { $($input:tt)*} ) => {
        simple_smbios_structure!(@outer ($($input)*) -> {struct $name $n,});
    }
}

#[macro_export]
macro_rules! serialize_structure_with_strings {
    ($x:ty) => {
        impl SmbiosStructure for $x {
            fn serialize(&self, sink: &mut dyn Sink) {
                sink.vec(self.data.as_bytes());
                for s in &self.strings {
                    sink.vec(s.as_bytes());
                    sink.byte(0);
                }
                sink.byte(0);
                if self.strings.is_empty() {
                    sink.byte(0);
                }
            }
        }
    };
}
