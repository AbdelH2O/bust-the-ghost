use crate::game::*;
use leptos::*;
use leptos_meta::*;
use leptos::html::Button;
use gloo::console::log;

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
            game.bust_ghost(0, 0);
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
            let el = create_node_ref::<Button>();
            // let is_hovered = use_element_hover(el);
            view! {
                <button
                    style=format!("background-color: {}; border: 1px solid black;display: flex; align-items: center; justify-content: center", color)
                    style:border= move || format!("1px solid {}", if clicked.get() == (x, y) {"red"} else {"black"})
                    
                    on:click=move |_| {
                        set_clicked.update(|clicked| *clicked = (x, y));
                        set_game.update(|game| {
                            log!(&format!("Bust: {}, {}", x, y));
                            let color = game.distance_sense(x, y);
                            let new_game = game.update_posterior_ghost_location_probabilities(color, x, y);
                            match new_game {
                                Ok(new_game) => {
                                    log!(&format!("New game: {:?}", new_game));
                                    *game = new_game;
                                }
                                Err(e) => {
                                    log!(&format!("Error: {}", e));
                                }
                            }
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
        <div style="display: flex; flex-direction: column; align-items: center;">

            <div style="display: grid; grid-template-columns: repeat(12, 1fr); grid-template-rows: repeat(9, 1fr); width: 60vw; height: 70vh;margin: auto;">
                {
                    cells
                  }
                  
            </div>
            <button on:click=handle_peep style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: green; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;">
                {move || button_text.get()}
            </button>
            <button on:click=handle_bust style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: red; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;">
                Bust {"("}{move || clicked_cell.get().0}, {move || clicked_cell.get().1}{")"}
            </button>
        </div>
    }
}
