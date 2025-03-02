# config.py

# Imports
import numpy as np

# --------------------------------------------------------------------------------------------

available_velocities_sets = ['D2Q9', 'D3Q15', 'D3Q19']   # Add D3Q15 and D3Q27 in the future
avilable_simtypes = ['fluid']                       # Add plasma in the future
available_color_schemes = ["grays", "hot", "cool", "viridis", "inferno", "plasma", "magma", "cividis", "jet", "turbo", "RdYlBu", "blues"]
available_dtypes = ["FP16", "FP32", "FP64"]

class Config:
    """
    Config class for managing simulation configuration settings.
    Attributes:
        config (dict): A dictionary containing the configuration settings.
    Methods:
        __init__(velocities_set='D2Q9', use_temperature=True, use_graphics=True, simtype='fluid', grid_size=(100, 100, 0), viscosity=0.1, total_timesteps=1000, cmap='inferno', window_dimensions=(1280, 720)):
            Initializes the Config object with default or provided settings.
        get():
            Retrieves the current configuration.
        checkConfig():
            Validates the configuration settings.
    """
    
    def __init__(self, velocities_set='D2Q9', 
                use_temperature=False, 
                use_graphics=True, 
                simtype='fluid', 
                grid_size=(64, 64, 1), 
                viscosity=0.1, 
                total_timesteps=1000, 
                cmap='inferno',
                window_dimensions=(1280, 720),
                dtype='FP32'
                ):	
        """
        Initialize the configuration for the simulation.
        Parameters:
        velocities_set (str): The set of velocities to use for the simulation. Default is 'D2Q9'.
        use_temperature (bool): Flag to indicate whether to use temperature in the simulation. Default is True.
        use_graphics (bool): Flag to indicate whether to use graphics in the simulation. Default is True.
        simtype (str): The type of simulation to run. Default is 'fluid'.
        grid_size (tuple): The size of the simulation grid. Default is (100, 100, 0).
        viscosity (float): The viscosity of the fluid. Default is 0.1.
        total_timesteps (int): The total number of timesteps to run the simulation. Default is 1000.
        cmap (str): The color map to use for visualization. Default is 'inferno'.
        window_dimensions (tuple): The dimensions of the window for visualization. Default is (1280, 720).
        dtype(str): Data type of the simulation. Default is "FP32".
        Attributes:
        config (dict): A dictionary containing the configuration parameters.
        """
        
        self.config = {
            'velocities_set': velocities_set,
            'simtype': simtype,
            'use_temperature': use_temperature,
            'use_graphics': use_graphics,
            'grid_size': grid_size,
            'viscosity': viscosity,
            'total_timesteps': total_timesteps,
            'cmap': cmap,
            'window_dimensions': window_dimensions,
            'dtype': dtype
        }
        self.checkConfig()

    def get(self):
        """
        Retrieve the current configuration.
        Returns:
            dict: The current configuration settings.
        """
        return self.config

    def checkConfig(self):       
        """
        Validates the configuration dictionary stored in `self.config`.
        Raises:
            ValueError: If any required key is missing from the configuration.
            ValueError: If the value for 'velocities_set' is not in `available_velocities_sets`.
            ValueError: If the value for 'simtype' is not in `avilable_simtypes`.
            ValueError: If the value for 'use_temperature' is not a boolean.
            ValueError: If the value for 'use_graphics' is not a boolean.
            ValueError: If the value for 'grid_size' is not a tuple.
            ValueError: If the 'grid_size' tuple does not have exactly 3 elements.
            ValueError: If the value for 'viscosity' is not a float.
            ValueError: If the value for 'viscosity' is not positive.
            ValueError: If the value for 'total_timesteps' is not a positive integer.
            ValueError: If the value for 'cmap' is not in `available_color_schemes`.
            ValueError: If the value for 'window_dimensions' is not a tuple.
        The required keys in the configuration are:
            - 'velocities_set': Must be present in `available_velocities_sets`.
            - 'simtype': Must be present in `avilable_simtypes`.
            - 'use_temperature': Must be a boolean.
            - 'use_graphics': Must be a boolean.
            - 'grid_size': Must be a tuple with exactly 3 elements.
            - 'viscosity': Must be a positive float.
            - 'total_timesteps': Must be a positive integer.
            - 'cmap': Must be present in `available_color_schemes`.
            - 'window_dimensions': Must be a tuple of dimension 2.
            - 'dtype': Must be present in `available_dtypes`
        """
        
        global available_velocities_sets, avilable_simtypes, available_color_schemes, available_dtypes

        # Check if all required keys are present
        required_keys = ['velocities_set', 'simtype', 'use_temperature', 'use_graphics', 'grid_size', 'viscosity', 'total_timesteps', 'cmap']
        for key in required_keys:
            if key not in self.config:
                raise ValueError(f"Missing required config argument: {key}")
        
        # Check if the values are valid
        if self.config['velocities_set'] not in available_velocities_sets:
            raise ValueError("Invalid velocities set")
        
        # Check if the simulation type is valid
        if self.config['simtype'] not in avilable_simtypes:       
            raise ValueError("Invalid simulation type")
        
        # Check if the use_temperature value is a boolean
        if not isinstance(self.config['use_temperature'], bool):
            raise ValueError("Invalid use_temperature value")
        
        # Check if the grid size is a tuple
        if not isinstance(self.config['grid_size'], tuple):
            raise ValueError("Invalid grid size value")
        
        # Check if the grid size tuple has 3 elements
        if len(self.config['grid_size']) != 3:
            raise ValueError("Invalid grid size value")
        
        # Check if the viscosity value is a float
        if not isinstance(self.config['viscosity'], float):
            raise ValueError("Invalid viscosity value")
        
        # Check if the viscosity value is positive
        if self.config['viscosity'] < 0:
            raise ValueError("Viscosity value must be positive")
        
        # Check if the use_graphics value is a boolean
        if not isinstance(self.config['use_graphics'], bool):
            raise ValueError("Invalid use_graphics value")
        
        # Check if the total_timesteps value is a positive integer
        if not isinstance(self.config['total_timesteps'], int) or self.config['total_timesteps'] <= 0:
            raise ValueError("Total timesteps value must be a positive integer")
        
        # Check if the color map is valid
        if self.config['cmap'] not in available_color_schemes:
            raise ValueError("Invalid color map")
        
        # Check if the window dimensions is a tuple
        if not isinstance(self.config['window_dimensions'], tuple):
            raise ValueError("Invalid window dimensions value")
        
        # Check if the window dimensions tuple has 2 elements
        if len(self.config['window_dimensions']) != 2:
            raise ValueError("Invalid window dimensions value")
        
        # Check if dtype is available
        if self.config["dtype"] not in available_dtypes:
            raise ValueError("Invalid dtype")



