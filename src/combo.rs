
#[derive(Clone, Debug, Hash)]
pub enum Combo<T> {
    Nothing,
    Anything,
    Just(T),
    Not(Box<Combo<T>>),
    AnyOf(Vec<Combo<T>>),
}

impl<T> Combo<T> {
    pub fn negate(self) -> Self {
        match self {
            Combo::Nothing => Combo::Anything,
            Combo::Anything => Combo::Nothing,
            x => Combo::Not(Box::new(x)),
        }
    }
    pub fn or(self, other: Self) -> Self {
        match self {
            Combo::Nothing => other,
            Combo::Anything => Combo::Anything,
            Combo::AnyOf(mut any) => {
                any.push(other);
                Combo::AnyOf(any)
            }
            x => Combo::AnyOf(vec![x, other]),
        }
    }

    pub fn contains(&self, x: &T) -> bool where T: Eq {
        match self {
            Combo::Nothing => false,
            Combo::Anything => true,
            Combo::Just(v) => x == v,
            Combo::Not(c) => !c.contains(x),
            Combo::AnyOf(vs) => {
                for c in vs.iter() {
                    if c.contains(x) { return true }
                }
                false
            },
        }
    }

    pub fn try_map<U, E>(self, f: &Fn(T) -> Result<U, E>) -> Result<Combo<U>, E> {
        match self {
            Combo::Nothing => Ok(Combo::Nothing),
            Combo::Anything => Ok(Combo::Anything),
            Combo::Just(t) => Ok(Combo::Just(f(t)?)),
            Combo::Not(t) => Ok(Combo::Not(Box::new(t.try_map(&f)?))),
            Combo::AnyOf(ts) => {
                let mut vs = Vec::with_capacity(ts.len());
                for t in ts.into_iter() {
                    let v = t.try_map(&f)?;
                    vs.push(v);
                }
                Ok(Combo::AnyOf(vs))
            },
        }
    }

}

impl<T: PartialEq> PartialEq for Combo<T> {
    fn eq(&self, other: &Combo<T>) -> bool {
        match (self, other) {
            (&Combo::Nothing, &Combo::Nothing) => true,
            (&Combo::Anything, &Combo::Anything) => true,
            (&Combo::Just(ref a), &Combo::Just(ref b)) => a == b,
            (&Combo::Not(ref a), &Combo::Not(ref b)) => a == b,
            // TODO order should not matter - make this a set
            (&Combo::AnyOf(ref a), &Combo::AnyOf(ref b)) => a == b,
            _ => false,
        }
    }
}

impl<T: PartialEq + Eq> Eq for Combo<T> {
}

