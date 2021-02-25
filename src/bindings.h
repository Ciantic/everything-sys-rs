// Disable windows.h, enable unicode
#define _INC_WINDOWS
#define UNICODE
#define _WIN64

#include <stddef.h>

// Required winapi types so that rust-bindings don't take ages to complete
#define CONST const
typedef int BOOL;
typedef long LONG;
#if !defined(_M_IX86)
typedef __int64 LONGLONG;
#else
typedef double LONGLONG;
#endif
typedef unsigned long DWORD;
typedef wchar_t WCHAR;
typedef WCHAR *LPWSTR;
typedef wchar_t const *LPCWSTR;
typedef unsigned long DWORD;
typedef void *PVOID;
typedef void *LPVOID;
typedef PVOID HANDLE;
typedef HANDLE HWND;
typedef unsigned int UINT;
typedef char CHAR;
typedef CONST CHAR *LPCSTR, *PCSTR;
#ifdef UNICODE
typedef LPWSTR PTSTR;
#else typedef LPSTR PTSTR;
#endif
#if defined(_WIN64)
typedef unsigned __int64 UINT_PTR;
#else
typedef unsigned int UINT_PTR;
#endif
typedef UINT_PTR *PUINT_PTR;
#if defined(_WIN64)
typedef unsigned __int64 ULONG_PTR;
#else
typedef unsigned long ULONG_PTR;
#endif
#if defined(_WIN64)
typedef __int64 LONG_PTR;
#else
typedef long LONG_PTR;
#endif
typedef UINT_PTR WPARAM;
typedef LONG_PTR LPARAM;
typedef CHAR *LPSTR;

#if defined(MIDL_PASS)
typedef struct _LARGE_INTEGER
{
    LONGLONG QuadPart;
} LARGE_INTEGER;
#else  // MIDL_PASS
typedef union _LARGE_INTEGER
{
    struct
    {
        DWORD LowPart;
        LONG HighPart;
    } DUMMYSTRUCTNAME;
    struct
    {
        DWORD LowPart;
        LONG HighPart;
    } u;
    LONGLONG QuadPart;
} LARGE_INTEGER;
#endif //MIDL_PASS

typedef struct _FILETIME
{
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
} FILETIME, *PFILETIME, *LPFILETIME;


#include "../Everything/include/Everything.h"