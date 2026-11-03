"""Developer-only resource packaging. No Python is shipped or needed by users."""
from pathlib import Path
import ctypes as C,hashlib,json,os
P=Path(__file__).resolve().parent
ROOT=Path(os.environ['LUGHA_ASSET_ROOT']) if 'LUGHA_ASSET_ROOT' in os.environ else P/'gui-preview'
manifest=json.loads((ROOT/'bundle-manifest.json').read_text(encoding='utf-8'))
cab=C.WinDLL('cabinet.dll',use_last_error=True)
cab.CreateCompressor.argtypes=[C.c_uint32,C.c_void_p,C.POINTER(C.c_void_p)]
cab.Compress.argtypes=[C.c_void_p,C.c_void_p,C.c_size_t,C.c_void_p,C.c_size_t,C.POINTER(C.c_size_t)]
cab.CloseCompressor.argtypes=[C.c_void_p]
(P/'assets').mkdir(exist_ok=True)
lines=['pub struct Asset {pub path: &\'static str,pub len:usize,pub sha: &\'static str,pub data: &\'static [u8]}','pub const ASSETS:&[Asset]=&[']
total=0; raw=0
for i,(name,info) in enumerate(sorted(manifest['files'].items())):
 if name.startswith('model/'):continue
 data=(ROOT/name).read_bytes()
 assert hashlib.sha256(data).hexdigest()==info['sha256'],name
 handle=C.c_void_p();assert cab.CreateCompressor(4,None,C.byref(handle))
 try:
  source=C.create_string_buffer(data);size=C.c_size_t()
  cab.Compress(handle,source,len(data),None,0,C.byref(size))
  output=C.create_string_buffer(size.value)
  assert cab.Compress(handle,source,len(data),output,len(output),C.byref(size))
  packed=output.raw[:size.value]
 finally:cab.CloseCompressor(handle)
 file=f'{i:03}.bin';(P/'assets'/file).write_bytes(packed)
 lines.append('Asset{path:'+json.dumps(name)+',len:'+str(len(data))+',sha:'+json.dumps(info['sha256'])+',data:include_bytes!("../assets/'+file+'")},')
 total+=len(packed);raw+=len(data)
lines.append('];')
(P/'src/assets.rs').write_text('\n'.join(lines)+'\n',encoding='utf-8')
(P/'assets-report.json').write_text(json.dumps({'raw_bytes':raw,'embedded_bytes':total,'model_embedded':False,'file_count':len(lines)-3},indent=2),encoding='utf-8')
print('Embedded resources:',total,'bytes; original:',raw,'bytes; model not embedded.')
