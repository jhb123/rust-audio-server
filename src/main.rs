//! This example shows that you can use egui in parallel from multiple threads.
use std::sync::{ Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::f64::consts::PI;
use egui::plot::{Line, Plot, PlotPoints};

const BUFSIZE : usize = 10000;

fn main()-> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).


    let mut data_generator = DataGenerator::new();

    let reader: Arc<Mutex<DataGenerator>> = Arc::new(Mutex::new(data_generator));
    let writer: Arc<Mutex<DataGenerator>> = reader.clone();
    let data_configurer: Arc<Mutex<DataGenerator>> = reader.clone();


    thread::spawn(move || {

        loop {
            {
                let mut data_generator = writer.lock().unwrap();
                let elements = data_generator.calculate_new_y();
            }
            thread::sleep(Duration::from_micros(50));
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

#[derive(Copy,Clone)]
struct DataGenerator {
    index : usize,
    pub x: [f64;BUFSIZE],
    pub y: [f64;BUFSIZE],
    start_time: Instant,
    freq: f64,
}

impl DataGenerator { 

    fn new() -> DataGenerator{
        DataGenerator {
            x : [0.0; BUFSIZE],
            y : [0.0; BUFSIZE],
            start_time: Instant::now(),
            freq : 1.0,
            index : 0
        }
    }

    fn calculate_new_y(&mut self) -> ([f64;BUFSIZE],[f64;BUFSIZE]){

        let time_s = self.start_time.elapsed().as_secs_f64();
        //self.x[self.index] = time_s;
        if self.index < BUFSIZE-1 {
            self.index += 1;
        }
        else{
            self.index = 0;
        }

        self.x[self.index] = time_s;
        self.y[self.index] = ((2.0*PI*self.freq)*time_s ).cos();

        

        (self.x, self.y)
    }
}

struct MyApp {
    data_source : Arc<Mutex<DataGenerator>>,
    data_configurer : Arc<Mutex<DataGenerator>>,
    freq: f64,
}

impl MyApp {
    fn new(data_source : Arc<Mutex<DataGenerator>>, data_configurer : Arc<Mutex<DataGenerator>>) -> MyApp{
        MyApp {
            data_source: data_source,
            data_configurer: data_configurer,
            freq : 5.0
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let old_freq = self.freq;

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
                [ xdata[i]*100000.0 ,ydata[i]]
            }).collect();
            
            //let sin = PlotPoints::from_ys_f64(&ydata);

            
            let line = Line::new(sin);
            let plt = Plot::new("my_plot").view_aspect(3.0).include_y(1.0).include_y(-1.0);
            
            plt.show(ui, |plot_ui| plot_ui.line(line));
            //animate the plot
            ui.ctx().request_repaint();
            //.allow_scroll(false);
            // plt.reset();
            
            ui.add(egui::Slider::new(&mut self.freq, 1.0..=10.0).text("Frequency"));
            if (old_freq != self.freq){
                self.data_configurer.lock().unwrap().freq = self.freq;
            }

        });
    }

}







