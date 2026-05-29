use dioxus::prelude::*;

const DIRTY_READ_CONTROLS_CSS: Asset = asset!("/assets/styling/dirty_read/dirty_read_controls.css");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyReadAction {
    Start,
    StartSessionA,
    WriteUncommitted,
    ReadBySessionB,
    RollbackSessionA,
    ReadFinalConfirmed,
    Diagnose,
    Reset,
}

#[component]
pub fn DirtyReadControls(
    current_step: String,
    has_selected_event: bool,
    loading: bool,
    on_action: EventHandler<DirtyReadAction>,
) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_CONTROLS_CSS
        }

        section {
            class: "dirty-read-controls",

            div {
                class: "dirty-read-controls__header",

                div {
                    p { "Ejecución paso a paso" }
                    h3 { "Control de la simulación" }
                }

                span {
                    class: "dirty-read-controls__step",
                    "{label_for_step(&current_step)}"
                }
            }

            div {
                class: "dirty-read-controls__actions",

                StepButton {
                    number: "1",
                    label: "Iniciar simulación",
                    description: "Lee la capacidad confirmada desde Firebase.",
                    disabled: !can_execute(&current_step, DirtyReadAction::Start, has_selected_event, loading),
                    active: current_step == "NOT_STARTED",
                    onclick: move |_| on_action.call(DirtyReadAction::Start)
                }

                StepButton {
                    number: "2",
                    label: "Sesión A inicia",
                    description: "A abre una operación temporal no confirmada.",
                    disabled: !can_execute(&current_step, DirtyReadAction::StartSessionA, has_selected_event, loading),
                    active: current_step == "INITIALIZED",
                    onclick: move |_| on_action.call(DirtyReadAction::StartSessionA)
                }

                StepButton {
                    number: "3",
                    label: "A escribe temporal",
                    description: "A descuenta capacidad solo en memoria.",
                    disabled: !can_execute(&current_step, DirtyReadAction::WriteUncommitted, has_selected_event, loading),
                    active: current_step == "SESSION_A_STARTED",
                    onclick: move |_| on_action.call(DirtyReadAction::WriteUncommitted)
                }

                StepButton {
                    number: "4",
                    label: "B lee capacidad",
                    description: "B lee el valor no confirmado.",
                    disabled: !can_execute(&current_step, DirtyReadAction::ReadBySessionB, has_selected_event, loading),
                    active: current_step == "SESSION_A_UNCOMMITTED_WRITE",
                    onclick: move |_| on_action.call(DirtyReadAction::ReadBySessionB)
                }

                StepButton {
                    number: "5",
                    label: "A hace rollback",
                    description: "A descarta el valor temporal.",
                    disabled: !can_execute(&current_step, DirtyReadAction::RollbackSessionA, has_selected_event, loading),
                    active: current_step == "SESSION_B_DIRTY_READ",
                    onclick: move |_| on_action.call(DirtyReadAction::RollbackSessionA)
                }

                StepButton {
                    number: "6",
                    label: "Leer final",
                    description: "Consulta la capacidad confirmada real.",
                    disabled: !can_execute(&current_step, DirtyReadAction::ReadFinalConfirmed, has_selected_event, loading),
                    active: current_step == "SESSION_A_ROLLBACK",
                    onclick: move |_| on_action.call(DirtyReadAction::ReadFinalConfirmed)
                }

                StepButton {
                    number: "7",
                    label: "Diagnosticar",
                    description: "Compara lo leído por B contra el valor final.",
                    disabled: !can_execute(&current_step, DirtyReadAction::Diagnose, has_selected_event, loading),
                    active: current_step == "FINAL_CONFIRMED_READ",
                    onclick: move |_| on_action.call(DirtyReadAction::Diagnose)
                }

                StepButton {
                    number: "↺",
                    label: "Reiniciar",
                    description: "Vuelve a iniciar la simulación para este evento.",
                    disabled: !can_execute(&current_step, DirtyReadAction::Reset, has_selected_event, loading),
                    active: false,
                    onclick: move |_| on_action.call(DirtyReadAction::Reset)
                }
            }
        }
    }
}

#[component]
fn StepButton(
    number: &'static str,
    label: &'static str,
    description: &'static str,
    disabled: bool,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class_name = if active {
        "dirty-read-controls__button dirty-read-controls__button--active"
    } else {
        "dirty-read-controls__button"
    };

    rsx! {
        button {
            class: "{class_name}",
            disabled,
            onclick: move |event| onclick.call(event),

            span {
                class: "dirty-read-controls__button-number",
                "{number}"
            }

            span {
                class: "dirty-read-controls__button-copy",
                strong { "{label}" }
                small { "{description}" }
            }
        }
    }
}

fn can_execute(
    current_step: &str,
    action: DirtyReadAction,
    has_selected_event: bool,
    loading: bool,
) -> bool {
    if loading || !has_selected_event {
        return false;
    }

    match action {
        DirtyReadAction::Start => current_step == "NOT_STARTED",
        DirtyReadAction::StartSessionA => current_step == "INITIALIZED",
        DirtyReadAction::WriteUncommitted => current_step == "SESSION_A_STARTED",
        DirtyReadAction::ReadBySessionB => current_step == "SESSION_A_UNCOMMITTED_WRITE",
        DirtyReadAction::RollbackSessionA => current_step == "SESSION_B_DIRTY_READ",
        DirtyReadAction::ReadFinalConfirmed => current_step == "SESSION_A_ROLLBACK",
        DirtyReadAction::Diagnose => current_step == "FINAL_CONFIRMED_READ",
        DirtyReadAction::Reset => current_step != "NOT_STARTED",
    }
}

fn label_for_step(step: &str) -> &'static str {
    match step {
        "NOT_STARTED" => "No iniciada",
        "INITIALIZED" => "Inicializada",
        "SESSION_A_STARTED" => "Sesión A iniciada",
        "SESSION_A_UNCOMMITTED_WRITE" => "Valor temporal creado",
        "SESSION_B_DIRTY_READ" => "B leyó valor temporal",
        "SESSION_A_ROLLBACK" => "Rollback ejecutado",
        "FINAL_CONFIRMED_READ" => "Valor final leído",
        "COMPLETED" => "Completada",
        _ => "Estado desconocido",
    }
}
