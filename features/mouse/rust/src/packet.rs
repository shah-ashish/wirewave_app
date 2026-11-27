//! Decoding for the Android mouse protocol, independent of networking and OS input.

pub const EVENT_MOVE_REL: u8 = 0x01;
pub const EVENT_BUTTON_DOWN: u8 = 0x02;
pub const EVENT_BUTTON_UP: u8 = 0x03;
pub const EVENT_SCROLL: u8 = 0x04;
pub const EVENT_CLICK: u8 = 0x05;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MouseEvent {
    Move { dx: i16, dy: i16 },
    ButtonDown { button: u8 },
    ButtonUp { button: u8 },
    Scroll { delta: i16 },
    Click { button: u8 },
}

impl MouseEvent {
    /// Accepts the existing five-byte form (left button) and ignores trailing bytes.
    /// Short packets and unknown event identifiers are ignored.
    pub(crate) fn decode(payload: &[u8]) -> Option<Self> {
        if payload.len() < 5 {
            return None;
        }
        let first_delta = i16::from_be_bytes([payload[1], payload[2]]);
        let second_delta = i16::from_be_bytes([payload[3], payload[4]]);
        let button = payload.get(5).copied().unwrap_or(1);

        match payload[0] {
            EVENT_MOVE_REL => Some(Self::Move {
                dx: first_delta,
                dy: second_delta,
            }),
            EVENT_BUTTON_DOWN => Some(Self::ButtonDown { button }),
            EVENT_BUTTON_UP => Some(Self::ButtonUp { button }),
            EVENT_SCROLL => Some(Self::Scroll { delta: first_delta }),
            EVENT_CLICK => Some(Self::Click { button }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MouseEvent;

    #[test]
    fn decodes_android_movement_bytes() {
        assert_eq!(
            MouseEvent::decode(&[1, 0, 3, 255, 252, 0]),
            Some(MouseEvent::Move { dx: 3, dy: -4 })
        );
    }

    #[test]
    fn preserves_signed_delta_limits() {
        assert_eq!(
            MouseEvent::decode(&[1, 127, 255, 128, 0, 0]),
            Some(MouseEvent::Move { dx: i16::MAX, dy: i16::MIN })
        );
    }

    #[test]
    fn decodes_both_scroll_directions() {
        assert_eq!(
            MouseEvent::decode(&[4, 0, 6, 0, 0, 0]),
            Some(MouseEvent::Scroll { delta: 6 })
        );
        assert_eq!(
            MouseEvent::decode(&[4, 255, 250, 0, 0, 0]),
            Some(MouseEvent::Scroll { delta: -6 })
        );
    }

    #[test]
    fn decodes_button_actions_without_changing_identifiers() {
        for button in [0, 1, 2, 3, 255] {
            assert_eq!(
                MouseEvent::decode(&[2, 0, 0, 0, 0, button]),
                Some(MouseEvent::ButtonDown { button })
            );
            assert_eq!(
                MouseEvent::decode(&[3, 0, 0, 0, 0, button]),
                Some(MouseEvent::ButtonUp { button })
            );
            assert_eq!(
                MouseEvent::decode(&[5, 0, 0, 0, 0, button]),
                Some(MouseEvent::Click { button })
            );
        }
    }

    #[test]
    fn five_byte_packets_default_to_left_button() {
        assert_eq!(
            MouseEvent::decode(&[5, 0, 0, 0, 0]),
            Some(MouseEvent::Click { button: 1 })
        );
    }

    #[test]
    fn ignores_short_and_unknown_packets() {
        let packet = [1, 0, 0, 0, 0];
        for length in 0..5 {
            assert_eq!(MouseEvent::decode(&packet[..length]), None);
        }
        assert_eq!(MouseEvent::decode(&[255, 0, 0, 0, 0, 1]), None);
    }

    #[test]
    fn preserves_tolerance_for_trailing_bytes() {
        assert_eq!(
            MouseEvent::decode(&[5, 0, 0, 0, 0, 2, 99, 100]),
            Some(MouseEvent::Click { button: 2 })
        );
    }
}
