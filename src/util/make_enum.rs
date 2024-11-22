macro_rules! make_enum {
  ($enum_type:tt, $($enum_name:tt),+) => {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum $enum_type {
      $(
        $enum_name,
      )+
    }

    impl $enum_type {
      pub fn to_string(&self) -> &str {
        match self {
          $(
            Self::$enum_name => stringify!($enum_name),
          )+
        }
      }
    }

    impl core::fmt::Display for $enum_type {
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
      }
    }
  };
}

pub(crate) use make_enum;