use {
    anyhow::anyhow,
    rocket::http::{
        impl_from_uri_param_identity,
        uri::{
            self,
            fmt::{
                Path,
                UriDisplay,
            },
        },
    },
    sqlx::{
        Decode,
        Encode,
        postgres::{
            PgArgumentBuffer,
            PgTypeInfo,
            PgValueRef,
        },
    },
    crate::prelude::*,
};

pub(crate) mod br;
pub(crate) mod coop;
pub(crate) mod fr;
pub(crate) mod latam;
pub(crate) mod league;
pub(crate) mod mp;
pub(crate) mod mq;
pub(crate) mod mw;
pub(crate) mod ndos;
pub(crate) mod ohko;
pub(crate) mod pic;
pub(crate) mod pot;
pub(crate) mod rsl;
pub(crate) mod s;
pub(crate) mod scrubs;
pub(crate) mod sgl;
pub(crate) mod soh;
pub(crate) mod tfb;
pub(crate) mod wttbb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
pub(crate) enum Series {
    BattleRoyale,
    CoOp,
    CopaDoBrasil,
    CopaLatinoamerica,
    League,
    MixedPools,
    Mq,
    Multiworld,
    NineDaysOfSaws,
    Pictionary,
    PotsOfTime,
    Rsl,
    Scrubs,
    SongsOfHope,
    SpeedGaming,
    Standard,
    TournoiFrancophone,
    TriforceBlitz,
    WeTryToBeBetter,
}

impl Series {
    pub(crate) fn slug(&self) -> &'static str {
        match self {
            Self::BattleRoyale => "ohko",
            Self::CoOp => "coop",
            Self::CopaDoBrasil => "br",
            Self::CopaLatinoamerica => "latam",
            Self::League => "league",
            Self::MixedPools => "mp",
            Self::Mq => "mq",
            Self::Multiworld => "mw",
            Self::NineDaysOfSaws => "9dos",
            Self::Pictionary => "pic",
            Self::PotsOfTime => "pot",
            Self::Rsl => "rsl",
            Self::Scrubs => "scrubs",
            Self::SongsOfHope => "soh",
            Self::SpeedGaming => "sgl",
            Self::Standard => "s",
            Self::TournoiFrancophone => "fr",
            Self::TriforceBlitz => "tfb",
            Self::WeTryToBeBetter => "wttbb",
        }
    }

    pub(crate) fn display_name(&self) -> &'static str {
        match self {
            Self::BattleRoyale => "Battle Royale",
            Self::CoOp => "Co-op Tournaments",
            Self::CopaDoBrasil => "Copa do Brasil",
            Self::CopaLatinoamerica => "Copa Latinoamerica",
            Self::League => "League",
            Self::MixedPools => "Mixed Pools Tournaments",
            Self::Mq => "12 MQ Tournaments",
            Self::Multiworld => "Multiworld Tournaments",
            Self::NineDaysOfSaws => "9 Days of SAWS",
            Self::Pictionary => "Pictionary Spoiler Log Races",
            Self::PotsOfTime => "Pots Of Time",
            Self::Rsl => "Random Settings League",
            Self::Scrubs => "Scrubs Tournaments",
            Self::SongsOfHope => "Songs of Hope",
            Self::SpeedGaming => "SpeedGaming Live",
            Self::Standard => "Standard Tournaments",
            Self::TournoiFrancophone => "Tournois Francophones",
            Self::TriforceBlitz => "Triforce Blitz",
            Self::WeTryToBeBetter => "WeTryToBeBetter",
        }
    }

    pub(crate) fn default_race_duration(&self) -> TimeDelta {
        match self {
            Self::TriforceBlitz => TimeDelta::hours(2),
            Self::BattleRoyale => TimeDelta::hours(2) + TimeDelta::minutes(30),
            Self::CoOp | Self::MixedPools | Self::Scrubs | Self::SpeedGaming | Self::WeTryToBeBetter => TimeDelta::hours(3),
            Self::CopaDoBrasil | Self::CopaLatinoamerica | Self::League | Self::NineDaysOfSaws | Self::SongsOfHope | Self::Standard | Self::TournoiFrancophone => TimeDelta::hours(3) + TimeDelta::minutes(30),
            Self::Mq | Self::Multiworld | Self::Pictionary => TimeDelta::hours(4),
            Self::PotsOfTime | Self::Rsl => TimeDelta::hours(4) + TimeDelta::minutes(30),
        }
    }
}

impl FromStr for Series {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        all::<Self>().find(|series| series.slug() == s).ok_or(())
    }
}

impl<'r> Decode<'r, Postgres> for Series {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let series = <&str as Decode<Postgres>>::decode(value)?;
        series.parse().map_err(|()| anyhow!("unknown series: {series}").into())
    }
}

impl<'q> Encode<'q, Postgres> for Series {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        Encode::<Postgres>::encode_by_ref(&self.slug(), buf)
    }

    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        Encode::<Postgres>::encode(self.slug(), buf)
    }

    fn produces(&self) -> Option<PgTypeInfo> {
        Encode::<Postgres>::produces(&self.slug())
    }

    fn size_hint(&self) -> usize {
        Encode::<Postgres>::size_hint(&self.slug())
    }
}

impl sqlx::Type<Postgres> for Series {
    fn type_info() -> PgTypeInfo {
        <&str as sqlx::Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as sqlx::Type<Postgres>>::compatible(ty)
    }
}

impl<'a> FromParam<'a> for Series {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.parse().map_err(|()| param)
    }
}

impl UriDisplay<Path> for Series {
    fn fmt(&self, f: &mut uri::fmt::Formatter<'_, Path>) -> fmt::Result {
        UriDisplay::fmt(self.slug(), f) // assume all series names are URI safe
    }
}

impl_from_uri_param_identity!([Path] Series);
