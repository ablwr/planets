extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate stdweb;
extern crate astro;

use astro::*;
use stdweb::*;

#[derive(Serialize, Deserialize)]
struct Planette {
    long: f64,
    lat: f64,
    rad: f64,
    name: String,
}

js_serializable!(Planette);

fn main() {


    let time = stdweb::web::Date::now();
    let current_julian_day: f64 = ( time / 86400000.0 ) + 2440587.5;

    let day_of_month = time::DayOfMonth{
        day      :  7,
        hr       :  18,
        min      :  05,
        sec      :  0.0,
        time_zone: 0.0};

    let date = time::Date{
        year       : 2018,
        month      : 5,
        decimal_day: time::decimal_day(&day_of_month),
        cal_type   : time::CalType::Gregorian};

    let julian_day = time::julian_day(&date);

    let planets = [&planet::Planet::Mercury, &planet::Planet::Venus, 
                &planet::Planet::Earth, &planet::Planet::Mars,
                &planet::Planet::Jupiter, &planet::Planet::Saturn,
                &planet::Planet::Uranus, &planet::Planet::Neptune];

    fn planet_stats(planet: &planet::Planet, jd: f64) -> Planette {    
        let (long, lat, rad) = planet::heliocent_coords(planet, jd);

        let name = match planet {
            planet::Planet::Mercury => "Mercury",
            planet::Planet::Venus => "Venus",
            planet::Planet::Earth => "Earth",
            planet::Planet::Mars => "Mars",
            planet::Planet::Jupiter => "Jupiter",
            planet::Planet::Saturn => "Saturn",
            planet::Planet::Uranus => "Uranus",
            planet::Planet::Neptune => "Neptune",
        };

        return Planette{long: long, lat: lat, rad: rad, name: name.to_string()};
    }



    let mut pdata: Vec<Planette> = vec![];

    for p in planets.iter() {
        pdata.push( planet_stats(p, current_julian_day));
    }

    initialize();
    js! {
        var planets_data = @{&pdata};
        draw(planets_data);
    }
    event_loop();
}
