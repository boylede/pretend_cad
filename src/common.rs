use amethyst::input::VirtualKeyCode;

#[derive(Debug, PartialEq)]
pub struct GenerationID<M> {
    id: usize,
    generation: usize,
    _marker: std::marker::PhantomData<M>,
}

impl<M> Copy for GenerationID<M> {}

impl<M> Clone for GenerationID<M> {
    fn clone(&self) -> GenerationID<M> {
        *self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenerationVec<T> {
    pub inner: Vec<(usize, Option<T>)>,
}

impl<T> GenerationVec<T> {
    pub fn new() -> Self {
        GenerationVec { inner: vec![] }
    }
    pub fn get(&self, id: GenerationID<T>) -> Option<&T> {
        let GenerationID { id, generation, .. } = id;
        if let Some((gen, Some(item))) = self.inner.get(id) {
            if *gen == generation {
                return Some(item);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    pub fn push(&mut self, item: T) -> GenerationID<T> {
        if let Some((index, (gen, _))) = self
            .inner
            .iter()
            .enumerate()
            .find(|(_i, (_gen, e))| e.is_none())
        {
            let new_gen = gen + 1;
            self.inner[index] = (new_gen, Some(item));
            GenerationID {
                generation: new_gen,
                id: index,
                _marker: Default::default(),
            }
        } else {
            let i = self.inner.len();
            self.inner.push((0, Some(item)));
            GenerationID {
                id: i,
                generation: 0,
                _marker: Default::default(),
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if let Some((_gen, found)) = self.inner.get_mut(index) {
            found.take()
        } else {
            None
        }
    }
}

pub fn as_alphanumeric(key: VirtualKeyCode) -> Option<char> {
    use VirtualKeyCode::*;
    match key {
        Key1 => Some('1'),
        Key2 => Some('2'),
        Key3 => Some('3'),
        Key4 => Some('4'),
        Key5 => Some('5'),
        Key6 => Some('6'),
        Key7 => Some('7'),
        Key8 => Some('8'),
        Key9 => Some('9'),
        Key0 => Some('0'),
        A => Some('a'),
        B => Some('b'),
        C => Some('c'),
        D => Some('d'),
        E => Some('e'),
        F => Some('f'),
        G => Some('g'),
        H => Some('h'),
        I => Some('i'),
        J => Some('j'),
        K => Some('k'),
        L => Some('l'),
        M => Some('m'),
        N => Some('n'),
        O => Some('o'),
        P => Some('p'),
        Q => Some('q'),
        R => Some('r'),
        S => Some('s'),
        T => Some('t'),
        U => Some('u'),
        V => Some('v'),
        W => Some('w'),
        X => Some('x'),
        Y => Some('y'),
        Z => Some('z'),
        Escape => None,
        F1 => None,
        F2 => None,
        F3 => None,
        F4 => None,
        F5 => None,
        F6 => None,
        F7 => None,
        F8 => None,
        F9 => None,
        F10 => None,
        F11 => None,
        F12 => None,
        F13 => None,
        F14 => None,
        F15 => None,
        F16 => None,
        F17 => None,
        F18 => None,
        F19 => None,
        F20 => None,
        F21 => None,
        F22 => None,
        F23 => None,
        F24 => None,
        Snapshot => None,
        Scroll => None,
        Pause => None,
        Insert => None,
        Home => None,
        Delete => None,
        End => None,
        PageDown => None,
        PageUp => None,
        Left => None,
        Up => None,
        Right => None,
        Down => None,
        Back => None,
        Return => None,
        Space => None,
        Compose => None,
        Caret => None,
        Numlock => None,
        Numpad0 => Some('0'),
        Numpad1 => Some('1'),
        Numpad2 => Some('2'),
        Numpad3 => Some('3'),
        Numpad4 => Some('4'),
        Numpad5 => Some('5'),
        Numpad6 => Some('6'),
        Numpad7 => Some('7'),
        Numpad8 => Some('8'),
        Numpad9 => Some('9'),
        AbntC1 => None,
        AbntC2 => None,
        Add => None,
        Apostrophe => None,
        Apps => None,
        At => None,
        Ax => None,
        Backslash => None,
        Calculator => None,
        Capital => None,
        Colon => None,
        Comma => None,
        Convert => None,
        Decimal => None,
        Divide => None,
        Equals => None,
        Grave => None,
        Kana => None,
        Kanji => None,
        LAlt => None,
        LBracket => None,
        LControl => None,
        LShift => None,
        LWin => None,
        Mail => None,
        MediaSelect => None,
        MediaStop => None,
        Minus => None,
        Multiply => None,
        Mute => None,
        MyComputer => None,
        NavigateForward => None,
        NavigateBackward => None,
        NextTrack => None,
        NoConvert => None,
        NumpadComma => None,
        NumpadEnter => None,
        NumpadEquals => None,
        OEM102 => None,
        Period => None,
        PlayPause => None,
        Power => None,
        PrevTrack => None,
        RAlt => None,
        RBracket => None,
        RControl => None,
        RShift => None,
        RWin => None,
        Semicolon => None,
        Slash => None,
        Sleep => None,
        Stop => None,
        Subtract => None,
        Sysrq => None,
        Tab => None,
        Underline => None,
        Unlabeled => None,
        VolumeDown => None,
        VolumeUp => None,
        Wake => None,
        WebBack => None,
        WebFavorites => None,
        WebForward => None,
        WebHome => None,
        WebRefresh => None,
        WebSearch => None,
        WebStop => None,
        Yen => None,
        Copy => None,
        Paste => None,
        Cut => None,
    }
}

pub fn is_confirmation(key: VirtualKeyCode) -> Option<bool> {
    use VirtualKeyCode::*;
    match key {
        Escape => Some(false),
        Return => Some(true),
        Space => Some(true),
        NumpadEnter => Some(true),
        Tab => Some(true),
        _ => None,
    }
}