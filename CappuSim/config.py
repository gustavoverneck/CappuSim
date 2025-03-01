# config.py

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
