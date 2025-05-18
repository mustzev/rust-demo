--! demos
select * from demos;

--! insert_demo
insert into demos (id, name, created_at)
values (:id, :name, :created_at)
returning id, name, created_at;
