use quizdown::{process_questions_str, Config, OutputFormat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn default_config() -> JsValue {
    JsValue::from_serde(&Config::default()).unwrap()
}

#[wasm_bindgen]
pub fn render_questions(
    text: &str,
    name: &str,
    format: JsValue,
    config: JsValue,
) -> Result<String, JsValue> {
    let format: OutputFormat = format.into_serde().unwrap();
    let config: Config = config.into_serde().unwrap();
    let parsed =
        process_questions_str(text, Some(config)).map_err(|e| format!("Parsing Error: {:?}", e))?;
    Ok(format
        .render(name, &parsed)
        .map_err(|e| format!("Rendering Error: {:?}", e))?)
}
