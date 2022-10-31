#![allow(non_snake_case, unused_variables,dead_code)]

use ShaderMathEngine::run;

fn main() {
    pollster::block_on(run());
}
