use roguest::fs::Directory;
use roguest::fs::FileType;
use std::fs;
use std::path::Path;

#[test]
fn test_directory_operations() {
    let dir_path = "test_dir_obj";
    let mut dir = Directory::new(dir_path);

    // Создание директории
    dir.create().unwrap();
    assert!(dir.exists());
    assert!(dir.is_directory());

    // Создание файла внутри директории
    let file_path = format!("{}/test_file.txt", dir_path);
    fs::write(&file_path, "Test content").unwrap();

    // Чтение содержимого директории
    let contents = dir.read().unwrap();
    assert!(contents.contains(&"test_file.txt".to_string()));

    // Чтение с типами
    let contents_with_types = dir.read_with_types().unwrap();
    assert!(
        contents_with_types
            .iter()
            .any(|(name, _)| name == "test_file.txt")
    );

    // Подсчет файлов
    let file_count = dir.count_files().unwrap();
    assert_eq!(file_count, 1);

    // Подсчет поддиректорий
    let subdir_count = dir.count_subdirectories().unwrap();
    assert_eq!(subdir_count, 0);

    // Создание поддиректории
    let subdir_path = format!("{}/subdir", dir_path);
    let subdir = Directory::new(&subdir_path);
    subdir.create().unwrap();

    // Подсчет поддиректорий снова
    let subdir_count = dir.count_subdirectories().unwrap();
    assert_eq!(subdir_count, 1);

    // Переименование директории
    let new_dir_path = "renamed_dir_obj";
    dir.rename(new_dir_path).unwrap();
    assert!(Directory::new(new_dir_path).exists());
    assert!(!Directory::new(dir_path).exists());

    // Удаление директории рекурсивно
    let renamed_dir = Directory::new(new_dir_path);
    renamed_dir.delete_recursive().unwrap();
    assert!(!renamed_dir.exists());
}

#[test]
fn test_directory_copy() {
    let src_dir_path = "source_dir_obj";
    let dest_dir_path = "destination_dir_obj";

    // Создание исходной директории
    let src_dir = Directory::new(src_dir_path);
    src_dir.create().unwrap();

    // Создание файла в исходной директории
    let file_path = format!("{}/copy_test_file.txt", src_dir_path);
    fs::write(&file_path, "Copy test content").unwrap();

    // Создание поддиректории с файлом
    let subdir_path = format!("{}/subdir", src_dir_path);
    let subdir = Directory::new(&subdir_path);
    subdir.create().unwrap();
    let subfile_path = format!("{}/subfile.txt", subdir_path);
    fs::write(&subfile_path, "Subfile content").unwrap();

    // Копирование директории
    src_dir.copy_to(dest_dir_path).unwrap();
    assert!(Directory::new(dest_dir_path).exists());

    // Проверка скопированного файла
    let copied_file_path = format!("{}/copy_test_file.txt", dest_dir_path);
    assert!(Path::new(&copied_file_path).exists());
    let content = fs::read_to_string(&copied_file_path).unwrap();
    assert_eq!(content, "Copy test content");

    // Проверка скопированной поддиректории
    let copied_subdir_path = format!("{}/subdir", dest_dir_path);
    assert!(Path::new(&copied_subdir_path).exists());
    let copied_subfile_path = format!("{}/subfile.txt", copied_subdir_path);
    assert!(Path::new(&copied_subfile_path).exists());
    let subcontent = fs::read_to_string(&copied_subfile_path).unwrap();
    assert_eq!(subcontent, "Subfile content");

    // Удаление директорий
    src_dir.delete_recursive().unwrap();
    let dest_dir = Directory::new(dest_dir_path);
    dest_dir.delete_recursive().unwrap();
}
