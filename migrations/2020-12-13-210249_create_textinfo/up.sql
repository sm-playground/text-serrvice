-- Your SQL goes here

CREATE TABLE textinfo (
    id BIGINT NOT NULL AUTO_INCREMENT,
    token VARCHAR(256) NOT NULL,
    text VARCHAR(2048) NOT NULL,
    PRIMARY KEY (`id`)
);

insert into textinfo (token, text) values ('AB.CD.EF', 'abcdef');
insert into textinfo (token, text) values ('AB.CD.EF.GH', 'abcdefgh');
insert into textinfo (token, text) values ('AB.CD.EF.GH.KL', 'abcdefghkl');
insert into textinfo (token, text) values ('AB.CD.EF.GH.DELETE.ME', 'Use id 4 to delete me');
insert into textinfo (token, text) values ('AB.CD.EF.GH.KEEP.ME', 'DO NOT Use id 5 to delete a text');