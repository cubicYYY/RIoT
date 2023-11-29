// @generated automatically by Diesel CLI.

diesel::table! {
    device (id) {
        id -> Unsigned<Bigint>,
        uid -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        desc -> Nullable<Text>,
        dtype -> Unsigned<Integer>,
        since -> Datetime,
        last_update -> Datetime,
        activated -> Bool,
    }
}

diesel::table! {
    owns (sid, did) {
        sid -> Unsigned<Bigint>,
        did -> Unsigned<Bigint>,
    }
}

diesel::table! {
    record (id) {
        id -> Unsigned<Bigint>,
        did -> Unsigned<Bigint>,
        #[max_length = 1]
        payload -> Binary,
        latitude -> Nullable<Double>,
        longitude -> Nullable<Double>,
        timestamp -> Datetime,
    }
}

diesel::table! {
    site (id) {
        id -> Unsigned<Bigint>,
        uid -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        desc -> Nullable<Text>,
        activated -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        privilege -> Unsigned<Integer>,
        #[max_length = 64]
        api_key -> Nullable<Varchar>,
        since -> Datetime,
        activated -> Bool,
    }
}

diesel::joinable!(device -> user (uid));
diesel::joinable!(owns -> device (did));
diesel::joinable!(owns -> site (sid));
diesel::joinable!(record -> device (did));
diesel::joinable!(site -> user (uid));

diesel::allow_tables_to_appear_in_same_query!(device, owns, record, site, user,);