# --------------------------------------------------------------------------------------------
# Define velocity sets
velocities_sets = {
    'D2Q9': {
        "c":[[0,0], [1,0], [-1,0], [0,1], [0,-1], [1,1], [-1,-1], [1,-1], [-1,1]],
        "w":[4./9., 1./9., 1./9., 1./9., 1./9., 1./36., 1./36., 1./36., 1./36.]
    },
    'D3Q7': {
        "c":[[0,0,0], [1,0,0], [-1,0,0], [0,1,0], [0,-1,0], [0,0,1], [0,0,-1]],
        "w":[1./4.] + [1./8.]*6
    },
    'D3Q15': {
        "c":[[0,0,0], [1,0,0], [-1,0,0], [0,1,0], [0,-1,0], [0,0,1], [0,0,-1],
            [1,1,1], [-1,-1,-1], [1,1,-1], [-1,-1,1], [1,-1,1], [-1,1,-1],
            [-1,1,1], [1,-1,-1]],
        "w":[[2./9.] + [1./9.]*6 + [1./72.]*8]
    },
    'D3Q13': {
        "c":[[0,0,0], [1,1,0], [-1,-1,0], [1,0,1], [-1,0,-1], [-1,1,1], [-1,-1,-1], [1,1,-1], [1,0,-1], [-1,0,1], [0,1,-1], [0,-1,1]],
        "w":[[1./2.], [1./24.]*12]
    },
    'D3Q19': {
        "c":[[0,0,0], [1,0,0], [-1,0,0], [0,1,0], [0,-1,0], [0,0,1], [0,0,-1],
            [1,1,0], [-1,-1,0], [1,0,1], [-1,0,-1], [0,1,1], [0,-1,-1],
            [1,-1,0], [-1,1,0], [1,0,-1], [-1,0,1], [0,1,-1], [0,-1,1]],
        "w":[[1./3.] + [1./18.]*6 + [1./36.]*12]
    },
    'D3Q27': {
        "c":[[0,0,0], [1,0,0], [-1,0,0], [0,1,0], [0,-1,0], [0,0,1], [0,0,-1],
            [1,1,0], [-1,-1,0], [1,0,1], [-1,0,-1], [0,1,1], [0,-1,-1],
            [1,-1,0], [-1,1,0], [1,0,-1], [-1,0,1], [0,1,-1], [0,-1,1],
            [1,1,1], [-1,-1,-1], [1,1,-1], [-1,-1,1], [1,-1,1], [-1,1,-1],
            [-1,1,1], [1,-1,-1]],
        "w":[[8./27.] + [2./27.]*6 + [1./54.]*12 + [1./216.]*8]
    }
}

