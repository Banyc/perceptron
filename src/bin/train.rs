use std::{fs, path::Path};

use crate_perceptron::{canvas::Canvas, perceptron::Perceptron};

fn main() {
    let rounds = 1000;
    let checkpoint = "checkpoints/";

    let canvas_width = 512;
    let canvas_height = 512;

    let mut canvas = Canvas::new(canvas_width, canvas_height);

    let checkpoint = Path::new(checkpoint);
    let perceptron_weight_checkpoint = checkpoint.join("perceptron/weight.npy");
    let perceptron_bias_checkpoint = checkpoint.join("perceptron/bias.txt");

    let weight = match perceptron_weight_checkpoint.exists() {
        true => {
            let x = ndarray_npy::read_npy(&perceptron_weight_checkpoint).unwrap();
            Some(x)
        }
        false => None,
    };
    let bias = match perceptron_bias_checkpoint.exists() {
        true => {
            let x = fs::read_to_string(&perceptron_bias_checkpoint).unwrap();
            let x: f64 = x.parse().unwrap();
            Some(x)
        }
        false => None,
    };

    let mut perceptron = Perceptron::new(canvas.height() * canvas.width(), weight, bias);

    let mut corrects = 0;
    for i in 0..rounds {
        canvas.clear();
        let label = match i % 2 == 0 {
            true => {
                canvas.draw_random_circle();
                true
            }
            false => {
                canvas.draw_random_rectangle();
                false
            }
        };
        let point_vector = canvas.one_dimensional_array();
        let is_correct = perceptron.train_and_is_correct(point_vector, label);
        if is_correct {
            corrects += 1;
        }
        if i % 100 == 0 {
            println!("i: {}, %correct: {}", i, corrects as f64 / 100.0);
            corrects = 0;
        }
    }

    fs::create_dir_all(&perceptron_weight_checkpoint.parent().unwrap()).unwrap();
    ndarray_npy::write_npy(&perceptron_weight_checkpoint, perceptron.weight()).unwrap();
    fs::create_dir_all(&perceptron_bias_checkpoint.parent().unwrap()).unwrap();
    fs::write(&perceptron_bias_checkpoint, perceptron.bias().to_string()).unwrap();
}
