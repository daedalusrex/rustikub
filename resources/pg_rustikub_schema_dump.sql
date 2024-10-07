--
-- PostgreSQL database dump
--

-- Dumped from database version 16.4 (Ubuntu 16.4-1.pgdg22.04+1)
-- Dumped by pg_dump version 16.4 (Ubuntu 16.4-1.pgdg22.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: rustikub; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA rustikub;


ALTER SCHEMA rustikub OWNER TO postgres;

--
-- Name: SCHEMA rustikub; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA rustikub IS 'For my rustikub project based on rummikub';


--
-- Name: color; Type: TYPE; Schema: rustikub; Owner: postgres
--

CREATE TYPE rustikub.color AS ENUM (
    'Red',
    'Blue',
    'Orange',
    'Black'
);


ALTER TYPE rustikub.color OWNER TO postgres;

--
-- Name: number; Type: DOMAIN; Schema: rustikub; Owner: postgres
--

CREATE DOMAIN rustikub.number AS integer
	CONSTRAINT number_check CHECK (((VALUE > 0) AND (VALUE <= 13)));


ALTER DOMAIN rustikub.number OWNER TO postgres;

--
-- Name: colornumber; Type: TYPE; Schema: rustikub; Owner: postgres
--

CREATE TYPE rustikub.colornumber AS (
	col rustikub.color,
	num rustikub.number
);


ALTER TYPE rustikub.colornumber OWNER TO postgres;

--
-- Name: tile_sum_type; Type: TYPE; Schema: rustikub; Owner: postgres
--

CREATE TYPE rustikub.tile_sum_type AS ENUM (
    'RegularTile',
    'JokersWild'
);


ALTER TYPE rustikub.tile_sum_type OWNER TO postgres;

--
-- Name: tile_type_raw; Type: TYPE; Schema: rustikub; Owner: postgres
--

CREATE TYPE rustikub.tile_type_raw AS (
	regular_or_joker rustikub.tile_sum_type,
	col_num rustikub.colornumber
);


ALTER TYPE rustikub.tile_type_raw OWNER TO postgres;

--
-- Name: tile_type; Type: DOMAIN; Schema: rustikub; Owner: postgres
--

CREATE DOMAIN rustikub.tile_type AS rustikub.tile_type_raw
	CONSTRAINT joker_colornumber_is_null CHECK ((NOT (((VALUE).regular_or_joker = 'JokersWild'::rustikub.tile_sum_type) AND ((VALUE).col_num IS NOT NULL))))
	CONSTRAINT regtile_colornumber_not_null CHECK ((NOT (((VALUE).regular_or_joker = 'RegularTile'::rustikub.tile_sum_type) AND ((VALUE).col_num IS NULL))));


ALTER DOMAIN rustikub.tile_type OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: boneyard; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.boneyard (
    game_id bigint NOT NULL,
    turn_id bigint NOT NULL,
    ordering integer NOT NULL,
    tile rustikub.tile_type
);


ALTER TABLE rustikub.boneyard OWNER TO postgres;

--
-- Name: color_as_table; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.color_as_table (
    c text NOT NULL,
    CONSTRAINT color_as_table_c_check CHECK ((c = ANY (ARRAY['red'::text, 'blue'::text, 'orange'::text, 'black'::text])))
);


ALTER TABLE rustikub.color_as_table OWNER TO postgres;

--
-- Name: game; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.game (
    game_id bigint NOT NULL,
    seed text,
    config jsonb,
    outcome jsonb
);


ALTER TABLE rustikub.game OWNER TO postgres;

--
-- Name: game_ids; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.game ALTER COLUMN game_id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME rustikub.game_ids
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- Name: group_set; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.group_set (
    game_id bigint NOT NULL,
    turn_id bigint NOT NULL,
    group_id bigint NOT NULL,
    num rustikub.number,
    colors rustikub.color[],
    CONSTRAINT group_size CHECK (((cardinality(colors) >= 2) AND (cardinality(colors) < 5)))
);


ALTER TABLE rustikub.group_set OWNER TO postgres;

--
-- Name: group_ids; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.group_set ALTER COLUMN group_id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME rustikub.group_ids
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: instances_of_tiles_owned; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.instances_of_tiles_owned (
    id integer NOT NULL,
    tile_type rustikub.tile_sum_type NOT NULL,
    col_num rustikub.colornumber,
    owner text,
    CONSTRAINT instances_of_tiles_owned_check CHECK ((NOT ((tile_type = 'RegularTile'::rustikub.tile_sum_type) AND (col_num IS NULL)))),
    CONSTRAINT instances_of_tiles_owned_check1 CHECK ((NOT ((tile_type = 'JokersWild'::rustikub.tile_sum_type) AND (col_num IS NOT NULL))))
);


ALTER TABLE rustikub.instances_of_tiles_owned OWNER TO postgres;

--
-- Name: instances_of_tiles_owned_id_seq; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.instances_of_tiles_owned ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME rustikub.instances_of_tiles_owned_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: player_rack; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.player_rack (
    player_id smallint NOT NULL,
    game_id bigint NOT NULL,
    strategy jsonb
);


ALTER TABLE rustikub.player_rack OWNER TO postgres;

--
-- Name: player_ids; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.player_rack ALTER COLUMN player_id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME rustikub.player_ids
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: player_rack_turn; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.player_rack_turn (
    turn_id bigint NOT NULL,
    game_id bigint NOT NULL,
    player_id smallint NOT NULL,
    tiles rustikub.tile_type[]
);


ALTER TABLE rustikub.player_rack_turn OWNER TO postgres;

--
-- Name: possible_tile_types; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.possible_tile_types (
    tile_type rustikub.tile_sum_type NOT NULL,
    col_num rustikub.colornumber,
    CONSTRAINT possible_tile_types_check CHECK ((NOT ((tile_type = 'RegularTile'::rustikub.tile_sum_type) AND (col_num IS NULL)))),
    CONSTRAINT possible_tile_types_check1 CHECK ((NOT ((tile_type = 'JokersWild'::rustikub.tile_sum_type) AND (col_num IS NOT NULL))))
);


ALTER TABLE rustikub.possible_tile_types OWNER TO postgres;

--
-- Name: regular_tile_colored_numbers; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.regular_tile_colored_numbers (
    col_num rustikub.colornumber NOT NULL
);


ALTER TABLE rustikub.regular_tile_colored_numbers OWNER TO postgres;

--
-- Name: run; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.run (
    game_id bigint NOT NULL,
    turn_id bigint NOT NULL,
    run_id bigint NOT NULL,
    start rustikub.number,
    finish rustikub.number,
    color rustikub.color,
    jokers rustikub.number[],
    CONSTRAINT max_jokers CHECK ((cardinality(jokers) <= 2))
);


ALTER TABLE rustikub.run OWNER TO postgres;

--
-- Name: run_ids; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.run ALTER COLUMN run_id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME rustikub.run_ids
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: scratch; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.scratch (
    color_col rustikub.color DEFAULT 'Red'::rustikub.color,
    num rustikub.number DEFAULT 1,
    my_tuple rustikub.colornumber DEFAULT ROW('Red'::rustikub.color, (1)::rustikub.number)
);


ALTER TABLE rustikub.scratch OWNER TO postgres;

--
-- Name: turn; Type: TABLE; Schema: rustikub; Owner: postgres
--

CREATE TABLE rustikub.turn (
    turn_id bigint NOT NULL,
    game_id bigint NOT NULL,
    active_player smallint NOT NULL
);


ALTER TABLE rustikub.turn OWNER TO postgres;

--
-- Name: turn_ordering; Type: SEQUENCE; Schema: rustikub; Owner: postgres
--

ALTER TABLE rustikub.turn ALTER COLUMN turn_id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME rustikub.turn_ordering
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- Name: boneyard boneyard_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.boneyard
    ADD CONSTRAINT boneyard_pkey PRIMARY KEY (game_id, turn_id, ordering);


--
-- Name: color_as_table color_as_table_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.color_as_table
    ADD CONSTRAINT color_as_table_pkey PRIMARY KEY (c);


--
-- Name: game game_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.game
    ADD CONSTRAINT game_pkey PRIMARY KEY (game_id);


--
-- Name: group_set group_set_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.group_set
    ADD CONSTRAINT group_set_pkey PRIMARY KEY (game_id, turn_id, group_id);


--
-- Name: instances_of_tiles_owned instances_of_tiles_owned_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.instances_of_tiles_owned
    ADD CONSTRAINT instances_of_tiles_owned_pkey PRIMARY KEY (id);


--
-- Name: player_rack player_rack_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.player_rack
    ADD CONSTRAINT player_rack_pkey PRIMARY KEY (game_id, player_id);


--
-- Name: player_rack_turn player_rack_turn_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.player_rack_turn
    ADD CONSTRAINT player_rack_turn_pkey PRIMARY KEY (game_id, turn_id, player_id);


--
-- Name: possible_tile_types possible_tile_types_tile_type_col_num_key; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.possible_tile_types
    ADD CONSTRAINT possible_tile_types_tile_type_col_num_key UNIQUE (tile_type, col_num);


--
-- Name: regular_tile_colored_numbers regular_tile_colored_numbers_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.regular_tile_colored_numbers
    ADD CONSTRAINT regular_tile_colored_numbers_pkey PRIMARY KEY (col_num);


--
-- Name: run run_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.run
    ADD CONSTRAINT run_pkey PRIMARY KEY (game_id, turn_id, run_id);


--
-- Name: turn turn_game_id_turn_id_active_player_key; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.turn
    ADD CONSTRAINT turn_game_id_turn_id_active_player_key UNIQUE (game_id, turn_id, active_player);


--
-- Name: turn turn_pkey; Type: CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.turn
    ADD CONSTRAINT turn_pkey PRIMARY KEY (game_id, turn_id);


--
-- Name: boneyard boneyard_game_id_turn_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.boneyard
    ADD CONSTRAINT boneyard_game_id_turn_id_fkey FOREIGN KEY (game_id, turn_id) REFERENCES rustikub.turn(game_id, turn_id);


--
-- Name: group_set group_set_game_id_turn_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.group_set
    ADD CONSTRAINT group_set_game_id_turn_id_fkey FOREIGN KEY (game_id, turn_id) REFERENCES rustikub.turn(game_id, turn_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: player_rack player_rack_game_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.player_rack
    ADD CONSTRAINT player_rack_game_id_fkey FOREIGN KEY (game_id) REFERENCES rustikub.game(game_id);


--
-- Name: player_rack_turn player_rack_turn_game_id_player_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.player_rack_turn
    ADD CONSTRAINT player_rack_turn_game_id_player_id_fkey FOREIGN KEY (game_id, player_id) REFERENCES rustikub.player_rack(game_id, player_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: player_rack_turn player_rack_turn_game_id_turn_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.player_rack_turn
    ADD CONSTRAINT player_rack_turn_game_id_turn_id_fkey FOREIGN KEY (game_id, turn_id) REFERENCES rustikub.turn(game_id, turn_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: run run_game_id_turn_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.run
    ADD CONSTRAINT run_game_id_turn_id_fkey FOREIGN KEY (game_id, turn_id) REFERENCES rustikub.turn(game_id, turn_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: turn turn_game_id_active_player_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.turn
    ADD CONSTRAINT turn_game_id_active_player_fkey FOREIGN KEY (game_id, active_player) REFERENCES rustikub.player_rack(game_id, player_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: turn turn_game_id_fkey; Type: FK CONSTRAINT; Schema: rustikub; Owner: postgres
--

ALTER TABLE ONLY rustikub.turn
    ADD CONSTRAINT turn_game_id_fkey FOREIGN KEY (game_id) REFERENCES rustikub.game(game_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

