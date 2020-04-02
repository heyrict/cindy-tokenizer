table! {
    auth_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    auth_group_permissions (id) {
        id -> Int4,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    auth_permission (id) {
        id -> Int4,
        name -> Varchar,
        content_type_id -> Int4,
        codename -> Varchar,
    }
}

table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

table! {
    django_content_type (id) {
        id -> Int4,
        app_label -> Varchar,
        model -> Varchar,
    }
}

table! {
    django_migrations (id) {
        id -> Int4,
        app -> Varchar,
        name -> Varchar,
        applied -> Timestamptz,
    }
}

table! {
    django_session (session_key) {
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

table! {
    hasura_directmessage_group_trigger (user_id) {
        user_id -> Int4,
        last_dm_id -> Int4,
    }
}

table! {
    hasura_int_groupby_trigger (group) {
        group -> Int4,
        value -> Int8,
    }
}

table! {
    hasura_user_ranking_trigger (user_id) {
        user_id -> Int4,
        value -> Int8,
    }
}

table! {
    sui_hei_award (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        groupName -> Varchar,
        requisition -> Text,
    }
}

table! {
    sui_hei_awardapplication (id) {
        id -> Int4,
        status -> Int4,
        comment -> Text,
        created -> Timestamptz,
        reviewed -> Nullable<Timestamptz>,
        applier_id -> Int4,
        award_id -> Int4,
        reviewer_id -> Nullable<Int4>,
        reason -> Text,
    }
}

table! {
    sui_hei_bookmark (id) {
        id -> Int4,
        value -> Int2,
        puzzle_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_chatmessage (id) {
        id -> Int4,
        content -> Text,
        created -> Nullable<Timestamptz>,
        editTimes -> Int4,
        chatroom_id -> Int4,
        user_id -> Int4,
        modified -> Timestamptz,
    }
}

table! {
    sui_hei_chatroom (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        created -> Date,
        user_id -> Int4,
        private -> Bool,
    }
}

table! {
    sui_hei_comment (id) {
        id -> Int4,
        content -> Text,
        spoiler -> Bool,
        puzzle_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_dialogue (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        good -> Bool,
        #[sql_name = "true"]
        true_ -> Bool,
        created -> Timestamptz,
        answeredtime -> Nullable<Timestamptz>,
        puzzle_id -> Int4,
        user_id -> Int4,
        answerEditTimes -> Int4,
        questionEditTimes -> Int4,
        qno -> Int4,
        modified -> Timestamptz,
    }
}

table! {
    sui_hei_directmessage (id) {
        id -> Int4,
        content -> Text,
        created -> Timestamptz,
        receiver_id -> Int4,
        sender_id -> Int4,
        editTimes -> Int4,
    }
}

table! {
    sui_hei_event (id) {
        id -> Int4,
        title -> Varchar,
        banner_img_url -> Varchar,
        status -> Int4,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
        page_link -> Varchar,
        page_src -> Text,
        user_id -> Int4,
    }
}

table! {
    sui_hei_eventaward (id) {
        id -> Int4,
        award_id -> Int4,
        event_id -> Int4,
    }
}

table! {
    sui_hei_favoritechatroom (id) {
        id -> Int4,
        chatroom_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_hint (id) {
        id -> Int4,
        content -> Text,
        created -> Timestamptz,
        puzzle_id -> Int4,
        edittimes -> Int4,
        receiver_id -> Nullable<Int4>,
        modified -> Timestamptz,
    }
}

table! {
    sui_hei_puzzle (id) {
        id -> Int4,
        title -> Varchar,
        yami -> Int4,
        genre -> Int4,
        content -> Text,
        solution -> Text,
        created -> Timestamptz,
        modified -> Timestamptz,
        status -> Int4,
        memo -> Text,
        user_id -> Int4,
        anonymous -> Bool,
        dazed_on -> Date,
        grotesque -> Bool,
    }
}

table! {
    sui_hei_puzzle_tag (id) {
        id -> Int4,
        puzzle_id -> Int4,
        tag_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_puzzle_tokenize_cache (id) {
        id -> Int4,
        puzzle_id -> Int4,
        tokens -> Jsonb,
    }
}

table! {
    sui_hei_schedule (id) {
        id -> Int4,
        content -> Text,
        created -> Timestamptz,
        scheduled -> Timestamptz,
        user_id -> Int4,
    }
}

table! {
    sui_hei_star (id) {
        id -> Int4,
        value -> Int2,
        puzzle_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_tag (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamptz,
    }
}

table! {
    sui_hei_user (id) {
        id -> Int4,
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
        nickname -> Varchar,
        profile -> Text,
        current_award_id -> Nullable<Int4>,
        hide_bookmark -> Bool,
        last_read_dm_id -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
    }
}

table! {
    sui_hei_useraward (id) {
        id -> Int4,
        created -> Date,
        award_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sui_hei_user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    sui_hei_user_user_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        permission_id -> Int4,
    }
}

joinable!(auth_group_permissions -> auth_group (group_id));
joinable!(auth_group_permissions -> auth_permission (permission_id));
joinable!(auth_permission -> django_content_type (content_type_id));
joinable!(django_admin_log -> django_content_type (content_type_id));
joinable!(django_admin_log -> sui_hei_user (user_id));
joinable!(hasura_directmessage_group_trigger -> sui_hei_user (user_id));
joinable!(hasura_user_ranking_trigger -> sui_hei_user (user_id));
joinable!(sui_hei_awardapplication -> sui_hei_award (award_id));
joinable!(sui_hei_bookmark -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_bookmark -> sui_hei_user (user_id));
joinable!(sui_hei_chatmessage -> sui_hei_chatroom (chatroom_id));
joinable!(sui_hei_chatmessage -> sui_hei_user (user_id));
joinable!(sui_hei_chatroom -> sui_hei_user (user_id));
joinable!(sui_hei_comment -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_comment -> sui_hei_user (user_id));
joinable!(sui_hei_dialogue -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_dialogue -> sui_hei_user (user_id));
joinable!(sui_hei_event -> sui_hei_user (user_id));
joinable!(sui_hei_eventaward -> sui_hei_award (award_id));
joinable!(sui_hei_eventaward -> sui_hei_event (event_id));
joinable!(sui_hei_favoritechatroom -> sui_hei_chatroom (chatroom_id));
joinable!(sui_hei_favoritechatroom -> sui_hei_user (user_id));
joinable!(sui_hei_hint -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_hint -> sui_hei_user (receiver_id));
joinable!(sui_hei_puzzle -> sui_hei_user (user_id));
joinable!(sui_hei_puzzle_tag -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_puzzle_tag -> sui_hei_tag (tag_id));
joinable!(sui_hei_puzzle_tag -> sui_hei_user (user_id));
joinable!(sui_hei_puzzle_tokenize_cache -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_schedule -> sui_hei_user (user_id));
joinable!(sui_hei_star -> sui_hei_puzzle (puzzle_id));
joinable!(sui_hei_star -> sui_hei_user (user_id));
joinable!(sui_hei_user_groups -> auth_group (group_id));
joinable!(sui_hei_user_groups -> sui_hei_user (user_id));
joinable!(sui_hei_user_user_permissions -> auth_permission (permission_id));
joinable!(sui_hei_user_user_permissions -> sui_hei_user (user_id));
joinable!(sui_hei_useraward -> sui_hei_award (award_id));

allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
    hasura_directmessage_group_trigger,
    hasura_int_groupby_trigger,
    hasura_user_ranking_trigger,
    sui_hei_award,
    sui_hei_awardapplication,
    sui_hei_bookmark,
    sui_hei_chatmessage,
    sui_hei_chatroom,
    sui_hei_comment,
    sui_hei_dialogue,
    sui_hei_directmessage,
    sui_hei_event,
    sui_hei_eventaward,
    sui_hei_favoritechatroom,
    sui_hei_hint,
    sui_hei_puzzle,
    sui_hei_puzzle_tag,
    sui_hei_puzzle_tokenize_cache,
    sui_hei_schedule,
    sui_hei_star,
    sui_hei_tag,
    sui_hei_user,
    sui_hei_useraward,
    sui_hei_user_groups,
    sui_hei_user_user_permissions,
);
