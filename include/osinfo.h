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
#ifndef __OSINFO_H__
#define __OSINFO_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  void* info;
} os_info_t;

typedef enum {
    OS_INFO_VERSION_TYPE_UNKNOWN,
    OS_INFO_VERSION_TYPE_SEMANTIC,
    OS_INFO_VERSION_TYPE_ROLLING,
    OS_INFO_VERSION_TYPE_CUSTOM,
} os_info_version_type_t;

typedef struct {
  uint64_t num1;
  uint64_t num2;
  uint64_t num3;
} os_info_semantic_t;

typedef union {
  os_info_semantic_t semantic;
  char* rolling;
  char* custom;
} os_info_version_t;

typedef enum {
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
} os_info_type_t;

typedef enum {
    OS_INFO_BITNESS_UNKNOWN,
    OS_INFO_BITNESS_X32,
    OS_INFO_BITNESS_X64,
} os_info_bitness_t;

extern os_info_t os_info_unknown(void);

extern os_info_t os_info_with_type(os_info_type_t os_type);

extern os_info_type_t os_info_os_type(const os_info_t* os_info);

extern os_info_version_type_t os_info_version(const os_info_t* os_info, os_info_version_t* version, int* is_err);

extern const char* os_info_edition(const os_info_t* os_info);

extern const char* os_info_codename(const os_info_t* os_info);

extern os_info_bitness_t  os_info_bitness(const os_info_t* os_info);

extern void os_info_clean(os_info_t* os_info);

extern void os_info_string_clean(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // __OSINFO_H__
