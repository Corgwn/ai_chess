#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AttackMaps {
    pub black: [u8; 120],
    pub white: [u8; 120],
}
// TODO: Maybe multi-thread map generation and add maps together to speed things up?
