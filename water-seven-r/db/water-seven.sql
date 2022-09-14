CREATE TABLE `user` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `role_id` int(10)  unsigned  AUTO_INCREMENT,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `role` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `role_name` varchar(255) NOT NULL DEFAULT (customer)
);

CREATE TABLE `production` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `serial_num` varchar(255) NOT NULL,
  `cas` varchar(255) NOT NULL,
  `inventory` varchar(255) NOT NULL,
  `brand` varchar(255) NOT NULL,
  `category` varchar(255),
  `pro_category` varchar(255),
  `name_zh` varchar(255) NOT NULL,
  `name_en` varchar(255) NOT NULL,
  `size` varchar(255),
  `price` float DEFAULT 0,
  `amount` bigint NOT NULL COMMENT 'must be positive',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `customs` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `name` varchar(255) NOT NULL,
  `phone` varchar(255),
  `group_org` int(10)  unsigned  AUTO_INCREMENT,
  `owner_id` int(10)  unsigned  AUTO_INCREMENT,
  `outside_money` float,
  `discount` float,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `customs_org` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `school` varchar(255),
  `group` varchar(255),
  `addr` varchar(255),
  `group_id` int(10)  unsigned  AUTO_INCREMENT
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `order` (
  `id` int(10)  unsigned  AUTO_INCREMENT PRIMARY KEY,
  `serial_order` varchar(255),
  `production_id` int(10)  unsigned  AUTO_INCREMENT,
  `customer_id` int(10)  unsigned  AUTO_INCREMENT,
  `count` integer,
  `price` float,
  `old_price` float,
  `profits` float,
  `owner_id` int(10)  unsigned  AUTO_INCREMENT,
  `money` float,
  `settled_money` float,
  `invoice_status` float,
  `status` ENUM ('off', 'on'),
  `take_goods` int(10)  unsigned  AUTO_INCREMENT
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `goods_info` (
  `id` int(10)  unsigned  AUTO_INCREMENT,
  `company` varchar(255),
  `code` varchar(255),
  `status` ENUM ('off', 'on'),
  `current_info` varchar(255)
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT'创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE INDEX `user_index_0` ON `user` (`id`);

CREATE INDEX `role_index_1` ON `role` (`id`);

CREATE INDEX `production_index_2` ON `production` (`id`);

CREATE INDEX `customs_index_3` ON `customs` (`id`);

CREATE INDEX `customs_org_index_4` ON `customs_org` (`id`);

ALTER TABLE `user` ADD FOREIGN KEY (`role_id`) REFERENCES `role` (`id`);

ALTER TABLE `customs` ADD FOREIGN KEY (`group_org`) REFERENCES `customs_org` (`id`);
ALTER TABLE water_seven.customs ADD CONSTRAINT customs_FK FOREIGN KEY (group_org) REFERENCES water_seven.customs_org(id);

ALTER TABLE `customs` ADD FOREIGN KEY (`owner_id`) REFERENCES `user` (`id`);


ALTER TABLE `customs_org` ADD FOREIGN KEY (`group_id`) REFERENCES `customs_org` (`id`);

ALTER TABLE `order` ADD FOREIGN KEY (`production_id`) REFERENCES `production` (`id`);

ALTER TABLE `order` ADD FOREIGN KEY (`customer_id`) REFERENCES `customs_org` (`id`);

ALTER TABLE `order` ADD FOREIGN KEY (`owner_id`) REFERENCES `user` (`id`);

ALTER TABLE `order` ADD FOREIGN KEY (`take_goods`) REFERENCES `goods_info` (`id`);
