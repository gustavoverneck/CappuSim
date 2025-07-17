use std::path::Path;

fn main() {
    // Set the OpenCL library path for Windows
    #[cfg(target_os = "windows")]
    {
        let mut found = false;
        
        // 1. Check for CUDA installations (most common)
        let cuda_base = r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA";
        if let Ok(entries) = std::fs::read_dir(cuda_base) {
            for entry in entries.flatten() {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let cuda_lib_path = entry.path().join("lib").join("x64");
                    let opencl_lib = cuda_lib_path.join("OpenCL.lib");
                    if opencl_lib.exists() {
                        println!("cargo:rustc-link-search=native={}", cuda_lib_path.display());
                        println!("cargo:rustc-link-lib=OpenCL");
                        println!("cargo:warning=Using CUDA OpenCL from: {}", cuda_lib_path.display());
                        found = true;
                        break;
                    }
                }
            }
        }
        
        // 2. Check for Intel OpenCL SDK
        if !found {
            let intel_paths = [
                r"C:\Program Files (x86)\Intel\OpenCL SDK\lib\x64",
                r"C:\Program Files\Intel\OpenCL SDK\lib\x64",
                r"C:\Program Files (x86)\OCL_SDK_Light\lib\x86_64",
            ];
            
            for path in &intel_paths {
                let opencl_lib = Path::new(path).join("OpenCL.lib");
                if opencl_lib.exists() {
                    println!("cargo:rustc-link-search=native={}", path);
                    println!("cargo:rustc-link-lib=OpenCL");
                    println!("cargo:warning=Using Intel OpenCL from: {}", path);
                    found = true;
                    break;
                }
            }
        }
        
        // 3. Check for AMD OpenCL
        if !found {
            let amd_paths = [
                r"C:\Program Files (x86)\AMD APP SDK\lib\x86_64",
                r"C:\Program Files\AMD APP SDK\lib\x86_64",
            ];
            
            for path in &amd_paths {
                let opencl_lib = Path::new(path).join("OpenCL.lib");
                if opencl_lib.exists() {
                    println!("cargo:rustc-link-search=native={}", path);
                    println!("cargo:rustc-link-lib=OpenCL");
                    println!("cargo:warning=Using AMD OpenCL from: {}", path);
                    found = true;
                    break;
                }
            }
        }
        
        // 4. Check Windows system directories
        if !found {
            let system_paths = [
                r"C:\Windows\System32",
                r"C:\Windows\SysWOW64",
            ];
            
            for path in &system_paths {
                let opencl_lib = Path::new(path).join("OpenCL.lib");
                if opencl_lib.exists() {
                    println!("cargo:rustc-link-search=native={}", path);
                    println!("cargo:rustc-link-lib=OpenCL");
                    println!("cargo:warning=Using system OpenCL from: {}", path);
                    found = true;
                    break;
                }
            }
        }
        
        if !found {
            panic!("OpenCL library not found. Please install one of the following:
- NVIDIA CUDA Toolkit (recommended)
- Intel OpenCL SDK
- AMD APP SDK
- Or ensure OpenCL.lib is in your system PATH");
        }
    }
    
    // For non-Windows systems, rely on system OpenCL installation
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=OpenCL");
    }
}
