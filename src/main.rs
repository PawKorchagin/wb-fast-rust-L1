use std::collections::HashMap;

fn group_temperatures(temps: Vec<f64>) -> HashMap<String, Vec<f64>> {
    let mut map: HashMap<String, Vec<f64>> = HashMap::new();
    
    for &temp in temps.iter() {
        let lower_bound = (temp / 10.0).floor() * 10.0;
        let upper_bound = lower_bound + 10.0;
        
        let key = format!("[{}, {})", lower_bound, upper_bound);
        
        map.entry(key)
            .or_insert_with(Vec::new)
            .push(temp);
    }
    
    map
}

fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let grouped = group_temperatures(temperatures);
    
    for (range, temps) in grouped.iter() {
        println!("{}: {:?}", range, temps);
    }
}
