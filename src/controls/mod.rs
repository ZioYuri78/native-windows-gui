/*!
    Holds various wrapper over Windows native controls, each in 
    their own module.
*/

mod base;
pub mod window;

pub use controls::window::Window;

use std::hash::Hash;
use winapi::HWND;

/**
    Trait that is shared by all control templates
*/
pub trait ControlTemplate<ID: Eq+Clone+Hash > {

    /**
        Create a new control from the template data.
    */
    fn create(&self,  ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()>;
}

pub fn cleanup() {
    unsafe { base::cleanup(); }
}

pub fn set_handle_data<T>(handle: HWND, data: T) {
    unsafe { base::set_handle_data(handle, data); }
}

pub fn get_handle_data<'a, T>(handle: HWND) -> &'a mut T {
    unsafe { base::get_handle_data(handle).unwrap() }
}

pub fn free_handle_data<T>(handle: HWND) {
    unsafe { base::free_handle_data::<T>(handle); }
}