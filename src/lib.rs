/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CString;
use os_info::Info;
use os_info::Version;
use os_info::Type;
use os_info::Bitness;

#[repr(C)]
struct os_info_t {
    info: *mut c_void
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum os_info_version_type_t {
    OS_INFO_VERSION_TYPE_UNKNOWN,
    OS_INFO_VERSION_TYPE_SEMANTIC,
    OS_INFO_VERSION_TYPE_ROLLING,
    OS_INFO_VERSION_TYPE_CUSTOM,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct os_info_semantic_t {
    num1: u64,
    num2: u64,
    num3: u64
}

#[repr(C)]
union os_info_version_t {
    semantic: os_info_semantic_t,
    rolling: *mut c_char,
    custom: *mut c_char
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum os_info_type_t {
    OS_INFO_TYPE_ALPINE,
    OS_INFO_TYPE_AMAZON,
    OS_INFO_TYPE_ANDROID,
    OS_INFO_TYPE_ARCH,
    OS_INFO_TYPE_CENTOS,
    OS_INFO_TYPE_DEBIAN,
    OS_INFO_TYPE_DRAGONFLY,
    OS_INFO_TYPE_EMSCRIPTEN,
    OS_INFO_TYPE_ENDEAVOUROS,
    OS_INFO_TYPE_FEDORA,
    OS_INFO_TYPE_FREEBSD,
    OS_INFO_TYPE_GARUDA,
    OS_INFO_TYPE_GENTOO,
    OS_INFO_TYPE_HARDENEDBSD,
    OS_INFO_TYPE_ILLUMOS,
    OS_INFO_TYPE_LINUX,
    OS_INFO_TYPE_MACOS,
    OS_INFO_TYPE_MANJARO,
    OS_INFO_TYPE_MARINER,
    OS_INFO_TYPE_MIDNIGHTBSD,
    OS_INFO_TYPE_MINT,
    OS_INFO_TYPE_NETBSD,
    OS_INFO_TYPE_NIXOS,
    OS_INFO_TYPE_OPENBSD,
    OS_INFO_TYPE_OPENSUSE,
    OS_INFO_TYPE_ORACLELINUX,
    OS_INFO_TYPE_POP,
    OS_INFO_TYPE_RASPBIAN,
    OS_INFO_TYPE_REDHAT,
    OS_INFO_TYPE_REDHATENTERPRISE,
    OS_INFO_TYPE_REDOX,
    OS_INFO_TYPE_SOLUS,
    OS_INFO_TYPE_SUSE,
    OS_INFO_TYPE_UBUNTU,
    OS_INFO_TYPE_UNKNOWN,
    OS_INFO_TYPE_WINDOWS,
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum os_info_bitness_t {
    OS_INFO_BITNESS_UNKNOWN,
    OS_INFO_BITNESS_X32,
    OS_INFO_BITNESS_X64,
}

#[no_mangle]
unsafe extern "C" fn os_info_unknown() -> os_info_t {
    let info = Info::unknown();
    os_info_t {
	info: Box::into_raw(Box::new(info)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_with_type(os_type: os_info_type_t) -> os_info_t {
    let os_type_rs = match os_type {
	os_info_type_t::OS_INFO_TYPE_ALPINE => Type::Alpine,
	os_info_type_t::OS_INFO_TYPE_AMAZON => Type::Amazon,
	os_info_type_t::OS_INFO_TYPE_ANDROID => Type::Android,
	os_info_type_t::OS_INFO_TYPE_ARCH => Type::Arch,
	os_info_type_t::OS_INFO_TYPE_CENTOS => Type::CentOS,
	os_info_type_t::OS_INFO_TYPE_DEBIAN => Type::Debian,
	os_info_type_t::OS_INFO_TYPE_DRAGONFLY => Type::DragonFly,
	os_info_type_t::OS_INFO_TYPE_EMSCRIPTEN => Type::Emscripten,
	os_info_type_t::OS_INFO_TYPE_ENDEAVOUROS => Type::EndeavourOS,
	os_info_type_t::OS_INFO_TYPE_FEDORA => Type::Fedora,
	os_info_type_t::OS_INFO_TYPE_FREEBSD => Type::FreeBSD,
	os_info_type_t::OS_INFO_TYPE_GARUDA => Type::Garuda,
	os_info_type_t::OS_INFO_TYPE_GENTOO => Type::Gentoo,
	os_info_type_t::OS_INFO_TYPE_HARDENEDBSD => Type::HardenedBSD,
	os_info_type_t::OS_INFO_TYPE_ILLUMOS => Type::Illumos,
	os_info_type_t::OS_INFO_TYPE_LINUX => Type::Linux,
	os_info_type_t::OS_INFO_TYPE_MACOS => Type::Macos,
	os_info_type_t::OS_INFO_TYPE_MANJARO => Type::Manjaro,
	os_info_type_t::OS_INFO_TYPE_MARINER => Type::Mariner,
	os_info_type_t::OS_INFO_TYPE_MIDNIGHTBSD => Type::MidnightBSD,
	os_info_type_t::OS_INFO_TYPE_MINT => Type::Mint,
	os_info_type_t::OS_INFO_TYPE_NETBSD => Type::NetBSD,
	os_info_type_t::OS_INFO_TYPE_NIXOS => Type::NixOS,
	os_info_type_t::OS_INFO_TYPE_OPENBSD => Type::OpenBSD,
	os_info_type_t::OS_INFO_TYPE_OPENSUSE => Type::openSUSE,
	os_info_type_t::OS_INFO_TYPE_ORACLELINUX => Type::OracleLinux,
	os_info_type_t::OS_INFO_TYPE_POP => Type::Pop,
	os_info_type_t::OS_INFO_TYPE_RASPBIAN => Type::Raspbian,
	os_info_type_t::OS_INFO_TYPE_REDHAT => Type::Redhat,
	os_info_type_t::OS_INFO_TYPE_REDHATENTERPRISE => Type::RedHatEnterprise,
	os_info_type_t::OS_INFO_TYPE_REDOX => Type::Redox,
	os_info_type_t::OS_INFO_TYPE_SOLUS => Type::Solus,
	os_info_type_t::OS_INFO_TYPE_SUSE => Type::SUSE,
	os_info_type_t::OS_INFO_TYPE_UBUNTU => Type::Ubuntu,
	os_info_type_t::OS_INFO_TYPE_UNKNOWN => Type::Unknown,
	os_info_type_t::OS_INFO_TYPE_WINDOWS => Type::Windows,
    };
    let info = Info::with_type(os_type_rs);
    os_info_t {
	info: Box::into_raw(Box::new(info)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_os_type(os_info: *const os_info_t) -> os_info_type_t {
    let info = &*((*os_info).info as *mut Info);
    match info.os_type() {
	Type::Alpine => os_info_type_t::OS_INFO_TYPE_ALPINE,
	Type::Amazon => os_info_type_t::OS_INFO_TYPE_AMAZON,
	Type::Android => os_info_type_t::OS_INFO_TYPE_ANDROID,
	Type::Arch => os_info_type_t::OS_INFO_TYPE_ARCH,
	Type::CentOS => os_info_type_t::OS_INFO_TYPE_CENTOS,
	Type::Debian => os_info_type_t::OS_INFO_TYPE_DEBIAN,
	Type::DragonFly => os_info_type_t::OS_INFO_TYPE_DRAGONFLY,
	Type::Emscripten => os_info_type_t::OS_INFO_TYPE_EMSCRIPTEN,
	Type::EndeavourOS => os_info_type_t::OS_INFO_TYPE_ENDEAVOUROS,
	Type::Fedora => os_info_type_t::OS_INFO_TYPE_FEDORA,
	Type::FreeBSD => os_info_type_t::OS_INFO_TYPE_FREEBSD,
	Type::Garuda => os_info_type_t::OS_INFO_TYPE_GARUDA,
	Type::Gentoo => os_info_type_t::OS_INFO_TYPE_GENTOO,
	Type::HardenedBSD => os_info_type_t::OS_INFO_TYPE_HARDENEDBSD,
	Type::Illumos => os_info_type_t::OS_INFO_TYPE_ILLUMOS,
	Type::Linux => os_info_type_t::OS_INFO_TYPE_LINUX,
	Type::Macos => os_info_type_t::OS_INFO_TYPE_MACOS,
	Type::Manjaro => os_info_type_t::OS_INFO_TYPE_MANJARO,
	Type::Mariner => os_info_type_t::OS_INFO_TYPE_MARINER,
	Type::MidnightBSD => os_info_type_t::OS_INFO_TYPE_MIDNIGHTBSD,
	Type::Mint => os_info_type_t::OS_INFO_TYPE_MINT,
	Type::NetBSD => os_info_type_t::OS_INFO_TYPE_NETBSD,
	Type::NixOS => os_info_type_t::OS_INFO_TYPE_NIXOS,
	Type::OpenBSD => os_info_type_t::OS_INFO_TYPE_OPENBSD,
	Type::openSUSE => os_info_type_t::OS_INFO_TYPE_OPENSUSE,
	Type::OracleLinux => os_info_type_t::OS_INFO_TYPE_ORACLELINUX,
	Type::Pop => os_info_type_t::OS_INFO_TYPE_POP,
	Type::Raspbian => os_info_type_t::OS_INFO_TYPE_RASPBIAN,
	Type::Redhat => os_info_type_t::OS_INFO_TYPE_REDHAT,
	Type::RedHatEnterprise => os_info_type_t::OS_INFO_TYPE_REDHATENTERPRISE,
	Type::Redox => os_info_type_t::OS_INFO_TYPE_REDOX,
	Type::Solus => os_info_type_t::OS_INFO_TYPE_SOLUS,
	Type::SUSE => os_info_type_t::OS_INFO_TYPE_SUSE,
	Type::Ubuntu => os_info_type_t::OS_INFO_TYPE_UBUNTU,
	Type::Unknown => os_info_type_t::OS_INFO_TYPE_UNKNOWN,
	Type::Windows => os_info_type_t::OS_INFO_TYPE_WINDOWS,
	_ => os_info_type_t::OS_INFO_TYPE_UNKNOWN
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_version(os_info: *const os_info_t, version: *mut os_info_version_t, is_err: *mut c_int) -> os_info_version_type_t {
    let info = &*((*os_info).info as *mut Info);
    match info.version() {
	Version::Unknown => {
	    *is_err = 0;
	    os_info_version_type_t::OS_INFO_VERSION_TYPE_UNKNOWN
	},
	Version::Semantic(num1, num2, num3) => {
	    (*version).semantic = os_info_semantic_t {
		num1: *num1,
		num2: *num2,
		num3: *num3
	    };
	    *is_err = 0;
	    os_info_version_type_t::OS_INFO_VERSION_TYPE_SEMANTIC
	},
	Version::Rolling(opt_v) => match opt_v {
	    Some(v) => match CString::new(v.to_owned()) {
		Ok(cstr) => {
		    (*version).rolling = cstr.into_raw();
		    *is_err = 0;
		    os_info_version_type_t::OS_INFO_VERSION_TYPE_ROLLING
		},
		Err(_) => {
		    *is_err = 1;
		    os_info_version_type_t::OS_INFO_VERSION_TYPE_ROLLING
		}
	    },
	    None => {
		*is_err = 0;
		os_info_version_type_t::OS_INFO_VERSION_TYPE_ROLLING
	    }
	},
	Version::Custom(v) => match CString::new(v.to_owned()) {
	    Ok(cstr) => {
		(*version).custom = cstr.into_raw();
		*is_err = 0;
		os_info_version_type_t::OS_INFO_VERSION_TYPE_CUSTOM
	    },
	    Err(_) => {
		*is_err = 1;
		os_info_version_type_t::OS_INFO_VERSION_TYPE_CUSTOM
	    }
	}
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_edition(os_info: *const os_info_t) -> *const c_char {
    let info = &*((*os_info).info as *mut Info);
    match info.edition() {
	Some(v) => {
	    let cstring = match CString::new(v) {
		Ok(v) => v,
		Err(_) => return std::ptr::null()
	    };
	    let cstr = cstring.as_ptr();
	    std::mem::forget(cstring);
	    cstr
	},
	None => std::ptr::null()
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_codename(os_info: *const os_info_t) -> *const c_char {
    let info = &*((*os_info).info as *mut Info);
    match info.codename() {
	Some(v) => {
	    let cstring = match CString::new(v) {
		Ok(v) => v,
		Err(_) => return std::ptr::null()
	    };
	    let cstr = cstring.as_ptr();
	    std::mem::forget(cstring);
	    cstr
	},
	None => std::ptr::null()
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_bitness(os_info: *const os_info_t) -> os_info_bitness_t {
    let info = &*((*os_info).info as *mut Info);
    match info.bitness() {
	Bitness::Unknown => os_info_bitness_t::OS_INFO_BITNESS_UNKNOWN,
	Bitness::X32 => os_info_bitness_t::OS_INFO_BITNESS_X32,
	Bitness::X64 => os_info_bitness_t::OS_INFO_BITNESS_X64,
	_ => os_info_bitness_t::OS_INFO_BITNESS_UNKNOWN
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_clean(os_info: *mut os_info_t) {
    if !os_info.is_null() {
	let _ = Box::from_raw((*os_info).info as *mut Info);
    }
}

#[no_mangle]
unsafe extern "C" fn os_info_string_clean(ptr: *mut c_char) {
    if !ptr.is_null() {
	let _ = CString::from_raw(ptr);
    }
}
