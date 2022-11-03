use bgpkit_parser::BgpkitParser;
use rustler::{Env, ListIterator, MapIterator, NifResult, Term};
use serde_json::{Result, Value};
use serde_rustler::{from_term, to_term};
use std::collections::{hash_map, HashMap};

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn mrt_parser<'a>(env: Env<'a>, path: String, filters: MapIterator<'a>) -> NifResult<Term<'a>> {
    let mut list = Term::list_new_empty(env);
    let parser = BgpkitParser::new(&path).unwrap();
    let elements = filters.fold(parser, |filtered, (k, v)| {
        filtered
            .add_filter(
                &k.decode::<String>().unwrap(),
                &v.decode::<String>().unwrap(),
            )
            .unwrap()
    });

    // let elements = BgpkitParser::new(&path).unwrap();
    for elem in elements {
        let mut map = Term::map_new(env);
        let str = serde_json::to_string(&elem).unwrap();
        let hm: HashMap<String, Value> = serde_json::from_str(&str).unwrap();
        map = Term::map_put(map, "timestamp", hm.get("timestamp").unwrap().as_f64()).unwrap();
        map = Term::map_put(map, "type", hm.get("type").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "local_pref", hm.get("local_pref").unwrap().as_i64()).unwrap();
        map = Term::map_put(map, "med", hm.get("med").unwrap().as_i64()).unwrap();
        map = Term::map_put(map, "peer_asn", hm.get("peer_asn").unwrap().as_i64()).unwrap();

        list = Term::list_prepend(list, map)
    }
    // TODO: serde_rustler::Serializer raises next error, need to fix it
    // the trait bound `serde_rustler::Serializer<'_>: From<Env<'a>>` is not satisfied
    // the trait `From<rustler::env::Env<'a>>` is implemented for `serde_rustler::Serializer<'a>`
    // for elem in BgpkitParser::new(&path).unwrap() {
    //     let str = serde_json::to_string(&elem).unwrap();
    //     serde_transcode::transcode(
    //         &mut serde_json::Deserializer::from_str(&str),
    //         serde_rustler::Serializer::from(env),
    //     )
    // .map_err(|e| e.into());
    // let hm: HashMap<String, Value> = serde_json::from_str(&str).unwrap();
    // for (k, v) in hm.iter() {
    //     map = Term::map_put(map, k, v.to_string()).unwrap();
    // }
    // list = Term::list_prepend(list, map);
    // }
    Ok(list)
}

rustler::init!("Elixir.RustlerExperiment.Native", [add, mrt_parser]);