# --------------------------------------------------------------------------------------------
# Define flags
flags_dict = {
    'fluid': 0,
    'solid': 1,
    'equilibrium': 2
    }

# --------------------------------------------------------------------------------------------
def unflatten(array: np.ndarray, shape: tuple) -> np.ndarray:
    """
    Unflattens a 1D array into a multi-dimensional array with the specified shape.
    
    Parameters:
        array (np.ndarray): The 1D array to unflatten.
        shape (tuple): The shape to unflatten the array into.
        
    Returns:
        np.ndarray: The unflattened multi-dimensional array.
    """
    return np.reshape(array, shape)

def xyz(n: int, shape: tuple) -> tuple:
    """
    Returns the x, y, z coordinates of index n in an array with the specified shape.
    
    Parameters:
        n (int): The index in the flattened array.
        shape (tuple): The shape of the multi-dimensional array.
        
    Returns:
        tuple: The (x, y, z) coordinates corresponding to the index n.
    """
    z = n // (shape[0] * shape[1])
    y = (n % (shape[0] * shape[1])) // shape[0]
    x = n % shape[0]
    return x, y, z

# --------------------------------------------------------------------------------------------
# Transpiler

import ast
import inspect
import textwrap

class PyClTranspiler(ast.NodeVisitor):
    def __init__(self):
        self.opencl_code = []
        self.indent_level = 0

    def visit_Module(self, node):
        # Start the OpenCL kernel with the standard header
        self.opencl_code.append("__kernel void initial_conditions(")
        self.opencl_code.append("    __global float* rho, __global float* u, __global int* flags, int Nx, int Ny, int Nz")
        self.opencl_code.append(") {")
        self.opencl_code.append("    int x = get_global_id(0);")
        self.opencl_code.append("    int y = get_global_id(1);")
        self.opencl_code.append("    int z = get_global_id(2);")
        self.opencl_code.append("")
        self.opencl_code.append("    if (x >= Nx || y >= Ny || z >= Nz) return;")
        self.opencl_code.append("")
        self.opencl_code.append("    int n = x * Ny * Nz + y * Nz + z;")
        self.indent_level += 1

        # Visit all statements in the module (skip the first row)
        for stmt in node.body:
            self.visit(stmt)

        # End the OpenCL kernel
        self.indent_level -= 1
        self.opencl_code.append("}")

    def visit_For(self, node):
        # Translate a for loop into OpenCL grid iteration
        if isinstance(node.iter, ast.Call) and isinstance(node.iter.func, ast.Name) and node.iter.func.id == "range":
            loop_var = node.target.id
            self.opencl_code.append(f"{'    ' * self.indent_level}int {loop_var} = get_global_id({loop_var});")
        else:
            raise NotImplementedError("Only range-based for loops are supported.")

        # Visit the body of the loop
        self.indent_level += 1
        for stmt in node.body:
            self.visit(stmt)
        self.indent_level -= 1

    def visit_If(self, node):
        # Translate the if condition
        condition = self.visit(node.test)
        self.opencl_code.append(f"{'    ' * self.indent_level}if ({condition}) {{")
        self.indent_level += 1

        # Visit the body of the if statement
        for stmt in node.body:
            self.visit(stmt)
        self.indent_level -= 1
        self.opencl_code.append(f"{'    ' * self.indent_level}}}")

        # Handle elif and else clauses
        if node.orelse:
            if isinstance(node.orelse[0], ast.If):
                # Process elif clauses
                for elif_clause in node.orelse:
                    self.visit(elif_clause)
            else:
                # Process else clause
                self.opencl_code.append(f"{'    ' * self.indent_level}else {{")
                self.indent_level += 1
                for stmt in node.orelse:
                    self.visit(stmt)
                self.indent_level -= 1
                self.opencl_code.append(f"{'    ' * self.indent_level}}}")

    def visit_Compare(self, node):
        # Translate a comparison (e.g., x == 0)
        left = self.visit(node.left)
        op = self.visit(node.ops[0])
        right = self.visit(node.comparators[0])
        return f"{left} {op} {right}"

    def visit_Eq(self, node):
        return "=="

    def visit_Name(self, node):
        # Translate variable names
        return node.id

    def visit_Constant(self, node):
        # Translate constants
        return str(node.value)

    def visit_Assign(self, node):
        # Translate variable assignments    
        target = self.visit(node.targets[0])
        value = self.visit(node.value)
        self.opencl_code.append(f"{'    ' * self.indent_level}{target} = {value};")

    def visit_Subscript(self, node):
        # Collect all nested indices (e.g., u[1] -> indices = [1])
        indices = []
        current_node = node
        while isinstance(current_node, ast.Subscript):
            # Extract the index from the subscript
            if isinstance(current_node.slice, ast.Index):
                index_node = current_node.slice.value  # Python <3.9
            else:
                index_node = current_node.slice  # Python >=3.9

            # Handle tuple indices (e.g., [x, y])
            if isinstance(index_node, ast.Tuple):
                indices.insert(0, [self.visit(e) for e in index_node.elts])
            else:
                indices.insert(0, self.visit(index_node))
            
            current_node = current_node.value  # Move to the parent node

        # Get the array name (e.g., 'u', 'rho')
        array_name = self.visit(current_node)

        # Flatten indices based on array type
        if array_name == 'u':
            # 4D array: u[component]
            if len(indices) != 1:
                raise ValueError(f"Array 'u' requires 1 indices (component). Got: {indices}")
            comp = int(indices[0])
            flat = f"n + {comp}"
        elif array_name in ('rho', 'flags'):
            # 3D array: rho
            if len(indices) != 0:
                raise ValueError(f"Array '{array_name}' requires 3 indices (x, y, z). Got: {indices}")
            flat = f"n"
        else:
            raise ValueError(f"Unsupported array: {array_name}")

        return f"{array_name}[{flat}]"

    def visit_Index(self, node):
        # Translate array indices
        return self.visit(node.value)

    def visit_Tuple(self, node):
        # Translate tuples (e.g., (x, y, z))
        elements = [self.visit(e) for e in node.elts]
        return ", ".join(elements)

    def translate(self, python_code):
        # Parse the Python code into an AST
        tree = ast.parse(python_code)

        # Visit the AST and generate OpenCL code
        self.visit(tree)
        return "\n".join(self.opencl_code)


