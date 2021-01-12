# substrate-lessonpro
## lesson#2 homework
### 1.指出视频中实现kitties的一个bug
 ```
答: 在进行kitty转账的时候未判断被转账kitty是否属于sender
 ```
### 2.KittyIndex不在pallet中指定，而是在runtime里面绑定
```
在runtime中指定KittyIndex,在pallet中引用
```
### 3.扩展存储，能得到一个账号拥有的所有kitties
```
新增UserKitties存储，在create和breed时增加
在transfer时，从旧用户中删除，增加到新用户
```

### 4.设计一个好的数据结构使得
#### 4.1.能得到一个kitty的parents,brothers,children.以及和他一起breed过的另一半.
```
1.定义一个结构体保存kitty的parents，children，breed，其中children/breed单独组成一个结构嵌套在里面，
这样可以children与breed对应上。
2.至于brothers则可以直接找父母的children就可以了，这里没有记录，再查询的时候使用get_brothers获取
```
#### 4.2.分析时间复杂度，尽量使得操作的较为高效.
```
T(n)
O(n)
```

### 5.测试代码能检查event，能测试所有的三个方法，能测试出所有定义的错误类型.
```
tests_breed.rs
tests_create.rs
tests_transfer.rs
```

### 6.create和breed需要质押一定数量的token，在transfer的时候能转移质押.
```
实现描述：
1.create和breed时候会质押5个代币，变量KittyLockToken定义在runtime中，若余额不足则报错BalanceNotEnough。
2.transfer时会先判断to的余额是否足够，若足够，则解锁from质押的代币，再质押上to的代币。
3.每次质押采用唯一LockId，并与Kitty绑定，每次解锁只会解锁一只Kitty对应的代币。
4.质押时的Id为KittyId-decode成[u8;8],但是不会将KittyIndex转化，所以采用较繁琐的方法。如果老师有时间，麻烦这里给个思路。
```