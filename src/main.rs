//! This example shows that you can use egui in parallel from multiple threads.
use std::sync::{ Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::f64::consts::PI;
use eframe::glow::BUFFER;
use egui::plot::{Line, Plot, PlotPoints};

const BUFSIZE : usize = 10000;

fn main()-> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).


    let mut data_generator = DataGenerator::new();

    let reader: Arc<Mutex<DataGenerator>> = Arc::new(Mutex::new(data_generator));
    let writer: Arc<Mutex<DataGenerator>> = reader.clone();
    let data_configurer: Arc<Mutex<DataGenerator>> = reader.clone();


    thread::spawn(move || {

        let mut sleep : u64;
        loop {
            {
                let mut data_generator = writer.lock().unwrap();
                let elements = data_generator.calculate_new_y();
                sleep = data_generator.sample_period;
            }
            thread::sleep(Duration::from_micros(sleep));

        }
    });

    let app = MyApp::new(reader, data_configurer);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 512.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(app)),
    )

}

#[derive(Clone)]
struct DataGenerator {
    index : usize,
    pub x: Vec<f64>, //[f64;BUFSIZE],
    pub y: Vec<f64>, //[f64;BUFSIZE],
    start_time: Instant,
    freq: f64,
    sample_period: u64
}

impl DataGenerator { 

    fn new() -> DataGenerator{
        DataGenerator {
            x : vec![0.0; BUFSIZE], //Vec::<f64>,//[0.0; BUFSIZE],
            y : vec![0.0; BUFSIZE],//[0.0; BUFSIZE],
            start_time: Instant::now(),
            freq : 1.0,
            index : 0,
            sample_period : 50,
        }
    }

    fn calculate_new_y(&mut self) -> (std::vec::Vec<f64>,std::vec::Vec<f64>){

        let time_s = self.start_time.elapsed().as_secs_f64();
        //self.x[self.index] = time_s;
        if self.index < BUFSIZE-1 {
            self.index += 1;
        }
        else{
            self.index = 0;
        }

        self.x[self.index] = time_s;
        self.y[self.index] = ((2.0*PI*self.freq)*time_s*1e3 ).cos();

        

        (self.x.clone(), self.y.clone())
    }
}

struct MyApp {
    data_source : Arc<Mutex<DataGenerator>>,
    data_configurer : Arc<Mutex<DataGenerator>>,
    freq: f64,
    sample_period: u64,
}

impl MyApp {
    fn new(data_source : Arc<Mutex<DataGenerator>>, data_configurer : Arc<Mutex<DataGenerator>>) -> MyApp{
        MyApp {
            data_source: data_source,
            data_configurer: data_configurer,
            freq : 1.0,
            sample_period: 50,
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let old_freq = self.freq;
        let old_sample_period = self.sample_period;

        let ydata;  // self.receiver.recv().unwrap().1;
        let xdata;  // self.receiver.recv().unwrap().1;

        {
            ydata = self.data_source.lock().unwrap().y.clone();
            xdata = self.data_source.lock().unwrap().x.clone();
            
        }
        //thread::sleep(Duration::from_millis(100));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plotter");

            let sin: PlotPoints = (0..(BUFSIZE)).map(|i| {
                [ xdata[i] ,ydata[i]]
            }).collect();
            
            //let sin = PlotPoints::from_ys_f64(&ydata);

            
            let line = Line::new(sin);
            let plt = Plot::new("my_plot").view_aspect(3.0).include_y(1.0).include_y(-1.0);
            
            plt.show(ui, |plot_ui| plot_ui.line(line));
            //animate the plot
            ui.ctx().request_repaint();
            //.allow_scroll(false);
            // plt.reset();
            
            ui.add(egui::Slider::new(&mut self.freq, 0.001..=100.0).text("Frequency (kHz)").logarithmic(true));
            if (old_freq != self.freq){
                self.data_configurer.lock().unwrap().freq = self.freq;
            }

            ui.add(egui::Slider::new(&mut self.sample_period, 1..=1000).text("sample_period (us)"));
            if (old_sample_period != self.sample_period){
                self.data_configurer.lock().unwrap().sample_period = self.sample_period;
            }

        });
    }

}







