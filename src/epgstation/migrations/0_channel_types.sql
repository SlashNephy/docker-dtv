ALTER TABLE `rule`
  ADD `EXT1` tinyint NOT NULL DEFAULT 0 AFTER `SKY`,
  ADD `EXT2` tinyint NOT NULL DEFAULT 0 AFTER `EXT1`,
  ADD `EXT3` tinyint NOT NULL DEFAULT 0 AFTER `EXT2`,
  ADD `EXT4` tinyint NOT NULL DEFAULT 0 AFTER `EXT3`,
  ADD `EXT5` tinyint NOT NULL DEFAULT 0 AFTER `EXT4`,
  ADD `EXT6` tinyint NOT NULL DEFAULT 0 AFTER `EXT5`,
  ADD `EXT7` tinyint NOT NULL DEFAULT 0 AFTER `EXT6`,
  ADD `EXT8` tinyint NOT NULL DEFAULT 0 AFTER `EXT7`,
  ADD `EXT9` tinyint NOT NULL DEFAULT 0 AFTER `EXT8`;
