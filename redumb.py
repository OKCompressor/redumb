import ctypes, pathlib, os

lib = ctypes.CDLL(pathlib.Path(__file__).with_name("libredumb.so"))

_argv = lambda s: ctypes.c_char_p(os.fsencode(s))

def encode(inp, dict_dir, sdict_dir, enc_dir):
    rc = lib.redumb_encode(_argv(inp), _argv(dict_dir),
                           _argv(sdict_dir), _argv(enc_dir))
    if rc != 0: raise RuntimeError("encode failed")

def restore(dict_dir, enc_dir, out):
    rc = lib.redumb_restore(_argv(dict_dir), _argv(enc_dir), _argv(out))
    if rc != 0: raise RuntimeError("restore failed")



# Usage:
#import redumb as rd
#rd.encode("data/enwik9", "out/dicts", "out/sdicts", "out/encs")
#rd.restore("out/dicts", "out/encs", "out/recon.txt")



