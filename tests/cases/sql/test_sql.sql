-- generate a simple sql to test the sql parser --
with t1 as (
    select * from table1
),
t2 as (
    select * from table2
)
select * from t1 join t2 on t1.id = t2.id