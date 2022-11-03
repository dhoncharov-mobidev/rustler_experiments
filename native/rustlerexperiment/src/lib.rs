use bgpkit_parser::BgpkitParser;
use rustler::{Env, MapIterator, NifResult, Term};
use serde_json::Value;
use std::collections::HashMap;

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

    for elem in elements {
        let mut map = Term::map_new(env);
        let str = serde_json::to_string(&elem).unwrap();
        let hm: HashMap<String, Value> = serde_json::from_str(&str).unwrap();
        map = Term::map_put(map, "timestamp", hm.get("timestamp").unwrap().as_f64()).unwrap();
        map = Term::map_put(map, "type", hm.get("type").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "peer_asn", hm.get("peer_asn").unwrap().as_i64()).unwrap();
        map = Term::map_put(map, "peer_ip", hm.get("peer_ip").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "prefix", hm.get("prefix").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "next_hop", hm.get("next_hop").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "as_path", hm.get("as_path").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "origin_asns", hm.get("origin_asns").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "origin", hm.get("origin").unwrap().as_str()).unwrap();
        map = Term::map_put(map, "local_pref", hm.get("local_pref").unwrap().as_i64()).unwrap();
        map = Term::map_put(map, "med", hm.get("med").unwrap().as_i64()).unwrap();
        list = Term::list_prepend(list, map)
    }
    Ok(list)
}

rustler::init!("Elixir.RustlerExperiment.Native", [add, mrt_parser]);
