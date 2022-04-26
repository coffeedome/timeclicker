use crate::models::model::Program as Program;
use std::io as io;
use std::fs as fs;

fn config_to_string(configpath: String) -> io::Result<String> {
    let config: String = fs::read_to_string(configpath).expect("Could not read config file");

    Ok(config)
}

pub fn render_programs(configpath: String) -> io::Result<Vec<Program>>{
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
    use std::vec;

    use super::*;

    #[test]
    fn test_render_programs(){
        
        struct TestCase {
            testName: String,
            testInput: String,
            testExpOutput: Vec<Program>
        }

        let TestCases = [
            TestCase{
                testName: "Valid input".to_string(),
                testInput: "Dummyfile.conf".to_string(),
                testExpOutput: vec![Program{
                    name: "MyProgram".to_string(),
                    vendor: "MyProgramType".to_string()
                }]
            }
        ];

        for testCase in TestCases {
            let testOutput = render_programs(testCase.testInput);
        }

    }
        
}