mod helpers;

use crate::helpers::take_zeroed;
use failure::{Error, Fail};
use open_hardware_monitor_sys::{
  Computer_GetHardwares, Computer_GetReport, Computer_Open, Computer_new, FFIGCHandle,
  Hardware_GetSensors, Hardware_GetSubHardwares, Hardware_Update, Sensor_GetValue,
};
pub use open_hardware_monitor_sys::{FFIHardware, FFIHardwareType, FFISensor, FFISensorType};

#[derive(Debug, Fail)]
pub enum ComputerError {
  #[fail(display = "pointer is null")]
  Null,
}

pub struct Computer {
  inner: FFIGCHandle,
}
impl Computer {
  pub fn new() -> Result<Self, Error> {
    let inner = unsafe { Computer_new() };
    if inner.is_null() {
      return Err(ComputerError::Null.into());
    }

    Ok(Self { inner })
  }

  pub fn open() -> Result<Self, Error> {
    let computer = Self::new()?;

    unsafe {
      Computer_Open(&computer.inner);
    }

    Ok(computer)
  }

  pub fn get_report(&self) -> Result<String, Error> {
    let char_ptr = unsafe { Computer_GetReport(&self.inner) };

    Ok(char_ptr.to_string()?)
  }

  pub fn get_hardwares(&self) -> Result<Vec<Hardware>, Error> {
    let mut ffi_hardwares = unsafe { Computer_GetHardwares(&self.inner) };

    Ok(
      ffi_hardwares.items[..ffi_hardwares.length as usize]
        .iter_mut()
        .map(|ffi_hardware| Hardware::from_ffi(take_zeroed(ffi_hardware)))
        .collect::<Result<Vec<_>, _>>()?,
    )
  }
}

#[derive(Debug)]
pub struct Hardware {
  inner: FFIGCHandle,
  pub name: String,
  pub hardware_type: FFIHardwareType,
}
impl Hardware {
  fn from_ffi(ffi_hardware: FFIHardware) -> Result<Self, Error> {
    Ok(Self {
      inner: ffi_hardware.ptr,
      name: ffi_hardware.name.to_string()?,
      hardware_type: ffi_hardware.hardwareType,
    })
  }

  pub fn update(&self) {
    unsafe {
      Hardware_Update(&self.inner);
    }
  }

  pub fn get_sub_hardwares(&self) -> Result<Vec<Hardware>, Error> {
    let mut ffi_hardwares = unsafe { Hardware_GetSubHardwares(&self.inner) };

    Ok(
      ffi_hardwares.items[..ffi_hardwares.length as usize]
        .iter_mut()
        .map(|ffi_hardware| Hardware::from_ffi(take_zeroed(ffi_hardware)))
        .collect::<Result<Vec<_>, _>>()?,
    )
  }

  pub fn get_sensors(&self) -> Result<Vec<Sensor>, Error> {
    let mut ffi_sensors = unsafe { Hardware_GetSensors(&self.inner) };

    Ok(
      ffi_sensors.items[..ffi_sensors.length as usize]
        .iter_mut()
        .map(|ffi_sensor| Sensor::from_ffi(take_zeroed(ffi_sensor)))
        .collect::<Result<Vec<_>, _>>()?,
    )
  }
}

#[derive(Debug)]
pub struct Sensor {
  inner: FFIGCHandle,
  pub name: String,
  pub sensor_type: FFISensorType,
}
impl Sensor {
  fn from_ffi(ffi_sensor: FFISensor) -> Result<Self, Error> {
    Ok(Self {
      inner: ffi_sensor.ptr,
      name: ffi_sensor.name.to_string()?,
      sensor_type: ffi_sensor.sensorType,
    })
  }

  pub fn get_value(&self) -> f32 {
    unsafe { Sensor_GetValue(&self.inner) }
  }
}
