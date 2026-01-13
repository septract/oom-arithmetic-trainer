use leptos::{
    component, create_signal, event_target_value, store_value, view, IntoView, ReadSignal,
    Show, SignalGet, SignalSet, SignalUpdate, WriteSignal,
};

use crate::challenge::{format_number, generate_challenges, get_daily_seed, Challenge};
use crate::parser::parse_answer;
use crate::scoring::{evaluate, ScoreResult};

const PROBLEMS_PER_DAY: usize = 5;

const MAGNITUDES: &[(f64, &str)] = &[(1e3, "K"), (1e6, "M"), (1e9, "B"), (1e12, "T")];

fn format_answer_display(value: f64) -> String {
    if value < 1.0 {
        return "0".to_string();
    }
    for &(mag, suffix) in MAGNITUDES.iter().rev() {
        if value >= mag {
            let scaled = value / mag;
            if scaled >= 100.0 {
                return format!("{scaled:.0}{suffix}");
            } else if scaled >= 10.0 {
                return format!("{scaled:.1}{suffix}");
            }
            return format!("{scaled:.2}{suffix}");
        }
    }
    format!("{value:.0}")
}

fn get_direction_indicator(user: f64, correct: f64) -> (&'static str, &'static str) {
    if user > correct * 1.05 {
        ("Too high", "high")
    } else if user < correct * 0.95 {
        ("Too low", "low")
    } else {
        ("", "")
    }
}

fn progress_percent(current: usize, total: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let percent = (current as f64 / total as f64) * 100.0;
    percent
}

#[component]
fn ProgressBar(current: ReadSignal<usize>, total_score: Box<dyn Fn() -> u32>) -> impl IntoView {
    view! {
        <div class="progress-container">
            <div class="progress-bar">
                <div
                    class="progress-fill"
                    style:width=move || format!("{}%", progress_percent(current.get(), PROBLEMS_PER_DAY))
                ></div>
            </div>
            <div class="progress-text">
                <span>{move || format!("Problem {} of {}", current.get() + 1, PROBLEMS_PER_DAY)}</span>
                <span>{move || format!("Score: {}", total_score())}</span>
            </div>
        </div>
    }
}

#[component]
fn ProblemCard(challenge: Challenge) -> impl IntoView {
    let operator = if challenge.is_division { "/" } else { "x" };
    view! {
        <div class="problem-card">
            <div class="problem-label">"Estimate"</div>
            <div class="problem">
                <span class="num">{format_number(challenge.num1)}</span>
                <span class="operator">{operator}</span>
                <span class="num">{format_number(challenge.num2)}</span>
                <span class="operator">"="</span>
                <span class="question">"?"</span>
            </div>
        </div>
    }
}

#[component]
fn ResultCard(
    result: ScoreResult,
    user_answer: f64,
    correct: f64,
    current_index: ReadSignal<usize>,
    on_next: Box<dyn Fn()>,
) -> impl IntoView {
    let result_class = match result {
        ScoreResult::Exact | ScoreResult::Close => "result-card correct",
        ScoreResult::Partial => "result-card close",
        ScoreResult::Wrong => "result-card wrong",
    };
    let (direction_text, direction_class) = get_direction_indicator(user_answer, correct);
    let label = result.label();

    view! {
        <div>
            <div class=result_class>
                <div class="result-label">{label}</div>
                <div class="result-details">
                    <div>
                        "You: "
                        <span class="your-answer">{format_number(user_answer)}</span>
                        {(!direction_text.is_empty()).then(|| view! {
                            <span class=format!("direction {direction_class}")>{direction_text}</span>
                        })}
                    </div>
                    <div>
                        "Answer: "
                        <span class="correct-answer">{format_number(correct)}</span>
                    </div>
                </div>
            </div>
            <button class="next-btn" on:click=move |_| on_next()>
                {move || if current_index.get() + 1 >= PROBLEMS_PER_DAY { "See Results" } else { "Next Problem" }}
            </button>
        </div>
    }
}

