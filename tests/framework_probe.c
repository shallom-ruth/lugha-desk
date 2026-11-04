/* Developer-only dependency test. Compile /MT so the probe has no dynamic VC runtime. */
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>
#include <psapi.h>

static int verify_vc_origins(const wchar_t *framework) {
    const wchar_t *prefixes[] = {L"vcruntime140", L"msvcp140", L"concrt140",
        L"vcomp140", L"vccorlib140", L"vcamp140"};
    HMODULE modules[1024];
    DWORD bytes, length;
    size_t index, prefix;
    wchar_t actual[32768], expected[32768];
    const wchar_t *name;
    if (!K32EnumProcessModules(GetCurrentProcess(), modules, (DWORD)sizeof(modules), &bytes) ||
        bytes > sizeof(modules)) return 6;
    for (index = 0; index < bytes / sizeof(HMODULE); index++) {
        length = GetModuleFileNameW(modules[index], actual, (DWORD)_countof(actual));
        if (!length || length >= _countof(actual)) return 6;
        name = wcsrchr(actual, L'\\');
        name = name ? name + 1 : actual;
        for (prefix = 0; prefix < _countof(prefixes); prefix++) {
            if (_wcsnicmp(name, prefixes[prefix], wcslen(prefixes[prefix]))) continue;
            if (swprintf_s(expected, _countof(expected), L"%ls\\%ls", framework, name) < 0) return 6;
            if (_wcsicmp(actual, expected)) {
                wprintf(L"FAIL: a transitive VC module came from outside the supplied framework: %ls\n", name);
                return 7;
            }
            break;
        }
    }
    return 0;
}

static int load_from(const wchar_t *directory, const wchar_t *name, int required) {
    wchar_t path[32768], actual[32768];
    HMODULE module;
    DWORD length;
    if (swprintf_s(path, _countof(path), L"%ls\\%ls", directory, name) < 0) return 2;
    if (GetFileAttributesW(path) == INVALID_FILE_ATTRIBUTES) {
        if (required) wprintf(L"FAIL: required supplied file is missing: %ls\n", name);
        return required ? 3 : 0;
    }
    module = LoadLibraryExW(path, NULL, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR |
                           LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_USER_DIRS);
    if (!module) {
        wprintf(L"FAIL: dependency load failed for %ls (Windows error %lu)\n", name, GetLastError());
        return 4;
    }
    length = GetModuleFileNameW(module, actual, (DWORD)_countof(actual));
    if (!length || length >= _countof(actual) || _wcsicmp(path, actual)) {
        wprintf(L"FAIL: loader did not use the supplied %ls\n", name);
        return 5;
    }
    wprintf(L"Loaded supplied %ls\n", name);
    /* Retain modules until process exit so dependent DLLs bind to these copies. */
    return 0;
}

int wmain(int argc, wchar_t **argv) {
    const wchar_t *framework[] = {L"vcruntime140.dll", L"vcruntime140_1.dll", L"msvcp140.dll",
        L"msvcp140_1.dll", L"msvcp140_2.dll", L"msvcp140_atomic_wait.dll",
        L"msvcp140_codecvt_ids.dll", L"concrt140.dll"};
    const wchar_t *runtime[] = {L"ggml-base.dll", L"libomp.dll", L"ggml-cpu-x64.dll",
        L"ggml.dll", L"llama.dll", L"llama-common.dll", L"mtmd.dll", L"llama-server-impl.dll"};
    size_t index;
    int result;
    if (argc != 3) return 1;
    if (!SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_USER_DIRS) ||
        !AddDllDirectory(argv[1]) || !AddDllDirectory(argv[2])) return 2;
    for (index = 0; index < _countof(framework); index++) {
        result = load_from(argv[1], framework[index], index <= 2);
        if (result) return result;
    }
    for (index = 0; index < _countof(runtime); index++) {
        result = load_from(argv[2], runtime[index], 1);
        if (result) return result;
    }
    result = verify_vc_origins(argv[1]);
    if (result) return result;
    wprintf(L"PASS: pinned runtime DLLs load with the supplied Desktop framework.\n");
    return 0;
}
