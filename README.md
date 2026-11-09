# Voxel Sandbox 2.0 Survival — этап 3 из 9, альфа

[Скачать Windows ZIP с EXE](VoxelSandbox-2.0-Stage3-Windows-x64.zip). Распакуйте архив целиком и запустите VoxelSandbox.exe.

**Инвентарь, дроп и сундуки пока действуют до закрытия мира. Полное сохранение новых систем — этап 9.** Обычные постройки сохраняются; на месте временного сундука сохраняется воздух. Для альфы используйте копию мира отдельно от Foundation.

Добавлены конечный инвентарь Survival (9+27+4+1), стеки64/16/1, перетаскивание/ПКМ/Shift, Q/Ctrl+Q, физический дроп/подбор/объединение/TTL, сундук27слотов, смерть/keepInventory и проверяемый RON/hot-reload. Creative сохраняет бесконечный каталог23предметов. Сундук временно использует исходные доски; новые предметные значки TODO(art). 17 оригинальных PNG не изменены и встроены в EXE.

Проверено180Linux-тестов,179Windows-тестов через Wine, Clippy и запуск Linux/Xvfb/Vulkan + Windows EXE/Wine. Настоящая Windows-машина и целевая производительность GPU пока не проверены. Новых зависимостей нет.

- [Исходники](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/9c8a3d700b72fc938f15dfc8f54d8d5a1f31368a)
- [Новые файлы целиком и diff](stage-3.patch)
- [Полный отчёт и команды](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/blob/9c8a3d700b72fc938f15dfc8f54d8d5a1f31368a/docs/STAGE_3_INVENTORY.md)
- [Сведения о сборке](BUILD-INFO.json) · [SHA-256](SHA256SUMS.txt)

Скриншоты — в qa/. Следующий этап4: руды, добыча по времени и трещины — после «дальше».
