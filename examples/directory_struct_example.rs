use roguest::fs::Directory;

fn main() -> std::io::Result<()> {
    println!("=== Пример использования структуры Directory ===");

    // Создание нового объекта директории
    let mut dir = Directory::new("example_dir_obj");

    // Создание директории
    dir.create()?;
    println!("Создана директория: {}", dir.get_path());

    // Проверка существования директории
    if dir.exists() {
        println!("Директория {} существует", dir.get_path());
    }

    // Проверка является ли путь директорией
    println!(
        "{} - это директория: {}",
        dir.get_path(),
        dir.is_directory()
    );

    // Создание файла внутри директории
    std::fs::write(
        "example_dir_obj/test_file.txt",
        "Содержимое тестового файла",
    )?;
    println!("Создан файл внутри директории");

    // Создание поддиректории
    let subdir = Directory::new("example_dir_obj/subdir");
    subdir.create()?;
    println!("Создана поддиректория");

    // Создание файла в поддиректории
    std::fs::write(
        "example_dir_obj/subdir/subfile.txt",
        "Содержимое файла в поддиректории",
    )?;
    println!("Создан файл в поддиректории");

    // Чтение содержимого директории
    let contents = dir.read()?;
    println!("Содержимое директории: {:?}", contents);

    // Чтение содержимого директории с типами
    let contents_with_types = dir.read_with_types()?;
    println!("Содержимое директории с типами: {:?}", contents_with_types);

    // Подсчет файлов в директории (рекурсивно)
    let file_count = dir.count_files()?;
    println!(
        "Количество файлов в директории (рекурсивно): {}",
        file_count
    );

    // Подсчет поддиректорий
    let subdir_count = dir.count_subdirectories()?;
    println!("Количество поддиректорий: {}", subdir_count);

    // Получение размера директории
    let size = dir.size()?;
    println!("Размер директории: {} байт", size);

    // Копирование директории
    let dest_dir = "copied_dir_obj";
    dir.copy_to(dest_dir)?;
    println!("Директория скопирована в: {}", dest_dir);

    // Проверка скопированного содержимого
    let copied_dir = Directory::new(dest_dir);
    if copied_dir.exists() {
        println!(
            "Скопированная директория {} существует",
            copied_dir.get_path()
        );
        let copied_contents = copied_dir.read()?;
        println!("Содержимое скопированной директории: {:?}", copied_contents);
    }

    // Перемещение директории
    let moved_dir = "moved_dir_obj";
    let mut dir_to_move = Directory::new(dest_dir);
    dir_to_move.move_to(moved_dir)?;
    println!("Директория перемещена в: {}", moved_dir);

    // Проверка перемещенной директории
    let mut moved_directory = Directory::new(moved_dir);
    if moved_directory.exists() {
        println!(
            "Перемещенная директория {} существует",
            moved_directory.get_path()
        );
    }

    // Переименование директории
    let renamed_dir = "renamed_dir_obj";
    moved_directory.rename(renamed_dir)?;
    println!("Директория переименована в: {}", renamed_dir);

    // Удаление директории рекурсивно
    let final_dir = Directory::new(renamed_dir);
    final_dir.delete_recursive()?;
    println!("Директория {} удалена рекурсивно", final_dir.get_path());

    // Удаление исходной директории
    dir.delete_recursive()?;
    println!("Исходная директория {} удалена рекурсивно", dir.get_path());

    println!("=== Пример завершен успешно ===");

    Ok(())
}
