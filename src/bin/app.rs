

fn main() {
    println!("Hello, world!"); 

    pollster::block_on(market_economy_simulation::run());
}