use roguest::fs::File;
use std::fs;

#[test]
fn test_file_operations() {
    let file_path = "test_file_obj.txt";
    let mut file = File::new(file_path);

    // Создание файла
    file.create_with_content("Hello, file object!").unwrap();
    assert!(file.exists());

    // Чтение файла
    let content = file.read().unwrap();
    assert_eq!(content, "Hello, file object!");

    // Запись в файл
    file.write("New content").unwrap();
    let content = file.read().unwrap();
    assert_eq!(content, "New content");

    // Добавление в файл
    file.append("\nAppended content").unwrap();
    let content = file.read().unwrap();
    assert_eq!(content, "New content\nAppended content");

    // Размер файла
    let size = file.size().unwrap();
    assert!(size > 0);

    // Переименование
    let new_path = "renamed_file_obj.txt";
    file.rename(new_path).unwrap();
    assert!(File::new(new_path).exists());
    assert!(!File::new(file_path).exists());

    // Удаление файла
    let renamed_file = File::new(new_path);
    renamed_file.delete().unwrap();
    assert!(!renamed_file.exists());
}

#[test]
fn test_file_with_memory_buffer() {
    let file_path = "test_mem_file.txt";
    let mut file = File::new(file_path);

    // Установка содержимого в памяти
    file.set_content("Memory content".to_string());

    // Сохранение в файл
    file.save().unwrap();
    assert!(file.exists());

    // Загрузка содержимого в память
    file.load().unwrap();
    assert_eq!(file.get_content().unwrap(), "Memory content");

    // Удаление файла
    file.delete().unwrap();
}
