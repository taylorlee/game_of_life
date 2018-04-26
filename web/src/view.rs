
use model::{Model, Msg};
use game::{Board, board_slice};
use yew::html::Html;

const LEN: usize = 50;
const DIM: usize = LEN * 2;

type Row = [bool; DIM];
type Grid = [Row; DIM];

fn new_grid() -> Grid {
    [[false; DIM]; DIM]
}

pub fn render(model: &Model) -> Html<Msg> {
    let rows = board2grid(&model.board);
    html! {
        <div>
            <section class="section",>
                <div class="container",>
                    <div class="level",>
                        <div class="level-item",>
                            <h1 class="title",>{ "Game of Life" }</h1>
                        </div>
                    </div>
                </div>
                <div class="level",></div>
                <div class="container",>
                    <div class="level",>
                        <div class="level-item",>
                            <div class=("tags","has-addons"),>
                              <span class="tag",> {"Generation #"}</span>
                              <span class=("tag","is-primary"),> { model.clock } </span>
                            </div>
                        </div>
                    </div>
                    <div class="level",>
                        { if model.running { show_pause() } else { show_start() } }
                    </div>
                    <div class="level",>
                        <div class="level-item",>
                            <div class=("tags","has-addons"),>
                              <span class="tag",> {"SPEED"} </span>
                              <span class=("tag","is-primary"),> { model.speed} </span>
                            </div>
                        </div>
                    </div>
                    <div class="level",>
                        <div class="level-item",>
                            <button class="button", onclick=|_| Msg::Decr,>{ "Slow Down" }</button>
                            <button class="button",  onclick=|_| Msg::Incr,>{ "Speed Up" }</button>
                        </div>
                    </div>
                </div>
                <div class="level",></div>
                <div class="container",>
                    <div class="level",>
                        <div class="level-item",>
                            <table class="grid",>
                                { for rows.iter().map(view_row)  }
                            </table>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn board2grid(board: &Board) -> Grid {
    let mut rows = new_grid();
    for rdx in 0..DIM {
        for cdx in 0..DIM {
            let x = rdx as isize - LEN as isize;
            let y = cdx as isize - LEN as isize;
            rows[rdx][cdx] = board_slice(&board, x, y);
        }
    }
    return rows;
}


fn show_start() -> Html<Msg> {
    html! {
        <div class="level-item",>
            <button class="button", onclick=move|_| Msg::Step,>{ "Step" }</button>
            <button class="button", onclick=move|_| Msg::Start,>{ "Start" }</button>
            <button class="button", onclick=move|_| Msg::Reset,>{ "Reset" }</button>
        </div>
    }
}

fn show_pause() -> Html<Msg> {
    html! {
        <div class="level-item",>
            <button class="button", onclick=move|_| Msg::Stop,>{ "Pause" }</button>
        </div>
    }
}

fn view_row(cells: &Row) -> Html<Msg> {
    html! {
        <tr class="row",>
            { for cells.iter().map(view_cell) }
        </tr>
    }
}

fn view_cell(living: &bool) -> Html<Msg> {
    html! {
        <td class=("cell", if *living { "alive" } else { "dead" } ),</td>
    }
}
