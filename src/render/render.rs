use crate::models::model::Program as Program;
use std::io as io;
use std::fs as fs;

fn config_to_string(configpath: String) -> io::Result<String> {
    let config: String = fs::read_to_string(configpath).expect("Could not read config file");

    Ok(config)
}

pub fn render_programs(configpath: &String) -> io::Result<Vec<Program>>{
    let config: String = fs::read_to_string(configpath).expect("Could not read config file");
    
    let output_buffer = Vec::new();
    
    let mut counter = 0;

    println!("----------------------");

    config.split("\n").for_each(|line| {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        println!("{} - {}", counter, key);
        counter += 1;
    });
    Ok(output_buffer)

}

#[cfg(test)]
mod tests {
    use std::{vec, clone};

    use super::*;

    #[test]
    fn test_render_programs(){
        
        struct TestCase {
            test_name: String,
            test_input: String,
            test_input_sample_contents: String,
            test_exp_output: Vec<Program>
        }

        let TestCases = [
            TestCase{
                test_name: "Valid input".to_string(),
                test_input: "Dummyfile.conf".to_string(),
                test_input_sample_contents:"Outlook: Work\nNFL: GameTwitter\nSafari: Work".to_string(),
                test_exp_output: vec![Program{
                    name: "MyProgram".to_string(),
                    vendor: "MyProgramType".to_string()
                }]
            }
        ];

        for testCase in TestCases {


            std::fs::write(&testCase.test_input, testCase.test_input_sample_contents).expect("Could not write to file");

            let test_output = render_programs(&testCase.test_input);
            
            let out = test_output.unwrap();


            // out.iter().for_each(|program| {
            //     assert_eq!(&program.name, &testCase.test_exp_output[0].name);
            //     assert_eq!(&program.vendor, &testCase.test_exp_output[0].vendor);
            // });
        
            assert_eq!(*out[0].name.to_string(), testCase.test_exp_output[0].name.to_string());
        }
        

    }
        
}