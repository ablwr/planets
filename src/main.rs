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

// pulled from stdweb examples
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn planet_stats(planet: &Planet, jd: f64, earth_view: bool) -> Planette {  

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

    if earth_view == true {
        let (points, rad) = astro::planet::geocent_apprnt_ecl_coords(planet, jd);
        return Planette{long: points.long, lat: points.lat, rad: rad, name: name.to_string()};
    }  else {
        let (long, lat, rad) = astro::planet::heliocent_coords(planet, jd);
        return Planette{long: long, lat: lat, rad: rad, name: name.to_string()};
    }
}


fn draw_planets(planets: [&astro::planet::Planet; 8], date: f64, earth_view: bool) {

    let mut new_pdata: Vec<Planette> = vec![];
    for p in planets.iter() {
        &new_pdata.push( planet_stats(p, date, earth_view));
    }

    js! { setup(); }
    if earth_view == false { js! { drawSun()}}
    js! { draw(@{&new_pdata});} 
}


fn main() {
    
    initialize();

    let mut earth_view: bool = false;

    let planets = [&Planet::Mercury, &Planet::Venus, &Planet::Earth, &Planet::Mars,
                   &Planet::Jupiter, &Planet::Saturn, &Planet::Uranus, &Planet::Neptune];

    // initial setup
    let time = Date::now();
    let date: f64 = ( time / 86400000.0 ) + 2440587.5;
    let mut pdata: Vec<Planette> = vec![];
    for p in planets.iter() {
        &pdata.push( planet_stats(p, date, earth_view));
    }
    js! {
        setup();
        drawSun();
        draw(@{&pdata});
    }

    let date_selector: html_element::InputElement = document().query_selector( "#date" ).unwrap().unwrap().try_into().unwrap();
    date_selector.add_event_listener( enclose!( (date_selector) move |_: event::InputEvent| {

         let date: f64 = js! {
            var d1 = new Date(@{&date_selector.raw_value()});
            var d2 = Date.parse(d1.toString());
            var d3 = (d2/86400000.0)+2440587.5;
            return parseFloat(d3);
        }.try_into().expect("no integers, dude");

        draw_planets(planets, date, earth_view);
    }));

    let view_selector: html_element::InputElement = document().query_selector( "#view" ).unwrap().unwrap().try_into().unwrap();
    view_selector.add_event_listener( enclose!( (view_selector, date) move |_: event::ClickEvent| {
        
        if earth_view == false {
            view_selector.set_raw_value("Earth");
            earth_view = true;
        } else {
            view_selector.set_raw_value("Sun");
            earth_view = false;
        } 
        // TODO: Earth button resets time, it should stick with the chosen time
        draw_planets(planets, date, earth_view);
    }));

    event_loop();
}
