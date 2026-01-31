use roguest::fs::File;

fn main() -> std::io::Result<()> {
    println!("=== Пример использования структуры File ===");

    // Создание нового объекта файла
    let mut file = File::new("example_file.txt");

    // Создание файла с содержимым
    file.create_with_content("Привет, это пример работы со структурой File!")?;
    println!("Создан файл: {}", file.get_path());

    // Чтение содержимого файла
    let content = file.read()?;
    println!("Содержимое файла: '{}'", content);

    // Запись нового содержимого
    file.write("Новое содержимое файла")?;
    println!("Записано новое содержимое");

    // Добавление к существующему содержимому
    file.append("\nДополнительная строка")?;
    println!("Добавлена строка к содержимому");

    // Чтение обновленного содержимого
    let updated_content = file.read()?;
    println!("Обновленное содержимое: '{}'", updated_content);

    // Проверка существования файла
    if file.exists() {
        println!("Файл {} существует", file.get_path());
    }

    // Проверка типа файла
    println!("{} - это файл: {}", file.get_path(), file.is_file());

    // Получение размера файла
    let size = file.size()?;
    println!("Размер файла: {} байт", size);

    // Работа содержимым в памяти
    file.load()?;
    println!("Содержимое файла загружено в память");

    // Получение содержимого из памяти
    if let Some(content) = file.get_content() {
        println!("Содержимое из памяти: '{}'", content);
    }

    // Установка нового содержимого в памяти
    file.set_content("Содержимое в памяти".to_string());
    println!("Установлено новое содержимое в памяти");

    // Сохранение содержимого из памяти в файл
    file.save()?;
    println!("Содержимое из памяти сохранено в файл");

    // Переименование файла
    file.rename("renamed_example_file.txt")?;
    println!("Файл переименован в: {}", file.get_path());

    // Удаление файла
    file.delete()?;
    println!("Файл {} удален", file.get_path());

    println!("=== Пример завершен успешно ===");

    Ok(())
}
