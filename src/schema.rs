table! {
    cinemas (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    cinemas_movies (id) {
        id -> Int4,
        exposed_format -> Nullable<Int2>,
        pixels_box -> Nullable<Array<Float8>>,
        cinema_id -> Int4,
        movie_id -> Int4,
    }
}

table! {
    color_movie_colormap (id) {
        id -> Int4,
        color_movie_id -> Int4,
        colormap_id -> Int4,
    }
}

table! {
    color_movies (id) {
        id -> Int4,
        format -> Int2,
        default_colormap -> Int4,
    }
}

table! {
    colormaps (id) {
        id -> Int4,
        name -> Text,
        colors -> Array<Text>,
        positions -> Array<Float8>,
    }
}

table! {
    images (id) {
        id -> Int4,
        time -> Timestamp,
        path -> Text,
        #[sql_name = "box"]
        box_ -> Nullable<Array<Float8>>,
        color_movie_id -> Int4,
    }
}

table! {
    images_tags_values (id) {
        id -> Int4,
        tags_value_id -> Int4,
        image_id -> Int4,
    }
}

table! {
    movies (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        pixels_box -> Nullable<Array<Float8>>,
        path -> Nullable<Text>,
    }
}

table! {
    movies_tags (id) {
        id -> Int4,
        movie_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    tags_values (id) {
        id -> Int4,
        value -> Text,
        tag_id -> Int4,
    }
}

table! {
    vector_data (id) {
        id -> Int4,
        time -> Timestamp,
        properties -> Nullable<Jsonb>,
        vector_movie_id -> Int4,
    }
}

table! {
    vector_movies (id) {
        id -> Int4,
        default_style -> Nullable<Int4>,
    }
}

table! {
    vector_styles (id) {
        id -> Int4,
        name -> Text,
        style -> Jsonb,
    }
}

table! {
    vector_styles_vector_movies (id) {
        id -> Int4,
        vector_movie_id -> Int4,
        vector_style_id -> Int4,
    }
}

joinable!(cinemas_movies -> cinemas (cinema_id));
joinable!(cinemas_movies -> movies (movie_id));
joinable!(color_movie_colormap -> color_movies (color_movie_id));
joinable!(color_movie_colormap -> colormaps (colormap_id));
joinable!(color_movies -> colormaps (default_colormap));
joinable!(images -> color_movies (color_movie_id));
joinable!(images_tags_values -> images (image_id));
joinable!(images_tags_values -> tags_values (tags_value_id));
joinable!(movies_tags -> movies (movie_id));
joinable!(movies_tags -> tags (tag_id));
joinable!(tags_values -> tags (tag_id));
joinable!(vector_data -> vector_movies (vector_movie_id));
joinable!(vector_movies -> vector_styles (default_style));
joinable!(vector_styles_vector_movies -> vector_movies (vector_movie_id));
joinable!(vector_styles_vector_movies -> vector_styles (vector_style_id));

allow_tables_to_appear_in_same_query!(
    cinemas,
    cinemas_movies,
    color_movie_colormap,
    color_movies,
    colormaps,
    images,
    images_tags_values,
    movies,
    movies_tags,
    tags,
    tags_values,
    vector_data,
    vector_movies,
    vector_styles,
    vector_styles_vector_movies,
);