def py_to_cl(python_function):
    """
    Translates a Python function into an OpenCL kernel.

    Args:
        python_function: The Python function to be translated.

    Returns:
        str: The generated OpenCL kernel code.
    """
    # Get the source code of the Python function
    source_code = inspect.getsource(python_function)

    # Skip the first row (function definition)
    source_code = "\n".join(source_code.splitlines()[1:])

    # Dedent the remaining code to remove the function's indentation
    source_code = textwrap.dedent(source_code)

    # Create a translator instance
    translator = PyClTranspiler()

    # Translate the Python code to OpenCL
    opencl_code = translator.translate(source_code)
    return opencl_code

# --------------------------------------------------------------------------------------------

def test_transpiller():
    # Example Python function
    def set_initial_conditions(x, y, z, rho, u, flags, Nx, Ny, Nz):
        if x == 0:
            rho = 1.0
        elif (y ==  6):
            u[1] = 0.0
        else:
            flags = 1
    # Example usage
    print("Original Python function:\n")
    print(inspect.getsource(set_initial_conditions))
    print("\nTranspiled function:\n")
    # Translate the Python function to OpenCL
    opencl_code = py_to_cl(set_initial_conditions)
    print(opencl_code)
    
    # Testing the translation
    import pyopencl as cl
    import numpy as np
    cl.ctx = cl.create_some_context()
    cl.queue = cl.CommandQueue(cl.ctx)
    Nx, Ny, Nz = 10, 10, 10
    # Initialize arrays
    rho = np.zeros((Nx, Ny, Nz), dtype=np.float32)
    u = np.zeros((Nx, Ny, Nz, 3), dtype=np.float32)
    flags = np.zeros((Nx, Ny, Nz), dtype=np.int32)
    # Create OpenCL buffers
    rho_buffer = cl.Buffer(cl.ctx, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=rho)
    u_buffer = cl.Buffer(cl.ctx, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=u)
    flags_buffer = cl.Buffer(cl.ctx, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=flags)
    # Start the OpenCL kernel
    kernel_code = py_to_cl(set_initial_conditions)
    program = cl.Program(cl.ctx, kernel_code).build()
    # Run the kernel
    program.initial_conditions(cl.queue, (Nx, Ny, Nz), None, rho_buffer, u_buffer, flags_buffer, np.int32(Nx), np.int32(Ny), np.int32(Nz))
    cl.enqueue_copy(cl.queue, rho.data, rho)
    print(rho)