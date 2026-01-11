use leptos::*;
use crate::challenge::{format_number, generate_challenges, get_daily_seed};
use crate::parser::parse_answer;
use crate::scoring::{evaluate, format_oom_difference, ScoreResult};

const PROBLEMS_PER_DAY: usize = 5;

#[component]
pub fn App() -> impl IntoView {
    let seed = get_daily_seed();
    let challenges = store_value(generate_challenges(seed, PROBLEMS_PER_DAY));

    let (current_index, set_current_index) = create_signal(0usize);
    let (user_input, set_user_input) = create_signal(String::new());
    let (submitted, set_submitted) = create_signal(false);
    let (score_results, set_score_results) = create_signal(Vec::<(ScoreResult, f64, f64)>::new());

    let current_challenge = move || {
        challenges.with_value(|c| c.get(current_index.get()).cloned())
    };

    let total_score = move || {
        score_results.get().iter().map(|(r, _, _)| r.points()).sum::<u32>()
    };

    let is_complete = move || current_index.get() >= PROBLEMS_PER_DAY;

    let do_submit = move || {
        if submitted.get() {
            return;
        }
        if let Some(challenge) = current_challenge() {
            if let Some(user_answer) = parse_answer(&user_input.get()) {
                let correct = challenge.answer();
                let result = evaluate(user_answer, correct);
                set_score_results.update(|results| {
                    results.push((result, user_answer, correct));
                });
                set_submitted.set(true);
            }
        }
    };

    let do_next = move || {
        set_current_index.update(|i| *i += 1);
        set_user_input.set(String::new());
        set_submitted.set(false);
    };

    view! {
        <div class="container">
            <h1>"OOM Trainer"</h1>

            <Show
                when=is_complete
                fallback=move || {
                    view! {
                        <Show
                            when=move || current_challenge().is_some()
                            fallback=|| view! { <div>"Loading..."</div> }
                        >
                            {move || {
                                let challenge = current_challenge().unwrap();
                                let operator = if challenge.is_division { "÷" } else { "×" };
                                view! {
                                    <div>
                                        <div class="problem">
                                            {format_number(challenge.num1)}
                                            <span class="operator">{operator}</span>
                                            {format_number(challenge.num2)}
                                            <span class="operator">"="</span>
                                            "?"
                                        </div>

                                        <div class="input-group">
                                            <input
                                                type="text"
                                                placeholder="e.g. 400 billion"
                                                prop:value=move || user_input.get()
                                                on:input=move |ev| set_user_input.set(event_target_value(&ev))
                                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                    if ev.key() == "Enter" {
                                                        if submitted.get() {
                                                            do_next();
                                                        } else {
                                                            do_submit();
                                                        }
                                                    }
                                                }
                                                prop:disabled=move || submitted.get()
                                            />
                                            <button
                                                on:click=move |_| do_submit()
                                                prop:disabled=move || submitted.get() || user_input.get().is_empty()
                                            >
                                                "Submit"
                                            </button>
                                        </div>

                                        <Show
                                            when=move || submitted.get()
                                            fallback=|| view! { <div></div> }
                                        >
                                            {move || {
                                                if let Some((result, user_answer, correct)) = score_results.get().last().cloned() {
                                                    let result_class = match result {
                                                        ScoreResult::Exact | ScoreResult::Close => "result correct",
                                                        ScoreResult::Partial => "result close",
                                                        ScoreResult::Wrong => "result wrong",
                                                    };
                                                    view! {
                                                        <div>
                                                            <div class=result_class>
                                                                <div class="label">{result.label()}</div>
                                                                <div class="value">
                                                                    {format!("Your answer: {}", format_number(user_answer))}
                                                                </div>
                                                                <div class="value">
                                                                    {format!("Correct: {}", format_number(correct))}
                                                                </div>
                                                                <div style="margin-top: 0.5rem; color: #aaa;">
                                                                    {format_oom_difference(user_answer, correct)}
                                                                </div>
                                                            </div>
                                                            <button class="next-btn" on:click=move |_| do_next()>
                                                                {move || if current_index.get() + 1 >= PROBLEMS_PER_DAY { "See Results" } else { "Next Problem" }}
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <div></div> }.into_view()
                                                }
                                            }}
                                        </Show>

                                        <div class="stats">
                                            <div>
                                                <div class="stat-value">{move || format!("{} / {}", current_index.get() + 1, PROBLEMS_PER_DAY)}</div>
                                                <div>"Problem"</div>
                                            </div>
                                            <div>
                                                <div class="stat-value">{total_score}</div>
                                                <div>"Score"</div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }}
                        </Show>
                    }
                }
            >
                <div class="complete">
                    <div class="result correct">
                        <div class="label">"Session Complete!"</div>
                        <div class="value">{move || format!("Score: {} / {}", total_score(), PROBLEMS_PER_DAY * 100)}</div>
                    </div>
                    <p style="color: #888; margin-top: 1rem;">"Come back tomorrow for new problems!"</p>
                </div>
            </Show>
        </div>
    }
}
