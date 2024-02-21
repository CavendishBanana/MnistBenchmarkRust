//Krzysztof Gryko
//Program zakłada, że komputer korzysta z reprezentacji little-endian
mod mnist_model;
use nalgebra::DMatrix;
//use std::env;
//use std::fs;
use std::fs::read_to_string;
//use mnist_model::get_input_image_dimensions;
use rand::Rng;
use std::time::Instant;
const rng_nums_count :usize = 100000;

fn prepare_data(filename: &str, samples_count :u32) -> (Vec<DMatrix<f32>>, Vec<u8>) {
    let mut string_lines :Vec<String> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        string_lines.push(line.to_string())
    }
	let one_by_255 :f64 = (1.0f64/255.0f64) as f64;
	let mut samples_counter :u32 = 0u32; 
	let input_dimensions :Vec<usize> = mnist_model::get_input_image_dimensions();
	let mut images :Vec<DMatrix<f32>> = Vec::new(); 
	let mut labels :Vec<u8> = Vec::new();
	for string_line in string_lines
	{
		if samples_counter >= samples_count
		{
			break;
		}
		
		let line_contents = string_line.split(",");
		let mut i = 0;
		let mut label :u8 = 0u8;
		let mut pixels :Vec<f32> = Vec::new();
		for string_number in line_contents
		{
			if i == 0
			{
				let label_tmp = string_number.parse::<i32>().unwrap();
				label = label_tmp as u8;
			}
			else
			{
				let pixel_i32 :i32 = string_number.parse::<i32>().unwrap();
				let pixel_f32 :f32 = ((pixel_i32 as f64)*one_by_255 ) as f32;
				pixels.push(pixel_f32);
			}
			
			i+=1;
		}
		let mut image_matrix :DMatrix<f32> = DMatrix::from_vec(input_dimensions[0], input_dimensions[1], pixels);
		images.push(image_matrix);
		labels.push(label);
	}
	let mut result = (images, labels);
    return result;
}


fn main() {
	
	let file_path = ".\\data\\mnist_train.csv";
	
	let samples_count :u32 = 10000u32;

	let (mut data, labels) :(Vec<DMatrix<f32>>,Vec<u8>) = prepare_data(file_path, samples_count);
	println!("Data loadaed");
	let input_dimensions :Vec<usize> = mnist_model::get_input_image_dimensions();
	
	//let mut matrices : Vec<Vec<DMatrix<f32>>> = mnist_model::get_matrices();
	let mut activation_functions :Vec<mnist_model::ActivationFunctionPtr> = vec![];
	unsafe{
		activation_functions = mnist_model::get_activation_functions_pointers();
	}
	
	let mut weights_matrices : Vec<DMatrix<f32>> = mnist_model::get_layers_weights();
	let mut biases_matrices : Vec<DMatrix<f32>> = mnist_model::get_layers_biases();
	let mut result_matrices_1 : Vec<DMatrix<f32>> = mnist_model::get_results_matrices();
	let mut result_matrices_2 : Vec<DMatrix<f32>> = mnist_model::get_results_matrices();
	
	let mut dummy_input_matrix :DMatrix<f32> = DMatrix::from_element( input_dimensions[0], input_dimensions[1], 0.0f32 );
	
	let layers_count :usize = mnist_model::get_model_layers_count();
	println!("Model prepared");
	let mut predicted_number :u8 = 0u8;

	let mut incorrect_correct_guesses = vec![0u32, 0u32];
	println!("Begin test");
	let start_test = Instant::now();
    unsafe{
        for i in 0..samples_count
        {
        predicted_number = mnist_model::make_prediction_2(&mut data[i as usize], &mut weights_matrices, &mut biases_matrices, &mut result_matrices_1, &mut result_matrices_2, &activation_functions, layers_count);		
        (incorrect_correct_guesses[ (predicted_number == labels[i as usize]) as usize ])+=1; 
		}
        
    }
    let duration_test = start_test.elapsed();
	println!("Test complete");
	let correct_guesses_fraction = (incorrect_correct_guesses[1] as f64)/(samples_count as f64);
	let incorrect_guesses_fraction = (incorrect_correct_guesses[0] as f64)/(samples_count as f64);
	println!("Prediction time for {} samples: {:?}, correct guesses: {}, correct guesses fraction: {}, incorrect guesses: {}, incorrect guesses fraction: {}", samples_count, duration_test, incorrect_correct_guesses[1], correct_guesses_fraction, incorrect_correct_guesses[0], incorrect_guesses_fraction);

}


/*
	let litt_end = mnist_model::is_little_endian();
	println!("little endian {}", litt_end);
	let big_end = mnist_model::is_big_endian();
	println!("big endian {}", big_end);
	
	let mut rng = rand::thread_rng();

    let mut vals: Vec<f32> = vec![0.0f32; 100000000 as usize];
	for i in 0..vals.len()
	{
		let num: f32 = rng.gen_range(-1000.0f32..1000.0f32);
		vals[i] = num;
	}
	let mut x :f32 = 0.0f32;
	let start_test_relu = Instant::now();
    unsafe{
        for i in 0..vals.len()
        {
			x = relu(x);
		}
        }
    let duration_test_relu = start_test_relu.elapsed();
	println!("tricky relu: {:?}", duration_test_relu);
	let start_test_relu_2 = Instant::now();
    unsafe{
        for i in 0..vals.len()
        {
			x = relu_2(x);
		}
        }
    let duration_test_relu_2 = start_test_relu_2.elapsed();
	println!("po bozemu relu: {:?}", duration_test_relu_2);
	*/
	/*
	println!("Make convert number test");
	convert_number();
	println!("convert number test done");
	
	//let wtf_vec = vec![1,2,3,4,5,6,7,8];
	let wtf_cols = 10;
	let wtf_rows = 5;
	let wtf_n = wtf_rows * wtf_cols;
	//let wtf_vec :Vec<> = ((1 as usize)..((wtf_n + 1) as usize)).collect();
	
	let mut wtf_vec = vec![0; wtf_n];
	for i in 0..wtf_vec.len()
	{
		wtf_vec[i] = (i as i32) + 1i32;
	}
	
	let mut wtf_matrix :DMatrix<i32> = DMatrix::from_vec(wtf_cols, wtf_rows, wtf_vec);
	wtf_matrix = wtf_matrix.transpose();
	println!("Test matrix initialization: ");
	println!("[");
	for i in 0..wtf_matrix.nrows()
	{
		let mut str_buff = "".to_string();
		for j in 0..wtf_matrix.ncols()
		{
			str_buff = str_buff + " " +  &wtf_matrix[(i,j)].to_string();
		}
		println!("{}", str_buff);
	}
	println!("]");
	*/



