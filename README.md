# libosinfo

[![](https://img.shields.io/github/v/tag/thechampagne/libosinfo?label=version)](https://github.com/thechampagne/libosinfo/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libosinfo)](https://github.com/thechampagne/libosinfo/blob/main/LICENSE)

A **C** library to detect the operating system type.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libosinfo.git
```
#### 2. Navigate to the root
```
cd libosinfo
```
#### 3. Build the project
```
cargo build
```

### API

```c
typedef struct {
  void* info;
} os_info_t;

typedef enum {
    // ...
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
    // ...
} os_info_type_t;

typedef enum {
   // ...
} os_info_bitness_t;

extern os_info_t os_info_unknown(void);

extern os_info_t os_info_with_type(os_info_type_t os_type);

extern os_info_type_t os_info_os_type(const os_info_t* os_info);

extern os_info_version_type_t os_info_version(const os_info_t* os_info, os_info_version_t* version, int* is_err);

extern const char* os_info_edition(const os_info_t* os_info);

extern const char* os_info_codename(const os_info_t* os_info);

extern os_info_bitness_t  os_info_bitness(const os_info_t* os_info);
```

### References
 - [os_info](https://github.com/stanislav-tkach/os_info)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libosinfo/blob/main/LICENSE).