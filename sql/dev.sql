
-- RESET working Database without drop all
ALTER SEQUENCE turn_ordering RESTART WITH 0;
Alter sequence player_ids restart with 1;
ALTER SEQUENCE game_ids RESTART with 0;
truncate game cascade ;
truncate player_rack cascade ;
truncate turn cascade ;
truncate player_rack_turn cascade ;
truncate boneyard cascade ;
truncate group_set cascade ;
truncate run cascade ;
-- Basic Set Up
insert into game DEFAULT VALUES;
insert into player_rack (game_id) (select game.game_id from game limit 1);
insert into turn (game_id, active_player)
values ((select game_id from game limit 1), (select player_id from player_rack limit 1));
insert into player_rack_turn (turn_id, game_id, player_id, tiles)
values ((select turn_id from turn limit 1),
        (select game_id from game limit 1),
        (select player_id from player_rack limit 1),
        array [('RegularTile'::tile_sum_type, ('Red'::color, 1)::colornumber)::tile_type]::tile_type[]);
insert into boneyard (game_id, turn_id, ordering, tile)
values ((select turn_id from turn limit 1),
        (select game_id from game limit 1),
        0,
        ('RegularTile'::tile_sum_type, ('Blue'::color, 1)::colornumber)::tile_type)
insert into group_set (game_id, turn_id, num, colors)
values ((select turn_id from turn limit 1),
        (select game_id from game limit 1),
        (5::number),
        (array ['Red', 'Blue', 'Orange']::color[]));
insert into run (game_id, turn_id, start, finish, color, jokers)
values ((select turn_id from turn limit 1),
        (select game_id from game limit 1),
        1::number, 4::number, 'Blue', array [1, 2]);

-- To save to rustkub main project
-- source jaco-dev.env
-- pg_dump --schema=rustikub -s > /home/jaco/Workshop/rustikub/resources/pg_rustikub_schema_dump.sql