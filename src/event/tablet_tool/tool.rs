use crate::{ffi, AsRaw, FromRaw};

/// Available tool types for a device with the `DeviceCapability::TabletTool` capability.
///
/// The tool type defines the default usage of the tool as advertised by the
/// manufacturer. Multiple different physical tools may share the same tool type, e.g. a
/// Wacom Classic Pen, Wacom Pro Pen and a Wacom Grip Pen are all of type
/// `TabletToolType::Pen`. Use `TabletTool::tool_id` to get a specific model where
/// applicable.
///
/// Note that on some device, the eraser tool is on the tail end of a pen device. On
/// other devices, e.g. MS Surface 3, the eraser is the pen tip while a button is held
/// down.
///
/// ## Note
///
/// The `TabletToolType` can only describe the default physical type of the device. For
/// devices with adjustable physical properties the tool type remains the same, i.e.
/// putting a Wacom stroke nib into a classic pen leaves the tool type as
/// `TabletToolType::Pen`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TabletToolType {
    /// A generic pen.
    Pen,
    /// Eraser.
    Eraser,
    /// A paintbrush-like tool.
    Brush,
    /// Physical drawing tool, e.g. Wacom Inking Pen
    Pencil,
    /// An airbrush-like tool.
    Airbrush,
    /// A mouse bound to the tablet.
    Mouse,
    /// A mouse tool with a lens.
    Lens,
    /// A rotary device with positional and rotation data
    #[cfg(feature = "libinput_1_14")]
    Totem,
}

ffi_ref_struct! {
    /// An object representing a tool being used by a device with the
    /// `DeviceCapability::TabletTool` capability.
    ///
    /// Tablet events generated by such a device are bound to a specific tool rather than
    /// coming from the device directly. Depending on the hardware it is possible to track
    /// the same physical tool across multiple `Device`s, see
    /// [Tracking unique tools](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-serial-numbers).
    struct TabletTool, ffi::libinput_tablet_tool, ffi::libinput_tablet_tool_ref, ffi::libinput_tablet_tool_unref
}

impl TabletTool {
    ffi_func!(
    /// Return the serial number of a tool.
    ///
    /// If the tool does not report a serial number, this function returns zero.
    /// See [Tracking unique tools](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-serial-numbers) for details.
    pub fn serial, ffi::libinput_tablet_tool_get_serial, u64);
    ffi_func!(
    /// Return the tool ID for a tool object.
    ///
    /// If nonzero, this number identifies the specific type of the tool with more
    /// precision than the type returned in `tool_type`,
    /// see [Vendor-specific tablet tool types](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-tool-types).
    /// Not all tablets support a tool ID.
    ///
    /// Tablets known to support tool IDs include the Wacom Intuos 3, 4, 5, Wacom Cintiq
    /// and Wacom Intuos Pro series.
    pub fn tool_id, ffi::libinput_tablet_tool_get_tool_id, u64);

    /// Return the tool type for a tool object,
    /// see [Vendor-specific tablet tool types](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-tool-types)
    /// for details.
    ///
    /// A return value of `None` means the tool type is not known.
    pub fn tool_type(&self) -> Option<TabletToolType> {
        match unsafe { ffi::libinput_tablet_tool_get_type(self.as_raw_mut()) } {
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_PEN => {
                Some(TabletToolType::Pen)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_ERASER => {
                Some(TabletToolType::Eraser)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_BRUSH => {
                Some(TabletToolType::Brush)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_PENCIL => {
                Some(TabletToolType::Pencil)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_AIRBRUSH => {
                Some(TabletToolType::Airbrush)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_MOUSE => {
                Some(TabletToolType::Mouse)
            }
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_LENS => {
                Some(TabletToolType::Lens)
            }
            #[cfg(feature = "libinput_1_14")]
            ffi::libinput_tablet_tool_type_LIBINPUT_TABLET_TOOL_TYPE_TOTEM => {
                Some(TabletToolType::Totem)
            }
            _x => {
                #[cfg(feature = "log")]
                log::warn!("Unknown `TabletToolType` returned by libinput: {}", _x);
                None
            }
        }
    }

    /// Check if a tablet tool has a button with the passed-in code (see linux/input.h).
    pub fn has_button(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_button(self.as_raw_mut(), button) != 0 }
    }

    ffi_func!(
    /// Return whether the tablet tool supports distance.
    pub fn has_distance, ffi::libinput_tablet_tool_has_distance, bool);
    ffi_func!(
    /// Return whether the tablet tool supports pressure.
    pub fn has_pressure, ffi::libinput_tablet_tool_has_pressure, bool);
    ffi_func!(
    /// Return whether the tablet tool supports z-rotation.v
    pub fn has_rotation, ffi::libinput_tablet_tool_has_rotation, bool);
    ffi_func!(
    /// Return whether the tablet tool has a slider axis.
    pub fn has_slider, ffi::libinput_tablet_tool_has_slider, bool);
    ffi_func!(
    /// Return whether the tablet tool supports tilt.
    pub fn has_tilt, ffi::libinput_tablet_tool_has_tilt, bool);
    ffi_func!(
    /// Return whether the tablet tool has a relative wheel.
    pub fn has_wheel, ffi::libinput_tablet_tool_has_wheel, bool);
    ffi_func!(
    /// Returns `true` if the physical tool can be uniquely identified by libinput, or
    /// `false` otherwise.
    ///
    /// If a tool can be uniquely identified, keeping a reference to the tool allows
    /// tracking the tool across proximity out sequences and across compatible tablets.
    /// See [Tracking unique tools](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-serial-numbers)
    /// for more details.
    pub fn is_unique, ffi::libinput_tablet_tool_is_unique, bool);
    #[cfg(feature = "libinput_1_14")]
    ffi_func!(
    /// Returns whether the tablet tool has a ellipsis major and minor.
    ///
    /// Where the underlying hardware only supports one of either major or minor,
    /// libinput emulated the other axis as a cicular contact, i.e. major == minor
    /// for all values of major.
    pub fn tablet_tool_has_size, ffi::libinput_tablet_tool_has_size, bool);
}
