# Voxel Sandbox 2.0 Survival — Stage 2 alpha

[Скачать ZIP с Windows EXE](VoxelSandbox-2.0-Stage2-Windows-x64.zip) → распаковать → запустить VoxelSandbox.exe.

Добавлены Survival/Creative, здоровье, голод, воздух, смерть/возрождение, консоль `/gamemode` и `/gamerule`, RON hot-reload. Новые миры появляются на безопасной поверхности. 17 исходных PNG не изменены и встроены в EXE.

Это этап 2 из 8, не готовая полная 2.0. Пока остаются бесконечные блоки и мгновенная добыча Foundation; конечный инвентарь и дроп — этап 3. Новые статы, режим и gamerules пока сбрасываются при загрузке; полное хранение — этап 8. Миры отдельно в saves-v2/.

112 Linux-тестов и 80 Windows-тестов через Wine прошли; EXE запущен через Wine/Vulkan. Настоящая Windows-машина пока не проверена.

- [Исходники](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/f464a9c1cd27956561aa8c948e73d43f66bdbe68)
- [Отчёт и команды](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/blob/f464a9c1cd27956561aa8c948e73d43f66bdbe68/docs/STAGE_2_SURVIVAL.md)
- [Все новые файлы целиком и diff изменённых](stage-1-2.patch)
- [Сведения о сборке](BUILD-INFO.json) · [SHA-256](SHA256SUMS.txt)
- [Foundation 1.0 — прежний архив](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/raw/9a34e8fdf7dbbca1d9d1ae55a12fe1234355cbba/VoxelSandbox-Windows-x64.zip)

Снимки запуска в `qa/` показывают новый HUD, утопление, смерть и возрождение Windows-превью. Нажмите «дальше», чтобы перейти к этапу 3: предметы, стеки, дроп, конечный инвентарь и контейнеры.