#[component]
fn MagnitudeButtons(
    answer_value: ReadSignal<f64>,
    set_answer_value: WriteSignal<f64>,
    set_user_input: WriteSignal<String>,
) -> impl IntoView {
    let adjust = move |multiplier: f64| {
        set_answer_value.update(|v| {
            let new_val = if *v < 1.0 { 1000.0 } else { *v * multiplier };
            *v = new_val.clamp(0.0, 1e15);
        });
        let val = answer_value.get();
        if val >= 1.0 {
            set_user_input.set(format_answer_display(val));
        }
    };

    view! {
        <div>
            {move || {
                let val = answer_value.get();
                let class_name = if val >= 1.0 { "answer-display has-value" } else { "answer-display" };
                view! {
                    <div class=class_name>
                        {if val >= 1.0 {
                            view! { <span class="answer-value">{format_answer_display(val)}</span> }.into_view()
                        } else {
                            view! { <span class="answer-placeholder">"Use buttons below"</span> }.into_view()
                        }}
                    </div>
                }
            }}
            <div class="magnitude-controls">
                <div class="magnitude-row">
                    <button class="mag-btn increase" on:click=move |_| adjust(1000.0)>"+1000x"</button>
                    <button class="mag-btn increase" on:click=move |_| adjust(100.0)>"+100x"</button>
                    <button class="mag-btn increase" on:click=move |_| adjust(10.0)>"+10x"</button>
                    <button class="mag-btn increase" on:click=move |_| adjust(2.0)>"+2x"</button>
                </div>
                <div class="magnitude-row">
                    <button class="mag-btn decrease" on:click=move |_| adjust(0.001)>"/1000"</button>
                    <button class="mag-btn decrease" on:click=move |_| adjust(0.01)>"/100"</button>
                    <button class="mag-btn decrease" on:click=move |_| adjust(0.1)>"/10"</button>
                    <button class="mag-btn decrease" on:click=move |_| adjust(0.5)>"/2"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TextInput(
    user_input: ReadSignal<String>,
    set_user_input: WriteSignal<String>,
    set_answer_value: WriteSignal<f64>,
    on_submit: Box<dyn Fn()>,
    has_answer: Box<dyn Fn() -> bool>,
) -> impl IntoView {
    view! {
        <div class="text-input-wrapper">
            <input
                type="text"
                placeholder="e.g. 400B, 4e11"
                prop:value=move || user_input.get()
                on:input=move |ev| {
                    set_user_input.set(event_target_value(&ev));
                    set_answer_value.set(0.0);
                }
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" && has_answer() { on_submit(); }
                }
            />
            <div class="input-hint">"Formats: 400B, 400 billion, 4e11, 4x10^11"</div>
        </div>
    }
}

#[component]
fn InputModeToggle(input_mode: ReadSignal<bool>, set_input_mode: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: center; gap: 0.5rem; margin-bottom: 1rem;">
            <button
                class="mag-btn"
                style:background=move || if input_mode.get() { "var(--accent)" } else { "var(--bg-card)" }
                style:color=move || if input_mode.get() { "#fff" } else { "var(--text-primary)" }
                on:click=move |_| set_input_mode.set(true)
            >"Buttons"</button>
            <button
                class="mag-btn"
                style:background=move || if input_mode.get() { "var(--bg-card)" } else { "var(--accent)" }
                style:color=move || if input_mode.get() { "var(--text-primary)" } else { "#fff" }
                on:click=move |_| set_input_mode.set(false)
            >"Type"</button>
        </div>
    }
}

fn count_results(results: &[(ScoreResult, f64, f64)]) -> (usize, usize, usize) {
    let exact = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Exact | ScoreResult::Close)).count();
    let partial = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Partial)).count();
    let wrong = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Wrong)).count();
    (exact, partial, wrong)
}

#[component]
fn CompleteScreen(
    results: ReadSignal<Vec<(ScoreResult, f64, f64)>>,
    total_score: Box<dyn Fn() -> u32>,
) -> impl IntoView {
    view! {
        <div class="complete-screen">
            <div class="complete-title">"Session Complete"</div>
            <div class="complete-score">{move || total_score()}</div>
            <div class="complete-subtitle">{format!("out of {} points", PROBLEMS_PER_DAY * 100)}</div>
            <div class="score-breakdown">
                {move || {
                    let (exact, partial, wrong) = count_results(&results.get());
                    view! {
                        <>
                            <div class="breakdown-row">
                                <span class="breakdown-label">"Correct (within 0.5 OOM)"</span>
                                <span class="breakdown-value correct">{exact}</span>
                            </div>
                            <div class="breakdown-row">
                                <span class="breakdown-label">"Close (within 1 OOM)"</span>
                                <span class="breakdown-value close">{partial}</span>
                            </div>
                            <div class="breakdown-row">
                                <span class="breakdown-label">"Off (1+ OOM)"</span>
                                <span class="breakdown-value wrong">{wrong}</span>
                            </div>
                        </>
                    }
                }}
            </div>
            <div class="come-back">"New problems tomorrow!"</div>
        </div>
    }
}


