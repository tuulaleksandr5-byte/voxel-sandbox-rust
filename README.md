# Voxel Sandbox — Windows x64, HUD и звуки

**Для входа в мир нужен исходный `assets/atlas_128x128.png`. PNG ещё не предоставлен и НЕ включён в архив. Без него меню показывает ошибку и мир не открывается. Процедурные текстуры удалены.**

[Скачать ZIP с EXE](VoxelSandbox-Windows-x64.zip?raw=true). Распакуйте весь архив, положите оригинальный PNG 768×384 (6×3 тайла по128) в `assets/atlas_128x128.png` рядом с EXE и запустите игру. Можно указать исходный PNG через `VOXEL_ATLAS`. При обновлении сохраняйте `saves`.

Удалены обе верхние панели; F3 скрыт по умолчанию. Хотбар из9 кубов на фиолетовой панели, грани85%/70%, контур, яркий выбранный слот. Иконки запекаются из оригинала один раз. Добавлены135 WAV-звуков:15 блоков × break/step/place ×3 варианта; случайный питч ±5%, положение и затухание, шаги0.4с/0.25с при беге.

Проверены87 тестов Linux и46 Windows через Wine; UI и мир проверены с отдельным тестовым PNG, который НЕ включён в пакет. Настоящая Windows, аппаратный GPU и динамики пока не проверены.

[Исходники](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/feature/playable-foundation) · [Отчёт](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/blob/feature/playable-foundation/docs/HUD_ICONS_AUDIO.md)

Source:`5dd222198c356889f2cb8d5c7bc8c8c2f930b7a6`

EXE SHA-256:`3d02f2677b910cc17c64e3d7f72a8b5d92167e49469257b4ddf9cbcbea15fa2b`

ZIP SHA-256:`5c8db52dfd3f67e2f78781c512f1cbfefbd8ab66558818c79de4f7a1e8f803f5`
