mod monitor;

use std::{env, thread, time::Duration, sync::{Arc, Mutex}};
use colored::Colorize;
use chrono::{Local};
use std::{fs, time::SystemTime};
use monitor::{MonitorData, Result};
use rand::Rng;
static TERMINATION_FLAG: Mutex<bool> = Mutex::new(false);
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Process command line arguments
    let command_line_args = process_command_line_args(&args);

    // Check if monitor file path is provided
    if let Some(monitor_file_path) = command_line_args.monitor_file_path {
        // Attempt to read MonitorData from file
        match MonitorData::from_file(&monitor_file_path) {
            Ok(mut data) => {
                // Generate random results for each monitor
                data = data.with_random_results();
                
                // Convert MonitorData to JSON
                let json_data = serde_json::to_string_pretty(&data).expect("Failed to serialize MonitorData to JSON");

                // Print JSON data
                println!("{}", json_data);
                
                // Deserialize JSON string back to MonitorData
                let deserialized_data: MonitorData = serde_json::from_str(&json_data)
                    .expect("Failed to deserialize from JSON");

                // Print deserialized MonitorData
                println!("{:?}", deserialized_data);

                // Process monitors
                process_monitors(data);
            }
            Err(err) => eprintln!("Error reading monitor file: {}", err),
        }
    } else {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
    }
}

struct CommandLineArgs {
    monitor_file_path: Option<String>,
}

impl CommandLineArgs {
    fn new() -> Self {
        CommandLineArgs {
            monitor_file_path: None,
        }
    }
}

fn process_command_line_args(args: &[String]) -> CommandLineArgs {
    let mut parsed_args = CommandLineArgs::new();
    let mut index = 1;

    while index < args.len() {
        match args[index].as_str() {
            "-monitorFile" => {
                if index + 1 < args.len() {
                    parsed_args.monitor_file_path = Some(args[index + 1].clone());
                    index += 2;
                } else {
                    println!("Error: -monitorFile option requires a value.");
                    return CommandLineArgs::new();
                }
            }
            _ => {
                println!("Error: Unknown option '{}'", args[index]);
                return CommandLineArgs::new();
            }
        }
    }

    parsed_args
}


fn process_monitors(monitor_data: MonitorData) {
    let shared_data = Arc::new(Mutex::new(monitor_data));
    let shared_data_clone = Arc::clone(&shared_data);
  
    // Start thread for updating monitors
    let update_thread = thread::spawn(move || {
      update_monitors(shared_data_clone);
    });
  
    // Start thread for storing monitors
    let store_thread = thread::spawn(move || {
      store_monitors(shared_data);
    });
  
    // Track start time
    let start_time = std::time::SystemTime::now();
  
    // Terminate process_monitors after 5 minutes
    let five_minutes = Duration::from_secs(30);
  
    
  
    // Loop with termination condition
    loop {
      if let Ok(elapsed) = start_time.elapsed() {
        if elapsed >= five_minutes {
          
          break;
        }
      }
      thread::sleep(Duration::from_secs(1));
      
    }
  
    // Join threads to ensure they complete their tasks before exiting
    update_thread.join().expect("Update thread panicked");
    store_thread.join().expect("Store thread panicked");
  }

  fn update_monitors(shared_data: Arc<Mutex<MonitorData>>) {
    let update_interval = Duration::from_secs(30);
  
    loop {
      // Check termination flag
      if *TERMINATION_FLAG.lock().unwrap() {
        break;
      }
  
      let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
  
      {
        let mut data = shared_data.lock().unwrap();
        for monitor in &mut data.monitors {
          let random_value = rand::thread_rng().gen::<i32>();
          let new_result = Result {
            value: random_value,
            processed_at: current_time as i64,
          };
          monitor.result = Some(new_result);
        }
      }
  
      thread::sleep(update_interval);
    }
  }
  
  fn store_monitors(shared_data: Arc<Mutex<MonitorData>>) {
    let store_interval = Duration::from_secs(60);
  
    loop {
      // Check termination flag
      if *TERMINATION_FLAG.lock().unwrap() {
        break;
      }
  
      let local_time = Local::now();
      let formatted_time = format!("{}", local_time.format("%d_%m_%Y_%l-%M%P").to_string().replace(" ", ""));
    
      let filename = format!("{}_monitors.json", formatted_time);
  
      {
        let data = shared_data.lock().unwrap();
        let json_data = serde_json::to_string_pretty(&*data).expect("Failed to serialize MonitorData to JSON");
        fs::write(&filename, json_data).expect("Failed to write JSON data to file");
      }
  
      println!("{}: {}", "Stored monitors in file".yellow(), filename.green());
  
      thread::sleep(store_interval);
    }
  }
  



