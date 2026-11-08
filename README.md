# Voxel Sandbox — Windows x64: исходные текстуры и значок

[Скачать ZIP с EXE](VoxelSandbox-Windows-x64.zip?raw=true). Распакуйте архив и запустите `VoxelSandbox.exe`. Все ресурсы встроены; отдельный атлас и установка Rust не нужны. Можно запускать один EXE. При обновлении перенесите папку `saves`.

Подключены все17 оригинальных PNG из присланного `textures_cut.zip` без изменения файлов и пикселей. Контрольные суммы: `assets/textures/source-hashes.json`. Гравий и мшистый булыжник теперь используют отдельные присланные текстуры. Генератора подменных текстур нет.

Значок приложения подготовлен по присланному логотипу, встроен в EXE и назначен окну/панели задач. HUD: прицел и фиолетовый хотбар из9 кубических иконок. F3 скрыт по умолчанию. Звуки:135 WAV, по3 варианта break/step/place на15 блоков, питч ±5%, пространственное положение и затухание.

Управление: Ctrl — бег/ускорение полёта; двойное W — бег; двойной Space — полёт, Space/Shift — вверх/вниз. Колёсико/1–9 — слот; F3 — координаты/FPS; F3+G — чанки.

Проверены92 теста Linux и51 Windows через Wine. Самостоятельный EXE из папки без ресурсов открывает мир с исходными текстурами через Wine/Vulkan. Настоящая Windows/аппаратный GPU/динамики пока не проверены.

[Исходники](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/feature/playable-foundation) · [Отчёт и снимки](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/blob/feature/playable-foundation/docs/ORIGINAL_TEXTURES.md)

Source:`74d34332a587131d3b462c1a99fa21b5769ec13b`

EXE SHA-256:`fdc0ec271060ebc5ff015df232a38884a46bb741da91432068bba23570dae4ff`

ZIP SHA-256:`08dc91506e468513aa08261126356729660f1255e33835fc9adff91487bc6986`
