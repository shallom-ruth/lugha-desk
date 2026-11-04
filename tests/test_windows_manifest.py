"""Inspect the actual linked EXE, not the resource source text."""
import ctypes
from ctypes import wintypes
import sys
import xml.etree.ElementTree as ET


def manifest(path):
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
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
    module = kernel.LoadLibraryExW(str(path), None, 2)  # DATAFILE; never execute it
    assert module, ctypes.WinError(ctypes.get_last_error())
    try:
        resource = kernel.FindResourceW(module, ctypes.cast(1, wintypes.LPCWSTR), ctypes.cast(24, wintypes.LPCWSTR))
        assert resource, "EXE has no embedded application manifest"
        size = kernel.SizeofResource(module, resource)
        data = kernel.LockResource(kernel.LoadResource(module, resource))
        return ET.fromstring(ctypes.string_at(data, size))
    finally:
        kernel.FreeLibrary(module)


if __name__ == "__main__":
    root = manifest(sys.argv[1])
    awareness = root.find(".//{http://schemas.microsoft.com/SMI/2016/WindowsSettings}dpiAwareness")
    assert awareness is not None and awareness.text == "PerMonitorV2", "EXE must declare PerMonitorV2 DPI awareness"
    level = root.find(".//{urn:schemas-microsoft-com:asm.v3}requestedExecutionLevel")
    assert level is not None and level.get("level") == "asInvoker", "Translation must not request administrator rights"
    assert level.get("uiAccess") == "false"
    print("Actual EXE manifest: PerMonitorV2, asInvoker, no UIAccess")
