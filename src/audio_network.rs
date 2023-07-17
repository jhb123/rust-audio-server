use std::{time::Instant, f64::consts::PI, time::Duration};
use std::{thread};
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub struct AudioNetwork {
    index : usize,
    pub x: Arc<Mutex<[f64;10000]>>,
    pub y: Arc<Mutex<[f64;10000]>>,
    start_time: Instant,
    freq: f64,
    pub running: bool,

}

impl AudioNetwork{

    pub fn new() -> AudioNetwork{
        AudioNetwork {
            x : Arc::new(Mutex::new([0.0; 10000])),
            y : Arc::new(Mutex::new([0.0; 10000])),
            start_time: Instant::now(),
            freq : 50.0,
            running : true,
            index : 0
        }
    }

    pub fn read_array(&self) -> [f64; 10000] {
        let array: std::sync::MutexGuard<'_, [f64; 10000]> = self.y.lock().unwrap();
        array.clone()
    }

    pub fn write_array(&mut self) {
        self.calculate_new_y();
    }


    fn calculate_new_y(&mut self) {

        let time_s = self.start_time.elapsed().as_secs_f64();
        //self.x[self.index] = time_s;
        let mut x_array = self.x.lock().unwrap();
        let mut y_array = self.x.lock().unwrap();

        x_array[self.index] = time_s;
        y_array[self.index] = ((2.0*PI*self.freq)*time_s ).cos();

        if self.index < 9999 {
            self.index += 1;
        }
        else{
            self.index = 0;
        }
    }

}