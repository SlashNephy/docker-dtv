ALTER TABLE `rule`
  ADD `title` TEXT NULL AFTER `id`,
  ADD `comment` TEXT NULL AFTER `title`;
