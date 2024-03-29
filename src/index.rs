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
    let (peeping, set_peeping) = create_signal(false);
    // compute initial game state
    set_game.update(|game| {
        game.place_ghost();
        game.compute_initial_prior_probabilities();
    });

    let handle_peep = move |_| {
        set_peeping(!peeping.get());
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet

        // sets the document title
        <div>

            <div style="display: grid; grid-template-columns: repeat(12, 1fr); grid-template-rows: repeat(9, 1fr); width: 70vw; height: 70vh;margin: auto;">
                {
                    gm.get().grid.iter().flat_map(|row| row.iter().map(|cell| {
                        view! {
                            <div style=format!("background-color: {}; border: 1px solid black;display: flex; align-items: center; justify-content: center", cell.color)>
                                {if peeping.get() {
                                    format!("{:.2}%", cell.probability*100.0)
                                } else {
                                    "".to_string()
                                }}    
                            </div>
                        }
                    })).collect::<Vec<_>>()
                  }
            </div>
            // <button on_click=handle_peep>
                // {if peeping.get() {
                    // "Hide"
                // } else {
                    // "Peep"
                // }}
            // </button>
        </div>
    }
}
