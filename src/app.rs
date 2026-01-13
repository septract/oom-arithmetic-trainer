use leptos::*;
use crate::challenge::{format_number, generate_challenges, get_daily_seed};
use crate::parser::parse_answer;
use crate::scoring::{evaluate, ScoreResult};

const PROBLEMS_PER_DAY: usize = 5;

// Magnitude labels for display
const MAGNITUDES: &[(f64, &str)] = &[
    (1e3, "K"),
    (1e6, "M"),
    (1e9, "B"),
    (1e12, "T"),
];

fn format_answer_display(value: f64) -> String {
    if value < 1.0 {
        return "0".to_string();
    }

    // Find appropriate magnitude
    for &(mag, suffix) in MAGNITUDES.iter().rev() {
        if value >= mag {
            let scaled = value / mag;
            if scaled >= 100.0 {
                return format!("{:.0}{}", scaled, suffix);
            } else if scaled >= 10.0 {
                return format!("{:.1}{}", scaled, suffix);
            } else {
                return format!("{:.2}{}", scaled, suffix);
            }
        }
    }

    format!("{:.0}", value)
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

#[component]
pub fn App() -> impl IntoView {
    let seed = get_daily_seed();
    let challenges = store_value(generate_challenges(seed, PROBLEMS_PER_DAY));

    let (current_index, set_current_index) = create_signal(0usize);
    let (user_input, set_user_input) = create_signal(String::new());
    let (answer_value, set_answer_value) = create_signal(0.0f64);
    let (submitted, set_submitted) = create_signal(false);
    let (score_results, set_score_results) = create_signal(Vec::<(ScoreResult, f64, f64)>::new());
    let (input_mode, set_input_mode) = create_signal(true); // true = buttons, false = text

    let current_challenge = move || {
        challenges.with_value(|c| c.get(current_index.get()).cloned())
    };

    let total_score = move || {
        score_results.get().iter().map(|(r, _, _)| r.points()).sum::<u32>()
    };

    let is_complete = move || current_index.get() >= PROBLEMS_PER_DAY;

    let adjust_magnitude = move |multiplier: f64| {
        set_answer_value.update(|v| {
            let new_val = if *v < 1.0 { 1000.0 } else { *v * multiplier };
            *v = new_val.max(0.0).min(1e15);
        });
        // Sync to text input
        let val = answer_value.get();
        if val >= 1.0 {
            set_user_input.set(format_answer_display(val));
        }
    };

    let do_submit = move || {
        if submitted.get() {
            return;
        }

        // Try button value first, then parse text input
        let user_answer = if answer_value.get() >= 1.0 {
            Some(answer_value.get())
        } else {
            parse_answer(&user_input.get())
        };

        if let Some(challenge) = current_challenge() {
            if let Some(answer) = user_answer {
                let correct = challenge.answer();
                let result = evaluate(answer, correct);
                set_score_results.update(|results| {
                    results.push((result, answer, correct));
                });
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

    let has_answer = move || answer_value.get() >= 1.0 || !user_input.get().is_empty();

    view! {
        <div class="container">
            // Header
            <header class="header">
                <h1>"OOM Trainer"</h1>
                <div class="subtitle">"Order of Magnitude Estimation"</div>
            </header>

            <Show
                when=is_complete
                fallback=move || {
                    view! {
                        // Progress bar
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div
                                    class="progress-fill"
                                    style:width=move || format!("{}%", (current_index.get() as f64 / PROBLEMS_PER_DAY as f64) * 100.0)
                                ></div>
                            </div>
                            <div class="progress-text">
                                <span>{move || format!("Problem {} of {}", current_index.get() + 1, PROBLEMS_PER_DAY)}</span>
                                <span>{move || format!("Score: {}", total_score())}</span>
                            </div>
                        </div>

                        <Show
                            when=move || current_challenge().is_some()
                            fallback=|| view! { <div>"Loading..."</div> }
                        >
                            {move || {
                                let challenge = current_challenge().unwrap();
                                let operator = if challenge.is_division { "/" } else { "x" };

                                view! {
                                    <div>
                                        // Problem card
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

                                        <Show
                                            when=move || !submitted.get()
                                            fallback=move || {
                                                // Show result
                                                if let Some((result, user_answer, correct)) = score_results.get().last().cloned() {
                                                    let result_class = match result {
                                                        ScoreResult::Exact | ScoreResult::Close => "result-card correct",
                                                        ScoreResult::Partial => "result-card close",
                                                        ScoreResult::Wrong => "result-card wrong",
                                                    };
                                                    let (direction_text, direction_class) = get_direction_indicator(user_answer, correct);

                                                    view! {
                                                        <div>
                                                            <div class=result_class>
                                                                <div class="result-label">{result.label()}</div>
                                                                <div class="result-details">
                                                                    <div>
                                                                        "You: "
                                                                        <span class="your-answer">{format_number(user_answer)}</span>
                                                                        {(!direction_text.is_empty()).then(|| view! {
                                                                            <span class=format!("direction {}", direction_class)>{direction_text}</span>
                                                                        })}
                                                                    </div>
                                                                    <div>
                                                                        "Answer: "
                                                                        <span class="correct-answer">{format_number(correct)}</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <button class="next-btn" on:click=move |_| do_next()>
                                                                {move || if current_index.get() + 1 >= PROBLEMS_PER_DAY {
                                                                    "See Results"
                                                                } else {
                                                                    "Next Problem"
                                                                }}
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <div></div> }.into_view()
                                                }
                                            }
                                        >
                                            // Input section
                                            <div class="input-section">
                                                // Toggle between input modes
                                                <div style="display: flex; justify-content: center; gap: 0.5rem; margin-bottom: 1rem;">
                                                    <button
                                                        class="mag-btn"
                                                        style:background=move || if input_mode.get() { "var(--accent)" } else { "var(--bg-card)" }
                                                        style:color=move || if input_mode.get() { "#fff" } else { "var(--text-primary)" }
                                                        on:click=move |_| set_input_mode.set(true)
                                                    >
                                                        "Buttons"
                                                    </button>
                                                    <button
                                                        class="mag-btn"
                                                        style:background=move || if !input_mode.get() { "var(--accent)" } else { "var(--bg-card)" }
                                                        style:color=move || if !input_mode.get() { "#fff" } else { "var(--text-primary)" }
                                                        on:click=move |_| set_input_mode.set(false)
                                                    >
                                                        "Type"
                                                    </button>
                                                </div>

                                                <Show
                                                    when=move || input_mode.get()
                                                    fallback=move || {
                                                        // Text input mode
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
                                                                        if ev.key() == "Enter" && has_answer() {
                                                                            do_submit();
                                                                        }
                                                                    }
                                                                />
                                                                <div class="input-hint">"Formats: 400B, 400 billion, 4e11, 4x10^11"</div>
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    // Button input mode
                                                    <div>
                                                        // Answer display
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

                                                        // Magnitude controls
                                                        <div class="magnitude-controls">
                                                            <div class="magnitude-row">
                                                                <button class="mag-btn increase" on:click=move |_| adjust_magnitude(1000.0)>"+1000x"</button>
                                                                <button class="mag-btn increase" on:click=move |_| adjust_magnitude(100.0)>"+100x"</button>
                                                                <button class="mag-btn increase" on:click=move |_| adjust_magnitude(10.0)>"+10x"</button>
                                                                <button class="mag-btn increase" on:click=move |_| adjust_magnitude(2.0)>"+2x"</button>
                                                            </div>
                                                            <div class="magnitude-row">
                                                                <button class="mag-btn decrease" on:click=move |_| adjust_magnitude(0.001)>"/1000"</button>
                                                                <button class="mag-btn decrease" on:click=move |_| adjust_magnitude(0.01)>"/100"</button>
                                                                <button class="mag-btn decrease" on:click=move |_| adjust_magnitude(0.1)>"/10"</button>
                                                                <button class="mag-btn decrease" on:click=move |_| adjust_magnitude(0.5)>"/2"</button>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </Show>
                                            </div>

                                            // Submit button
                                            <button
                                                class="submit-btn"
                                                on:click=move |_| do_submit()
                                                prop:disabled=move || !has_answer()
                                            >
                                                "Submit"
                                            </button>
                                        </Show>
                                    </div>
                                }
                            }}
                        </Show>
                    }
                }
            >
                // Complete screen
                {move || {
                    let results = score_results.get();
                    let exact_count = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Exact | ScoreResult::Close)).count();
                    let partial_count = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Partial)).count();
                    let wrong_count = results.iter().filter(|(r, _, _)| matches!(r, ScoreResult::Wrong)).count();

                    view! {
                        <div class="complete-screen">
                            <div class="complete-title">"Session Complete"</div>
                            <div class="complete-score">{total_score()}</div>
                            <div class="complete-subtitle">{format!("out of {} points", PROBLEMS_PER_DAY * 100)}</div>

                            <div class="score-breakdown">
                                <div class="breakdown-row">
                                    <span class="breakdown-label">"Correct (within 1 OOM)"</span>
                                    <span class="breakdown-value correct">{exact_count}</span>
                                </div>
                                <div class="breakdown-row">
                                    <span class="breakdown-label">"Close (within 2 OOM)"</span>
                                    <span class="breakdown-value close">{partial_count}</span>
                                </div>
                                <div class="breakdown-row">
                                    <span class="breakdown-label">"Off (3+ OOM)"</span>
                                    <span class="breakdown-value wrong">{wrong_count}</span>
                                </div>
                            </div>

                            <div class="come-back">"New problems tomorrow!"</div>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
