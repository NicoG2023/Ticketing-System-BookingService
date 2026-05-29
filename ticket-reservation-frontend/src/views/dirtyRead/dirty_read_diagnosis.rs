use dioxus::prelude::*;

use crate::models::DirtyReadSimulationResponse;

const DIRTY_READ_DIAGNOSIS_CSS: Asset =
    asset!("/assets/styling/dirty_read/dirty_read_diagnosis.css");

#[component]
pub fn DirtyReadDiagnosis(simulation: Option<DirtyReadSimulationResponse>) -> Element {
    let diagnosis = simulation
        .as_ref()
        .map(|state| state.diagnosis.clone())
        .unwrap_or_else(|| "Inicia la simulación para ver el diagnóstico paso a paso.".to_string());

    let explanation = simulation
        .as_ref()
        .map(|state| state.explanation.clone())
        .unwrap_or_else(|| {
            "Un Dirty Read ocurre cuando una sesión lee datos temporales que otra sesión todavía no ha confirmado.".to_string()
        });

    let control = simulation
        .as_ref()
        .map(|state| state.control.clone())
        .unwrap_or_else(|| {
            "Este problema se evita impidiendo que otras sesiones lean datos no confirmados."
                .to_string()
        });

    let dirty_read_detected = simulation
        .as_ref()
        .map(|state| state.dirty_read_detected)
        .unwrap_or(false);

    let completed = simulation
        .as_ref()
        .map(|state| state.current_step == "COMPLETED")
        .unwrap_or(false);

    let class_name = if completed && dirty_read_detected {
        "dirty-read-diagnosis dirty-read-diagnosis--danger"
    } else if completed {
        "dirty-read-diagnosis dirty-read-diagnosis--success"
    } else {
        "dirty-read-diagnosis"
    };

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_DIAGNOSIS_CSS
        }

        section {
            class: "{class_name}",

            div {
                class: "dirty-read-diagnosis__main",

                span {
                    class: "dirty-read-diagnosis__badge",
                    if completed && dirty_read_detected {
                        "DIRTY READ DETECTADO"
                    } else if completed {
                        "SIN ANOMALÍA"
                    } else {
                        "DIAGNÓSTICO"
                    }
                }

                h3 {
                    if completed && dirty_read_detected {
                        "La Sesión B leyó información que nunca fue confirmada"
                    } else if completed {
                        "No se detectó una lectura sucia"
                    } else {
                        "Resultado de la simulación"
                    }
                }

                p {
                    "{diagnosis}"
                }
            }

            div {
                class: "dirty-read-diagnosis__details",

                article {
                    h4 { "¿Qué significa?" }
                    p { "{explanation}" }
                }

                article {
                    h4 { "¿Cómo se controla?" }
                    p { "{control}" }
                }
            }
        }
    }
}
