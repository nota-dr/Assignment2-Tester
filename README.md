# **Computer Communication Assignment2 Tester**

Automated integration tests to evaluate students' Assignment 2 submissions in Computer Communication.\
This tester contains an additional axum server crate in order to check the client implemented in assignment2.

## Features
- Automated Testing: Streamlines the evaluation process by automating test execution.
- Comprehensive Coverage: Includes a variety of tests to ensure all aspects of the assignment are assessed.
- Detailed Logging: Generates logs for each test, aiding in debugging and result analysis.

## Prerequisites
- Before using the tester, ensure you have the following installed:
- Rust programing language
- Cargo package manager

### Additional crates
- axum server based on http protocol.

### Test Descriptions
The tester includes the following tests:

1. **Usage** 
- Verifies basic command-line argument options.
2. **Http Request Structure**
- Checks whether the raw HTTP request is constructed correctly.
3. **Http Request Structure With Params**
- Similar to the previous test but includes parameters.
4. **Response Contains Text**
- Ensures the response contains all expected headers and body content.
5. **Response Contains Image**
- Validates responses where the body contains non-printable bytes (e.g., images).
6. **Relative Redirect N Times**
- Checks the correct implementation of relative redirects.
7. **Absolute Redirect**
- Assesses the handling of absolute redirects.
8. **Life Cycle**
- Evaluates the overall functionality, including expected output sequences.
9. **Valgrind**
- Detects errors and memory leaks using Valgrind.

### Tester Usage
from parent directory:
1. **Prepare the Testee Directory**
- Place the student's assignment files inside the testee directory located in the working space root (parent directory).
2. **Execute the Tester**
Execute the following command ***from the working space root (parent directory)*** \
``` cargo run -p assignment2-tester --bin assignment2-tester ```\
This command will execute all the integration tests against the provided assignment files.
3. **Review Log Files**
- After execution, log files will be generated inside the testee directory.

### Axum Server Usage
for students that want to test their client against the server without running the tester.
from the parent directory:
1. **Execute Axum Server**
``` cargo run -p assignment2-tester --bin axum_server ```\
This command will run the server on localhost port 9090