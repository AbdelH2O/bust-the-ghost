use crate::game::*;
use gloo::console::log;
use leptos::*;
use leptos_meta::*;
// use lepto p

#[component]
pub fn GameView() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let game = Game::new(9, 12);
    // create a signal to update the game state
    let (gm, set_game) = create_signal(game);
    let (peeping, set_peeping) = create_signal(true);
    let (button_text, set_button_text) = create_signal("Hide");
    let (clicked_cell, set_clicked_cell) = create_signal((0, 0));
    let (state, set_state) = create_signal(-1);
    let (direction_hint, set_direction_hint) = create_signal("".to_string());
    // compute initial game state
    set_game.update(|game| {
        game.place_ghost();
        game.compute_initial_prior_probabilities();
    });

    let handle_peep = move |_| {
        set_peeping.update(|peeping| *peeping = !*peeping);
        set_button_text.update(|text| {
            if *text == "Peep" {
                *text = "Hide";
            } else {
                *text = "Peep";
            }
        });
    };

    let handle_bust = move |_| {
        set_game.update(|game| {
            let result = game.bust_ghost(clicked_cell.get().0, clicked_cell.get().1);
            if result == 0 {
                set_state.update(|state| *state = 0);
                log!("Out of busts! You lose!");
            } else if result == 1 {
                set_state.update(|state| *state = 1);
                log!("You win!");
            } else {
                set_state.update(|state| *state = -2);
                log!("Missed!");
            }
        });
    };

    let cells = move || {
        gm.get().grid.iter().flat_map(|row| row.iter()).map(|cell| {
            let color = cell.color.clone();
            let probability = cell.probability;
            let x = cell.x;
            let y = cell.y;
            let clicked = clicked_cell.clone();
            let set_clicked = set_clicked_cell.clone();
            view! {
                <button
                    style=format!("background-color: {}; border: 1px solid black;display: flex; align-items: center; justify-content: center; cursor: pointer", color)
                    style:border= move || format!("1px solid {}", if clicked.get() == (x, y) {"red"} else {"black"})
                    on:click=move |_| {
                        set_clicked.update(|clicked| *clicked = (x, y));
                        set_game.update(|game| {
                            log!(&format!("Clicked: {}, {}", x, y));
                            let (color, direction) = game.distance_sense(x, y);
                            set_direction_hint.update(|hint| *hint = direction.clone());
                            log!(&format!("Color: {}, Direction: {}", color, direction.clone()));
                            if game.grid[x as usize][y as usize].visited {
                                return;
                            }
                            game.grid[x as usize][y as usize].visited = true;
                            log!(&format!("Bust: {}, {}", x, y));
                            if game.score == 0 {
                                set_state.update(|state| *state = 0);
                                log!("Out of attempts! You lose!");
                            }
                            game.grid[x as usize][y as usize].color = color.clone();
                            game.update_posterior_ghost_location_probabilities(color, x, y, direction);
                        });
                    }
                >
                    {if peeping.get() {
                        format!("{:.2}%", probability*100.0)
                    } else {
                        "".to_string()
                    }}
                </button>
            }
        }).collect::<Vec<_>>()
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet

        // sets the document title
        <div style="height: 100vh; display: flex; flex-direction: column; align-items: center; justify-content: center;">
            {
                move || match state.get() {
                    0 | 1 => {
                        view! {
                            <div style="position: absolute; background-color: rgba(0, 0, 0, 0.5); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                                <div style="background-color: white; padding: 20px; border-radius: 4px; display: flex; flex-direction: column; align-items: center;">
                                    <h1 style="text-align: center;">{if state.get() == 0 {"You lose!"} else {"You win!"}}</h1>
                                    <button on:click=move |_| {
                                        set_state.update(|state| *state = -1);
                                        set_game.update(|game| {
                                            game.reset();
                                            game.place_ghost();
                                            game.compute_initial_prior_probabilities();
                                        });
                                    } style="padding: 10px;cursor: pointer; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: green; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;">
                                        Play again
                                    </button>
                                </div>
                            </div>
                        }
                    },
                    _ => view! {<div></div>},
                }
            }
            <div style="display: flex; flex-direction: column; align-items: center;height: 80%; width: 80%">
                <div style="margin-bottom: 20px; display: flex; flex-direction: column; align-items: center; gap: 10px;">
                    <h1 style="text-align: center;">Bust The Ghost</h1>
                    <p style="text-align: center;">Click on a cell to bust the ghost. The color of the cell will give you a clue about the ghosts location.</p>
                    <p style="text-align: center;">Score: {move || gm.get().score} attempte left</p>
                    <p style="text-align: center;">Busts: {move || gm.get().busts} left</p>
                    {move || match state.get() {
                        -2 => {
                            view! {
                                <p style="text-align: center; color: red;">Missed! Try again!</p>
                            }
                        },
                        _ => {
                            view! {
                                <p style="text-align: center; color: white; user-select: none">{"a"}</p>
                            }
                        }
                    }}
                    {move || if direction_hint.get().len() > 0 {
                        view! {
                            <p style="text-align: center; color: #1d4ed8;font-size: 20px">{move || direction_hint.get()}{" "}{move || match direction_hint.get().as_str(){
                                "N" => "⬆️",
                                "S" => "⬇️",
                                "E" => "➡️",
                                "W" => "⬅️",
                                "NE" => "↗️",
                                "NW" => "↖️",
                                "SE" => "↘️",
                                "SW" => "↙️",
                                _ => "😱"
                            }}</p>
                        }
                    } else {
                        view! {
                            <p style="text-align: center; color: white; user-select: none;font-size: 20px">{"a"}</p>
                        }
                    }}
                </div>
                <div style="display: grid; grid-template-columns: repeat(12, 1fr); grid-template-rows: repeat(9, 1fr); width: 100%; height: 100%;margin: auto;">
                    {cells}
                </div>
                <button on:click=handle_peep style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: green; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;cursor:pointer">
                    {move || button_text.get()}
                </button>
                <button on:click=handle_bust style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: red; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;cursor: pointer">
                    Bust {"("}{move || clicked_cell.get().0}, {move || clicked_cell.get().1}{")"}
                </button>
            </div>
        </div>
    }
}
