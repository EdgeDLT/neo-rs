use std::error::Error;

pub fn get_asset_id_by_symbol(symbol: &str) ->Result<&str,Error>{
    let id = match symbol.to_lowercase().as_str() {
        "neo" => neo_core::consts::ASSET_ID_NEO,
        "gas"=>neo_core::consts::ASSET_ID_GAS,
        _ => {Err(())}
    };
    Ok(id)
}