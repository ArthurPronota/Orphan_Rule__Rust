# Orphan Rule в Rust

## Что такое Orphan Rule

**Orphan Rule** (правило сироты) — это правило когерентности (coherence) в Rust, которое **ограничивает**, где можно реализовывать **trait** для **типа**.

**Правило:**

> Реализовать trait для типа можно **только** если **хотя бы одно** из следующего верно:
> - **trait** определён **в текущем** крейте;
> - **тип** определён **в текущем** крейте;
> - **оба** — в текущем крейте.

**Нельзя** реализовать **чужой** trait для **чужого** типа.

## Формально

```rust
impl Trait for Type { ... }
```

| `Trait` в моём крейте | `Type` в моём крейте | Разрешено? |
|---|---|---|
| ✅ | ✅ | ✅ Да |
| ✅ | ❌ | ✅ Да (мой trait) |
| ❌ | ✅ | ✅ Да (мой type) |
| ❌ | ❌ | ❌ **Нет** |

## Пример: **разрешено**

### Мой trait, чужой type

```rust
// Мой крейт
trait MyTrait {
    fn hello(&self);
}

// Чужой тип — например, Vec из std
impl MyTrait for Vec<i32> {
    fn hello(&self) {
        println!("Hello from MyTrait for Vec");
    }
}
```

✅ **Разрешено** — потому что `MyTrait` **мой**.

### Чужой trait, мой type

```rust
// Мой тип
struct MyStruct;

// Чужой trait — Display из std
impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct")
    }
}
```

✅ **Разрешено** — потому что `MyStruct` **мой**.

## Пример: **запрещено**

```rust
// Чужой trait — Display из std
// Чужой type — Vec из std
impl std::fmt::Display for Vec<i32> {   // ❌ ОШИБКА
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "my Vec")
    }
}
```

**Ошибка:**

```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> src/main.rs:3:1
  |
3 | impl std::fmt::Display for Vec<i32> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use only types from inside the current crate
  |
  = note: define and implement a trait or new type instead
```

**Почему:** и `Display`, и `Vec` — **чужие**.

## Зачем нужно Orphan Rule

### 1. **Когерентность** (coherence)

Без правила **две** библиотеки могли бы **реализовать** один trait для одного типа **по-разному**:

```rust
// Библиотека A
impl Display for Vec<i32> {
    fn fmt(&self, f) -> ... {
        write!(f, "A: {:?}", self)
    }
}

// Библиотека B
impl Display for Vec<i32> {
    fn fmt(&self, f) -> ... {
        write!(f, "B: {:?}", self)
    }
}
```

**Проблема:** какая реализация **используется**? Компилятор **не может** выбрать. **Конфликт**.

### 2. **Обратная совместимость**

Без правила:
- Библиотека A добавляет `impl Display for MyType`.
- Библиотека B **тоже** добавляет `impl Display for MyType`.
- Обновление одной → **ломает** другую.

**Orphan Rule** гарантирует, что **только** владелец типа или трейта может реализовать.

### 3. **Предсказуемость**

Компилятор **точно знает**, что реализация **не появится** из **чужого** крейта.

## Обход: Newtype Pattern

Если **нужно** реализовать чужой trait для чужого типа — создайте **свой** тип-обёртку:

```rust
// ❌ Нельзя
// impl Display for Vec<i32> { ... }

// ✅ Можно — свой newtype
struct MyVec(Vec<i32>);

impl Display for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
```

**Ваш пример** — именно **этот** случай:

```rust
struct MyWrap(Vec<i32>);   // ← мой тип

impl Display for MyWrap {   // ✅ Разрешено — MyWrap мой
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
```

- **`MyWrap`** — **локальный** тип.
- **`Display`** — **чужой** trait.
- **Orphan rule** **разрешает** — потому что тип **мой**.

## Разбор вашего примера

```rust
use std::fmt::{Display, write};

struct MyWrap(Vec<i32>);

impl Display for MyWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);   // [1, 2, 3, 4, 5] — через Debug

    let my_wrap = MyWrap(vec![1, 2, 3, 4, 5]);
    println!("{}", my_wrap);   // [1, 2, 3, 4, 5] — через Display
}
```

### Что происходит

- **`Vec<i32>`** — **не реализует** `Display` (только `Debug`).
- **`{:?}`** — работает для `Vec` (через `Debug`).
- **`{}`** — **не работает** для `Vec` → **ошибка компиляции**.
- **`MyWrap`** — **свой** тип → можно реализовать **`Display`**.
- **`{}`** — работает для `MyWrap` (через `Display`).

### Почему `Vec<i32>` не реализует `Display`

**Причина:** `Display` требует **человеко-читаемого** формата. Для `Vec` **неоднозначно**, как его выводить:

- `[1, 2, 3]` — Rust-стиль?
- `1, 2, 3` — CSV?
- `(1 2 3)` — Lisp?

**Поэтому:** `Vec` реализует **только `Debug`** (`{:?}`).

## Сводная таблица

| Trait | Type | Orphan Rule |
|---|---|---|
| Мой | Мой | ✅ |
| Мой | Чужой | ✅ |
| Чужой | Мой | ✅ |
| **Чужой** | **Чужой** | ❌ |

## Примеры из жизни

### `Display` для `Vec` — **нельзя**

```rust
impl Display for Vec<i32> { ... }   // ❌
```

**Решение:** `MyWrap(Vec<i32>)` + `impl Display for MyWrap`.

### `Serialize` для `DateTime` — **нельзя**

```rust
impl Serialize for chrono::DateTime<Utc> { ... }   // ❌
```

**Решение:** `MyDateTime(chrono::DateTime<Utc>)` или использовать **обёртку** `serde_with`.

### `From<MyType>` для `Vec` — **нельзя**

```rust
impl From<MyType> for Vec<i32> { ... }   // ❌
```

**Решение:** `impl From<MyType> for MyVec`.

## Когерентность

**Когерентность** — свойство системы типов, при котором для каждой пары `(trait, type)` существует **не более одной** реализации.

**Orphan Rule** — **гарантирует** когерентность.

## Итог

- **Orphan Rule** — правило, **ограничивающее** где можно реализовать trait.
- **Разрешено**, если **trait** или **type** — **в текущем** крейте.
- **Запрещено** реализовать **чужой** trait для **чужого** типа.
- **Зачем:** **когерентность** (нет конфликтов реализаций).
- **Обход:** **newtype pattern** — обернуть чужой тип в свой.
- **`MyWrap(Vec<i32>)`** + `impl Display for MyWrap` — **разрешено**, потому что `MyWrap` **мой**.
- **`Vec<i32>`** реализует только `Debug`, потому что `Display` **неоднозначен**.
- **Правило:** «trait или type — в текущем крейте → можно реализовать».
