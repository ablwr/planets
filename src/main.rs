extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate stdweb;
extern crate astro;

use astro::planet::*;
use stdweb::*;
use stdweb::web::*;
use stdweb::unstable::TryInto;

#[derive(Serialize, Deserialize)]
struct Planette {
    long: f64,
    lat: f64,
    rad: f64,
    name: String,
}

js_serializable!(Planette);

// the finest cargo culting
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    
    initialize();

    let planets = [&Planet::Mercury, &Planet::Venus, &Planet::Mars,
                   &Planet::Jupiter, &Planet::Saturn, &Planet::Uranus, &Planet::Neptune];

    fn planet_stats(planet: &Planet, jd: f64) -> Planette {    
        let (points, rad) = astro::planet::geocent_apprnt_ecl_coords(planet, jd);

        let name = match planet {
            Planet::Mercury => "Mercury",
            Planet::Venus => "Venus",
            Planet::Earth => "Earth",
            Planet::Mars => "Mars",
            Planet::Jupiter => "Jupiter",
            Planet::Saturn => "Saturn",
            Planet::Uranus => "Uranus",
            Planet::Neptune => "Neptune",
        };
        return Planette{long: points.long, lat: points.lat, rad: rad, name: name.to_string()};
    }

    let time = Date::now();
    let current_julian_day: f64 = ( time / 86400000.0 ) + 2440587.5;
    let mut pdata: Vec<Planette> = vec![];
    for p in planets.iter() {
        &pdata.push( planet_stats(p, current_julian_day as f64));
    }
    js! {
        setup();
        draw(@{&pdata});
    }

    let date_selector: html_element::InputElement = document().query_selector( "#date" ).unwrap().unwrap().try_into().unwrap();
    date_selector.add_event_listener( enclose!( (date_selector) move |_: event::InputEvent| {
        
        let chosen_date: f64 = js! {
            var d1 = new Date(@{&date_selector.raw_value()});
            var d2 = Date.parse(d1.toString());
            var d3 = (d2/86400000.0)+2440587.5;
            return parseFloat(d3);
        }.try_into().expect("no integers, dude");

        let mut new_pdata: Vec<Planette> = vec![];
        for p in planets.iter() {
            &new_pdata.push( planet_stats(p, chosen_date as f64));
        }

        js! {
            setup();
            draw(@{&new_pdata});
        }  
    }));

    event_loop();
}
