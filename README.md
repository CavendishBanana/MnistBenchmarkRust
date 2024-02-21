
Recommended to open and update mnist_model file in text editor instead of IDE - the file stores network weights as u8 literals, so IDE may fail to open the file due to its size.
Currently input file path is given directly as str literal in the main function body, so - as for now -  giving the program another input data location requires updating the main function body and saving there new path to input data file. 
