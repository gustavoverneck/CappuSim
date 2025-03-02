import pyopencl as cl
import numpy as np
from tqdm import tqdm
import ast
import inspect
import textwrap

# Function to get the version of a library
def get_version(lib):
    try:
        return lib.__version__
    except AttributeError:
        return "Version not found"

# Print versions of all libraries
print(f"pyopencl version: {get_version(cl)}")
print(f"numpy version: {get_version(np)}")
print(f"tqdm version: {get_version(tqdm)}")
print(f"ast version: {get_version(ast)}")  # Part of Python standard library, no version
print(f"inspect version: {get_version(inspect)}")  # Part of Python standard library, no version
print(f"textwrap version: {get_version(textwrap)}")  # Part of Python standard library, no version

# For pyopencl.tools, which doesn't have a direct version attribute
try:
    import pyopencl.tools
    print(f"pyopencl.tools version: {get_version(pyopencl.tools)}")
except ImportError:
    print("pyopencl.tools not found")

# For Python standard library modules (no version attribute)
import time
import os
print(f"Python standard library time module: No version (part of Python {os.sys.version})")
print(f"Python standard library os module: No version (part of Python {os.sys.version})")