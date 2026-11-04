"""Verify the actual app/setup PE icon resources match the shared source ICO."""
import ctypes
from ctypes import wintypes
from pathlib import Path
import struct
import sys


def check(exe):
    source = (Path(__file__).resolve().parents[1] / 'assets/lugha-desk.ico').read_bytes()
    reserved, kind, count = struct.unpack_from('<HHH', source)
    assert (reserved, kind) == (0, 1)
    kernel = ctypes.WinDLL('kernel32', use_last_error=True)
    kernel.LoadLibraryExW.argtypes = [wintypes.LPCWSTR, wintypes.HANDLE, wintypes.DWORD]
    kernel.LoadLibraryExW.restype = wintypes.HMODULE
    kernel.FindResourceW.argtypes = [wintypes.HMODULE, wintypes.LPCWSTR, wintypes.LPCWSTR]
    kernel.FindResourceW.restype = wintypes.HANDLE
    kernel.SizeofResource.argtypes = [wintypes.HMODULE, wintypes.HANDLE]
    kernel.SizeofResource.restype = wintypes.DWORD
    kernel.LoadResource.argtypes = [wintypes.HMODULE, wintypes.HANDLE]
    kernel.LoadResource.restype = wintypes.HANDLE
    kernel.LockResource.argtypes = [wintypes.HANDLE]
    kernel.LockResource.restype = ctypes.c_void_p
    kernel.FreeLibrary.argtypes = [wintypes.HMODULE]
    module = kernel.LoadLibraryExW(str(Path(exe).resolve()), None, 2)
    assert module, ctypes.WinError(ctypes.get_last_error())

    def resource(identifier, resource_type):
        found = kernel.FindResourceW(module, ctypes.cast(identifier, wintypes.LPCWSTR), ctypes.cast(resource_type, wintypes.LPCWSTR))
        assert found, f'{exe}: missing icon resource {identifier}/{resource_type}'
        size = kernel.SizeofResource(module, found)
        pointer = kernel.LockResource(kernel.LoadResource(module, found))
        return ctypes.string_at(pointer, size)

    try:
        group = resource(1, 14)  # RT_GROUP_ICON; same ID used by the window class
        assert struct.unpack_from('<HHH', group) == (0, 1, count)
        sizes = []
        for index in range(count):
            expected = struct.unpack_from('<BBBBHHII', source, 6 + index * 16)
            actual = struct.unpack_from('<BBBBHHIH', group, 6 + index * 14)
            assert actual[:7] == expected[:7], f'{exe}: icon dimensions/format differ'
            assert resource(actual[7], 3) == source[expected[7]:expected[7]+expected[6]], f'{exe}: icon pixels differ'
            sizes.append(expected[0] or 256)
        assert sizes == [16, 24, 32, 48, 64, 128, 256]
        print(f'{exe}: shared Lugha Desk icon verified at {sizes}')
    finally:
        kernel.FreeLibrary(module)


if __name__ == '__main__':
    for exe in sys.argv[1:]:
        check(exe)
