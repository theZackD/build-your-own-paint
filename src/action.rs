#[derive(Clone, Copy)]
pub enum Action {
    Rien,
    Dessin,
    Deplacement(usize),
    Teinte,
    Sv,
}
