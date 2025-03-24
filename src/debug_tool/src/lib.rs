use std::any::type_name;

/// TOOLS for debuging

/// Affiche le type d'une variable
///
/// # Arguments
/// * `&T` - Une variable quelconque
///
/// # Example
/// ```rust
/// let ma_variable = {};
/// print_type_of(&ma_variable);
/// ```
pub fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}
