# Контракты ядра — Issue #1

`game_core` не зависит от Bevy, файловых форматов и конкретных подсистем.
Клиент сохраняет Bevy `=0.20.1`. Здесь нет генерации мира, графики, движения,
мобов или механических сетей (#22). Все приведённые интерфейсы — общие границы;
реализации сервисов появятся в соответствующих crates.

## ID и владение

- `BlockId::new(u32)` / `raw()`: явный стабильный номер; `BlockId::AIR` = 0.
- `ItemId::new(u32)` / `raw()`: отдельное пространство предметов, включая 0.
- `PlayerId::new(u64)` / `raw()`: стабильный игрок, а не `bevy::Entity`.

Реестр контента назначает номера явно и сохраняет их между запусками/версиями.
Не вычислять ID из порядка загрузки, случайного hash или ECS entity. Не
переиспользовать удалённые ID. Core сохраняет любое значение без усечения;
проверку неизвестного ID выполняет реестр. Разные типы не взаимозаменяемы.
Пустой слот — `None`, а не специальный предмет с ID=0.

## Координаты и секции

`BlockPos { x, y, z: i32 }` — адрес блока, `ChunkPos { x, z: i32 }` — колонка
16×16, `SectionPos { x, y, z: i32 }` — адрес секции 16×16×16. Это разные
системы координат, не позиции в метрах и не локальные координаты секции.

- `BlockPos::chunk()`, `section()`, `local()` используют евклидово деление.
  Блок (-1,-1,-17) → секция (-1,-1,-2), локальный адрес (15,15,15).
- `LocalBlockPos::new(x: u8, y: u8, z: u8) -> Option<Self>` допускает 0..16
  по каждой оси; компоненты доступны через `x()/y()/z()`.
- `index() -> usize`: `x + 16*z + 256*y`; `from_index(usize)` принимает 0..4096.
  Это порядок ячеек в памяти, а не версия или готовый формат файла.
- `SectionPos::block_at(local) -> Option<BlockPos>` проверяет переполнение i32
  и границы Y. `is_in_world()` проверяет допустимость адреса секции.
- `BlockPos::is_in_world()` / `valid_world_height()` принимают Y=-64..320
  включительно. По X/Z поддерживается весь диапазон i32.

`Section::new(position, fill) -> Option<Section>` создаёт небольшой объект
данных с ёмкостью 4096 блоков. `get(local) -> Option<BlockId>` читает ячейку,
`set(local, block) -> Option<BlockId>` возвращает прежний ID. `position()`
возвращает адрес. Неверная позиция секции не создаётся. Это контейнер данных
для передачи между модулями, а не менеджер мира/чанков.

Секции Y=-4..20 пересекают мир. В верхней секции Y=20 допустим только локальный
Y=0, соответствующий глобальному Y=320. Остальные 15 слоёв недоступны для
чтения/записи и не должны попадать в мир. Секция клонируется независимо;
владение и синхронизацию живых секций организует `game_world`.

Существующие `section_coordinate`, `local_coordinate`, `valid_world_height`
и константы доступны в корне crate для совместимости.

## Запросы и факты

`simulation::GameCommand` — намерение клиента:

- `BreakBlock { player, position }`;
- `PlaceBlock { player, position, block }`;
- `SelectSlot { player, slot }`;
- `MoveStack { player, from, to, quantity: NonZeroU32 }` — внутри инвентаря игрока.

`CommandSink::submit(GameCommand) -> Result<(), Self::Error>` принимает запрос
в обработку; успех постановки в очередь не означает успешного исполнения.
Команды могут содержать недопустимые координаты/неизвестные ID: симуляция
обязана проверить права, дальность, загрузку секции, выбранный предмет,
количество, свободное место и реестр. PlaceBlock не даёт права создавать
произвольный блок и не списывает инвентарь автоматически.

`simulation::GameEvent` — уже подтверждённый результат авторитетной симуляции:

- `BlockChanged(BlockChanged { position, previous, current })`;
- `InventoryChanged { player, slot, stack: Option<ItemStack> }`;
- `SelectedSlotChanged { player, slot }`.

Оба enum помечены `#[non_exhaustive]`; внешние обработчики обязаны иметь
ветку `_`. Новые варианты можно добавить без поломки таких обработчиков.
Изменение существующих payload, ID, индексации или семантики требует
согласования со смежными задачами. Не сериализовать enum по порядку вариантов:
wire/save-схемы и их версионирование разрабатываются отдельно.
Bevy-модули могут оборачивать эти типы в свои Message/Event/Resource.

## Интерфейсы модулей

| Модуль core | Публичный интерфейс | Ответственность реализации |
|---|---|---|
| `world` | `WorldRead::block(BlockPos) -> Result<BlockId, WorldAccessError>` | Отделять `OutOfBounds` от `SectionUnavailable`; незагруженный мир не равен AIR |
| `world` | `WorldWrite::set_block(BlockPos, BlockId) -> Result<Option<BlockChanged>, WorldAccessError>` | Валидировать адрес, возвращать None при отсутствии изменения; событие — после изменения |
| `render` | `SectionRenderer::update_section(&Section, &dyn WorldRead)` / `remove_section(SectionPos)` | Построение/удаление представления; читать соседей через WorldRead, не менять мир |
| `player` | `PlayerView::state(PlayerId) -> Option<PlayerState>`; `PlayerState { block_position, selected_slot }` | Снимок адреса блока и выбранного слота; дробную позицию/скорость хранит будущий player-модуль |
| `inventory` | `InventoryRead::slot_count() -> u16`, `stack(SlotId) -> Result<Option<ItemStack>, SlotOutOfBounds>` | Отличать пустой слот от неверного адреса |
| `inventory` | `InventoryWrite::set_stack(SlotId, Option<ItemStack>) -> Result<Option<ItemStack>, SlotOutOfBounds>` | Возвращать прежний стек; лимиты предметов/транзакции — в game_items |
| `persistence` | `SectionStore::load_metadata/save_metadata`, `load_section/save_section` | Хранение метаданных и снимков секций; свои ошибки через associated Error |

`SectionRenderer` возвращает `Result<(), Self::Error>`; `SectionStore` возвращает
`Result<Option<T>, Self::Error>` при загрузке (None — записи нет), `Result<(),
Self::Error>` при сохранении. `WorldMetadata` содержит `schema_version: u32`,
`world_seed: u64`, `generator_version: u32`; значения согласует persistence с
world. Метаданные — заголовок будущей схемы, а не уже реализованное сохранение
игроков/инвентарей. Проверка поддерживаемой версии, атомарная запись и миграции —
обязанность persistence; данные неизвестной версии нельзя молча принимать.

Traits не требуют Bevy, Send/Sync или конкретного планировщика задач. Future
crates могут выбрать внутреннее хранение, асинхронный ввод/вывод и адаптеры,
предоставляя согласованную синхронную границу. Асинхронная оркестрация и
транзакции не реализованы. Рендеру нужен согласованный снимок секции и соседей;
world/симуляция обеспечивают порядок изменений.

## Безопасная параллельная разработка

После принятия этого PR можно независимо реализовать:

1. **World**: менеджер секций/чанков, WorldRead/WorldWrite, генератор в своей задаче.
2. **Render**: meshing и SectionRenderer, используя Section и WorldRead соседей.
3. **Player**: ввод/камера/движение в game_player, команды через CommandSink,
   запросы коллизий/raycast через WorldRead, снимок через PlayerView.
4. **Items**: реестр ItemId, InventoryRead/InventoryWrite, правила стеков/хотбар.
5. **Persistence**: SectionStore, формат, проверка версий, безопасная запись.
6. **Simulation/UI/QA**: авторитетная обработка GameCommand, потребление GameEvent,
   интерфейс выбранного слота и тестовые сценарии.

Каждый поток меняет свой crate и тесты. Общие Cargo workspace entries и изменения
core интегратор согласует отдельно. Транзакцию «блок + списание/дроп предметов»
выполняет симуляция: отдельные WorldWrite и InventoryWrite сами не обеспечивают
атомарность. Не публиковать события из рендера или ввода. Межмодульная интеграция
и полные сохранения требуют отдельных тестов, не подменяются этими контрактами.

## Проверка

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Тесты consumer-контрактов используют независимые реализации в памяти для
проверки границ API; это не реализация world/render/persistence и не тест
дискового формата. Реальные тесты поведения ядра проверяют ID, непустые стеки,
координаты, индексы и границы секций.
