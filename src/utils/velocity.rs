/// A structure representing a 3D velocity vector with `x`, `y`, and `z` components.
///
/// # Examples
///
/// ```
/// use crate::utils::velocity::Velocity;
///
/// let velocity = Velocity::new(1.0, 2.0, 3.0);
/// println!("{:?}", velocity);
/// ```
///
/// # Fields
///
/// * `x` - The velocity component along the x-axis.
/// * `y` - The velocity component along the y-axis.
/// * `z` - The velocity component along the z-axis.
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    /// Creates a new `Velocity` instance with the specified `x`, `y`, and `z` components.
    ///
    /// # Parameters
    ///
    /// * `x` - The velocity component along the x-axis.
    /// * `y` - The velocity component along the y-axis.
    /// * `z` - The velocity component along the z-axis.
    ///
    /// # Returns
    ///
    /// A new `Velocity` instance with the given components.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::velocity::Velocity;
    ///
    /// let velocity = Velocity::new(1.0, 2.0, 3.0);
    /// assert_eq!(velocity.x, 1.0);
    /// assert_eq!(velocity.y, 2.0);
    /// assert_eq!(velocity.z, 3.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Velocity { x, y, z }
    }

    /// Creates a new `Velocity` instance with all components set to zero.
    ///
    /// # Returns
    ///
    /// A `Velocity` instance with `x`, `y`, and `z` components all set to `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::velocity::Velocity;
    ///
    /// let zero_velocity = Velocity::zero();
    /// assert_eq!(zero_velocity.x, 0.0);
    /// assert_eq!(zero_velocity.y, 0.0);
    /// assert_eq!(zero_velocity.z, 0.0);
    /// ```
    pub fn zero() -> Self {
        Velocity { x: 0.0, y: 0.0, z: 0.0 }
    }
}
