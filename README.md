# smbios_tables

## Design

This crate provides the ability (`no_std` compatible) to generate SMBIOS
structures at runtime given the proper inputs. Supports SMBIOS 3.7.

## Usage

* Provide a type which implements the `Sink` trait, e.g.

```rust
struct VecBuf {
   buf: Vec<u8>
}

impl smbios_tables::Sink for VecBuf {
    fn byte(&mut self, byte: u8) {
	    self.buf.push(byte);
	}
}
```

* Use the `set_*()` APIs to build the tables, e.g.

```rust
fn type0(sink: &mut dyn smbios_tables::Sink, handle: u16) {
    let mut b = smbios_tables::BiosInformation::new(1);
	b.set_vendor("My Vendor");
	b.set_bios_version("1.0");
	b.set_bios_release_date("1/1/2024");
	b.serialize(sink);
}
```

* Create an Entrypoint, e.g.
```rust
fn entrypoint(smbios_structures_size: u32, pointer_to_smbios_structures: u64) -> smbios_tables::Entrypoint {
    smbios_tables::Entrypoint::new(smbios_structures_size, pointer_to_smbios_structures)
}
```

## Licence

This crate is licensed under the Apache 2.0 licence. The full text can be found
in the LICENSE file.
