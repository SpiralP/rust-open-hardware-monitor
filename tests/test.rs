use open_hardware_monitor::*;

#[test]
fn test_computer() {
  let computer = Computer::open().unwrap();
  println!("{}", computer.get_report().unwrap());

  let hardwares = computer.get_hardwares().unwrap();
  println!("Hardwares: {:#?}", hardwares);

  let hardware = &hardwares[0];
  hardware.update();

  println!("SubHardwares: {:#?}", hardware.get_sub_hardwares());

  let sensors = hardware.get_sensors().unwrap();
  println!("Sensors: {:#?}", sensors);

  for sensor in &sensors {
    if sensor.sensor_type == FFISensorType::Temperature {
      println!("{:?} {}", sensor, sensor.get_value());
    }
  }

  println!();

  std::thread::sleep(std::time::Duration::from_secs(1));
  hardware.update();

  for sensor in &sensors {
    if sensor.sensor_type == FFISensorType::Temperature {
      println!("{:?} {}", sensor, sensor.get_value());
    }
  }
}
