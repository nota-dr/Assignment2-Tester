pub mod hw2_tests;
use tests_lib::*;

pub fn add_tests(tm: &mut tests_lib::TestManager) -> Vec<String> {
    let tests_templates = vec![
        TestTemplateBuilder::new("Usage1")
            .args_template("./client http://httpbin.org -r 10 pr1=1 pr2=2 pr3=3 pr4=4 pr5=5 pr6=6 pr7=7 pr8=8 pr9=9")
            .agent(Box::new(|| Box::new(hw2_tests::Usage)))
            .validate_timeout(5)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("Usage2")
            .args_template("./client -r 10 pr1=1 pr2=2 pr3=3 pr4=4 5 pr6=6 pr7=7 pr8=8 pr9=9 pr10=10 http://httpbin.org")
            .agent(Box::new(|| Box::new(hw2_tests::Usage)))
            .validate_timeout(5)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("HTTP Request Structure")
            .args_template("./client http://httpbin.org")
            .agent(Box::new(|| Box::new(hw2_tests::HttpRequestStructure)))
            .validate_timeout(20)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("HTTP Response Contains Text")
            .args_template("./client -r 1 country=israel http://universities.hipolabs.com/search")
            .agent(Box::new(|| Box::new(hw2_tests::ResponseContainsText)))
            .validate_timeout(20)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("HTTP Response Contains Image")
            .args_template("./client http://localhost:9090/resources/meow.png")
            .agent(Box::new(|| Box::new(hw2_tests::ResponseContainsImage)))
            .validate_timeout(30)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("Relative Redirect N Times")
            .args_template("./client http://localhost:9090/recursive/3")
            .agent(Box::new(|| Box::new(hw2_tests::RelativeRedirectTimes)))
            .validate_timeout(20)
            .log_output(true)
            .valgrind(true)
            .build(),
        TestTemplateBuilder::new("Absolute Redirect")
            .args_template("./client http://localhost:9090/absolute")
            .agent(Box::new(|| Box::new(hw2_tests::AbsoluteRedirect)))
            .validate_timeout(20)
            .log_output(true)
            .valgrind(true)
            .build(),
        TestTemplateBuilder::new("Life Cycle")
            .args_template("./client http://localhost:9090/resources?file=Approval.gif")
            .agent(Box::new(|| Box::new(hw2_tests::LifeCycle)))
            .validate_timeout(30)
            .log_output(true)
            .valgrind(true)
            .build(),
        TestTemplateBuilder::new("Valgrind")
            .agent(Box::new(|| Box::new(hw2_tests::Valgrind)))
            .validate_timeout(30)
            .log_output(true)
            .build()
    ];
    let mut keys = Vec::new();
    for template in tests_templates {
        let key = tm.register_template(template);
        tm.instantiate_test(&key, None);
        keys.push(key);
    }
    keys
}
