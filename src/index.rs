use crate::game::*;
use leptos::*;
use leptos_meta::*;

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

    

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet

        // sets the document title
        <div style="display: flex; flex-direction: column; align-items: center;">

            <div style="display: grid; grid-template-columns: repeat(12, 1fr); grid-template-rows: repeat(9, 1fr); width: 60vw; height: 70vh;margin: auto;">
                {
                    move || gm.get().grid.iter().flat_map(|row| row.iter().map(|cell| {
                        view! {
                            <button on:click={move |_| {
                                set_clicked_cell.update(|clicked_cell| {
                                    let cloned_cell = cell.clone();
                                    *clicked_cell = (cloned_cell.x, cloned_cell.y); 
                                });
                            }} style=format!("background-color: {}; border: 1px solid black;display: flex; align-items: center; justify-content: center", cell.color)>
                                {if peeping.get() {
                                    format!("{:.2}%", cell.probability*100.0)
                                } else {
                                    "".to_string()
                                }}
                            </button>
                        }
                    })).collect::<Vec<_>>()
                  }
            </div>
            <button on:click=handle_peep style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: green; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;">
                {move || button_text.get()}
            </button>
            <button on:click=handle_bust style="padding: 10px; padding-left: 20px; padding-right: 20px; margin-top: 20px; background-color: red; color: white; border-radius: 4px; border: none; width: 200px; font-size: 20px;">
                Bust
            </button>
        </div>
    }
}
