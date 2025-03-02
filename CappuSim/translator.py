import ast
import inspect
import textwrap

class PythonToOpenCLTranslator(ast.NodeVisitor):
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
        # Translate an if statement
        condition = self.visit(node.test)
        self.opencl_code.append(f"{'    ' * self.indent_level}if ({condition}) {{")
        self.indent_level += 1

        # Visit the body of the if statement
        for stmt in node.body:
            self.visit(stmt)
        self.indent_level -= 1
        self.opencl_code.append(f"{'    ' * self.indent_level}}}")

        # Handle elif and else clauses
        for elif_clause in node.orelse:
            if isinstance(elif_clause, ast.If):
                # Translate elif clause
                elif_condition = self.visit(elif_clause.test)
                self.opencl_code.append(f"{'    ' * self.indent_level}else if ({elif_condition}) {{")
                self.indent_level += 1
                for stmt in elif_clause.body:
                    self.visit(stmt)
                self.indent_level -= 1
                self.opencl_code.append(f"{'    ' * self.indent_level}}}")
            else:
                # Translate else clause
                self.opencl_code.append(f"{'    ' * self.indent_level}else {{")
                self.indent_level += 1
                self.visit(elif_clause)  # Visit the else clause directly
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
    # Collect all indices recursively
    indices = []
    current_node = node
    while isinstance(current_node, ast.Subscript):
        if isinstance(current_node.slice, ast.Index):
            index = self.visit(current_node.slice.value)
        else:
            index = self.visit(current_node.slice)
        indices.insert(0, index)  # Insert at start to maintain order
        current_node = current_node.value
    array_name = self.visit(current_node)

    # Generate flattened index based on array type
    if array_name == 'u':
        if len(indices) != 4:
            raise ValueError("u requires 4 indices (x, y, z, component)")
        x, y, z, comp = indices
        flat = f"({x} * Ny * Nz * 3) + ({y} * Nz * 3) + ({z} * 3) + {comp}"
    elif array_name in ('rho', 'flags'):
        if len(indices) != 3:
            raise ValueError(f"{array_name} requires 3 indices (x, y, z)")
        x, y, z = indices
        flat = f"({x} * Ny * Nz) + ({y} * Nz) + {z}"
    else:
        raise ValueError(f"Unknown array: {array_name}")

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
    translator = PythonToOpenCLTranslator()

    # Translate the Python code to OpenCL
    opencl_code = translator.translate(source_code)
    return opencl_code


# Example Python function
def set_initial_conditions(x, y, z, rho, u, flags, Nx, Ny, Nz):
    if x == 0:
        rho[x,y,z] = 1.0
    elif (y ==  6):
        u[x,y,z][1] = 0.0
    else:
        flags[x,y,z] = 1


if __name__ == "__main__":
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