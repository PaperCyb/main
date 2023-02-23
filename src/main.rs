
const SUN:i32 = 1;
const BLACK_HOLE:i32 = 1;
const HOLE: i32 = 3;


fn main() {
    let planet = 9;
    println!("There are multiple bodies in this solar system.");
    println!("there are {} sun(s), {} Planet(s) and {} singularity.", SUN, planet, BLACK_HOLE);
    println!("Ever since the black hole entered the solar system, it has eaten {} plant(s).", eat_sum(planet, HOLE, BLACK_HOLE));
    let planet = planet - eat_sum(planet, HOLE, BLACK_HOLE);
    println!("There are now {} planets left",planet)

}

fn eat_sum(a: i32, b: i32,c: i32) -> i32 {
    a - b - c
}