
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winnt::*;
/* automatically generated by rust-bindgen 0.57.0 */

extern "C" {
    pub fn Everything_SetSearchW(lpString: LPCWSTR);
}
extern "C" {
    pub fn Everything_SetSearchA(lpString: LPCSTR);
}
extern "C" {
    pub fn Everything_SetMatchPath(bEnable: BOOL);
}
extern "C" {
    pub fn Everything_SetMatchCase(bEnable: BOOL);
}
extern "C" {
    pub fn Everything_SetMatchWholeWord(bEnable: BOOL);
}
extern "C" {
    pub fn Everything_SetRegex(bEnable: BOOL);
}
extern "C" {
    pub fn Everything_SetMax(dwMax: DWORD);
}
extern "C" {
    pub fn Everything_SetOffset(dwOffset: DWORD);
}
extern "C" {
    pub fn Everything_SetReplyWindow(hWnd: HWND);
}
extern "C" {
    pub fn Everything_SetReplyID(dwId: DWORD);
}
extern "C" {
    pub fn Everything_SetSort(dwSort: DWORD);
}
extern "C" {
    pub fn Everything_SetRequestFlags(dwRequestFlags: DWORD);
}
extern "C" {
    pub fn Everything_GetMatchPath() -> BOOL;
}
extern "C" {
    pub fn Everything_GetMatchCase() -> BOOL;
}
extern "C" {
    pub fn Everything_GetMatchWholeWord() -> BOOL;
}
extern "C" {
    pub fn Everything_GetRegex() -> BOOL;
}
extern "C" {
    pub fn Everything_GetMax() -> DWORD;
}
extern "C" {
    pub fn Everything_GetOffset() -> DWORD;
}
extern "C" {
    pub fn Everything_GetSearchA() -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetSearchW() -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetLastError() -> DWORD;
}
extern "C" {
    pub fn Everything_GetReplyWindow() -> HWND;
}
extern "C" {
    pub fn Everything_GetReplyID() -> DWORD;
}
extern "C" {
    pub fn Everything_GetSort() -> DWORD;
}
extern "C" {
    pub fn Everything_GetRequestFlags() -> DWORD;
}
extern "C" {
    pub fn Everything_QueryA(bWait: BOOL) -> BOOL;
}
extern "C" {
    pub fn Everything_QueryW(bWait: BOOL) -> BOOL;
}
extern "C" {
    pub fn Everything_IsQueryReply(
        message: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        dwId: DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn Everything_SortResultsByPath();
}
extern "C" {
    pub fn Everything_GetNumFileResults() -> DWORD;
}
extern "C" {
    pub fn Everything_GetNumFolderResults() -> DWORD;
}
extern "C" {
    pub fn Everything_GetNumResults() -> DWORD;
}
extern "C" {
    pub fn Everything_GetTotFileResults() -> DWORD;
}
extern "C" {
    pub fn Everything_GetTotFolderResults() -> DWORD;
}
extern "C" {
    pub fn Everything_GetTotResults() -> DWORD;
}
extern "C" {
    pub fn Everything_IsVolumeResult(dwIndex: DWORD) -> BOOL;
}
extern "C" {
    pub fn Everything_IsFolderResult(dwIndex: DWORD) -> BOOL;
}
extern "C" {
    pub fn Everything_IsFileResult(dwIndex: DWORD) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultFileNameW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultFileNameA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultPathW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultPathA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultFullPathNameA(dwIndex: DWORD, buf: LPSTR, bufsize: DWORD) -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultFullPathNameW(
        dwIndex: DWORD,
        wbuf: LPWSTR,
        wbuf_size_in_wchars: DWORD,
    ) -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultListSort() -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultListRequestFlags() -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultExtensionW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultExtensionA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultSize(dwIndex: DWORD, lpSize: *mut LARGE_INTEGER) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultDateCreated(dwIndex: DWORD, lpDateCreated: *mut FILETIME) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultDateModified(dwIndex: DWORD, lpDateModified: *mut FILETIME) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultDateAccessed(dwIndex: DWORD, lpDateAccessed: *mut FILETIME) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultAttributes(dwIndex: DWORD) -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultFileListFileNameW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultFileListFileNameA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultRunCount(dwIndex: DWORD) -> DWORD;
}
extern "C" {
    pub fn Everything_GetResultDateRun(dwIndex: DWORD, lpDateRun: *mut FILETIME) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultDateRecentlyChanged(
        dwIndex: DWORD,
        lpDateRecentlyChanged: *mut FILETIME,
    ) -> BOOL;
}
extern "C" {
    pub fn Everything_GetResultHighlightedFileNameW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultHighlightedFileNameA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultHighlightedPathW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultHighlightedPathA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_GetResultHighlightedFullPathAndFileNameW(dwIndex: DWORD) -> LPCWSTR;
}
extern "C" {
    pub fn Everything_GetResultHighlightedFullPathAndFileNameA(dwIndex: DWORD) -> LPCSTR;
}
extern "C" {
    pub fn Everything_Reset();
}
extern "C" {
    pub fn Everything_CleanUp();
}
extern "C" {
    pub fn Everything_GetMajorVersion() -> DWORD;
}
extern "C" {
    pub fn Everything_GetMinorVersion() -> DWORD;
}
extern "C" {
    pub fn Everything_GetRevision() -> DWORD;
}
extern "C" {
    pub fn Everything_GetBuildNumber() -> DWORD;
}
extern "C" {
    pub fn Everything_Exit() -> BOOL;
}
extern "C" {
    pub fn Everything_IsDBLoaded() -> BOOL;
}
extern "C" {
    pub fn Everything_IsAdmin() -> BOOL;
}
extern "C" {
    pub fn Everything_IsAppData() -> BOOL;
}
extern "C" {
    pub fn Everything_RebuildDB() -> BOOL;
}
extern "C" {
    pub fn Everything_UpdateAllFolderIndexes() -> BOOL;
}
extern "C" {
    pub fn Everything_SaveDB() -> BOOL;
}
extern "C" {
    pub fn Everything_SaveRunHistory() -> BOOL;
}
extern "C" {
    pub fn Everything_DeleteRunHistory() -> BOOL;
}
extern "C" {
    pub fn Everything_GetTargetMachine() -> DWORD;
}
extern "C" {
    pub fn Everything_GetRunCountFromFileNameW(lpFileName: LPCWSTR) -> DWORD;
}
extern "C" {
    pub fn Everything_GetRunCountFromFileNameA(lpFileName: LPCSTR) -> DWORD;
}
extern "C" {
    pub fn Everything_SetRunCountFromFileNameW(lpFileName: LPCWSTR, dwRunCount: DWORD) -> BOOL;
}
extern "C" {
    pub fn Everything_SetRunCountFromFileNameA(lpFileName: LPCSTR, dwRunCount: DWORD) -> BOOL;
}
extern "C" {
    pub fn Everything_IncRunCountFromFileNameW(lpFileName: LPCWSTR) -> DWORD;
}
extern "C" {
    pub fn Everything_IncRunCountFromFileNameA(lpFileName: LPCSTR) -> DWORD;
}