#[component]
pub fn App() -> impl IntoView {
    let seed = get_daily_seed();
    let challenges = store_value(generate_challenges(seed, PROBLEMS_PER_DAY));
    let (current_index, set_current_index) = create_signal(0usize);
    let (user_input, set_user_input) = create_signal(String::new());
    let (answer_value, set_answer_value) = create_signal(0.0f64);
    let (submitted, set_submitted) = create_signal(false);
    let (score_results, set_score_results) = create_signal(Vec::<(ScoreResult, f64, f64)>::new());
    let (input_mode, set_input_mode) = create_signal(true);

    let current_challenge = move || challenges.with_value(|c| c.get(current_index.get()).copied());
    let total_score = move || score_results.get().iter().map(|(r, _, _)| r.points()).sum::<u32>();
    let is_complete = move || current_index.get() >= PROBLEMS_PER_DAY;
    let has_answer = move || answer_value.get() >= 1.0 || !user_input.get().is_empty();

    let do_submit = move || {
        if submitted.get() { return; }
        let user_answer = if answer_value.get() >= 1.0 { Some(answer_value.get()) } else { parse_answer(&user_input.get()) };
        if let Some(challenge) = current_challenge() {
            if let Some(answer) = user_answer {
                let correct = challenge.answer();
                set_score_results.update(|r| r.push((evaluate(answer, correct), answer, correct)));
                set_submitted.set(true);
            }
        }
    };

    let do_next = move || {
        set_current_index.update(|i| *i += 1);
        set_user_input.set(String::new());
        set_answer_value.set(0.0);
        set_submitted.set(false);
    };

    view! {
        <div class="container">
            <header class="header">
                <h1>"OOM Trainer"</h1>
                <div class="subtitle">"Order of Magnitude Estimation"</div>
            </header>
            <Show when=is_complete fallback=move || view! {
                <ProgressBar current=current_index total_score=Box::new(total_score) />
                <Show when=move || current_challenge().is_some() fallback=|| view! { <div>"Loading..."</div> }>
                    {move || {
                        let challenge = current_challenge().unwrap();
                        view! {
                            <div>
                                <ProblemCard challenge=challenge />
                                <Show
                                    when=move || !submitted.get()
                                    fallback=move || {
                                        score_results.get().last().copied().map_or_else(
                                            || view! { <div></div> }.into_view(),
                                            |(result, user_answer, correct)| view! {
                                                <ResultCard result=result user_answer=user_answer correct=correct
                                                    current_index=current_index on_next=Box::new(do_next) />
                                            }.into_view()
                                        )
                                    }
                                >
                                    <div class="input-section">
                                        <InputModeToggle input_mode=input_mode set_input_mode=set_input_mode />
                                        <Show
                                            when=move || input_mode.get()
                                            fallback=move || view! {
                                                <TextInput
                                                    user_input=user_input
                                                    set_user_input=set_user_input
                                                    set_answer_value=set_answer_value
                                                    on_submit=Box::new(do_submit)
                                                    has_answer=Box::new(has_answer)
                                                />
                                            }
                                        >
                                            <MagnitudeButtons
                                                answer_value=answer_value
                                                set_answer_value=set_answer_value
                                                set_user_input=set_user_input
                                            />
                                        </Show>
                                    </div>
                                    <button class="submit-btn" on:click=move |_| do_submit() prop:disabled=move || !has_answer()>
                                        "Submit"
                                    </button>
                                </Show>
                            </div>
                        }
                    }}
                </Show>
            }>
                <CompleteScreen results=score_results total_score=Box::new(total_score) />
            </Show>
        </div>
    }
}
