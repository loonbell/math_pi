use std::f64;
use std::io;

fn main() 
{
    let mut _input = String::new();
    let mut _rayon:u32 = 0;

    while _rayon <= 1
    {
        println!("Enter the radius of the circle for estimation (32 bits unsigned integer) : ");
        io::stdin()
            .read_line(&mut _input)
            .expect("failed to read from stdin");
        let trimmed = _input.trim();
        match trimmed.parse::<u32>()
        {
            Ok(i) =>_rayon = i,
            Err(..) => println!("this is not a 32 bits unsigned integer : {}", trimmed),
        };

        if _rayon == 1
        {
            println!("Radius needs to be > 1");
        }

        _input.clear();
    }

    let mut _circle_x:u32 = _rayon/2;
    let mut _circle_y:f64 = 0.0;

    _circle_y = where_is_y(_circle_x, _rayon);

    let mut _old_circle_x:u64 = 0;
    let mut _old_circle_y:f64 = 0.0;

    let mut _estimated_circle_circonf:f64 = 0.0;

    while _circle_x >= 1
    {
        _old_circle_y = _circle_y;
        _circle_x -= 1;
        _circle_y = where_is_y(_circle_x, _rayon);

        _estimated_circle_circonf += 
            dif_calculation(_circle_y, _old_circle_y);

        // status check
        if _rayon > 10 && _circle_x != 0 && 0 == _circle_x % (_rayon/10)
        {
            println!("{} left", _circle_x);
        }
    }
    println!("Done!");

    _estimated_circle_circonf = _estimated_circle_circonf * 12.0;

    let _pi;

    _pi = (_estimated_circle_circonf / 2.0) / _rayon as f64;

    println!("The estimation of Pi is : {}", _pi);
}

//dif between 2 points
fn dif_calculation(coor_y: f64, old_coor_y:f64) -> f64
{
    let mut _dif_square:f64;
    let mut _dif_y;
    _dif_y = coor_y - old_coor_y;
    _dif_square = 1.0 + (_dif_y * _dif_y);

    _dif_square.sqrt()
}

//where is y relatively to the new x
fn where_is_y(x:u32, rayon:u32) -> f64
{
    let square_y:f64;
    square_y = (rayon as f64 * rayon as f64) - (x as f64 * x as f64);
    square_y.sqrt()
}
