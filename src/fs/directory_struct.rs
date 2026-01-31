use std::fs;
use std::io;
use std::path::Path;

/// Структура для работы с директорией
pub struct Directory {
    path: String,
}

impl Directory {
    /// Создание нового объекта директории
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Создание директории
    pub fn create(&self) -> io::Result<()> {
        fs::create_dir_all(&self.path)
    }

    /// Удаление директории (пустой)
    pub fn delete(&self) -> io::Result<()> {
        fs::remove_dir(&self.path)
    }

    /// Удаление директории (рекурсивно со всем содержимым)
    pub fn delete_recursive(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.path)
    }

    /// Получение содержимого директории
    pub fn read(&self) -> io::Result<Vec<String>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            entries.push(entry.file_name().to_string_lossy().to_string());
        }
        Ok(entries)
    }

    /// Получение содержимого директории с информацией о типе (файл/директория)
    pub fn read_with_types(&self) -> io::Result<Vec<(String, FileType)>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_type = if entry.file_type()?.is_dir() {
                FileType::Directory
            } else {
                FileType::File
            };
            entries.push((file_name, file_type));
        }
        Ok(entries)
    }

    /// Проверка существования директории
    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Проверка является ли путь директорией
    pub fn is_directory(&self) -> bool {
        Path::new(&self.path).is_dir()
    }

    /// Переименование/перемещение директории
    pub fn rename(&mut self, new_path: &str) -> io::Result<()> {
        fs::rename(&self.path, new_path)?;
        self.path = new_path.to_string();
        Ok(())
    }

    /// Получение информации о директории
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.path)
    }

    /// Получение размера директории (суммарный размер всех файлов)
    pub fn size(&self) -> io::Result<u64> {
        let mut total_size = 0;
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let metadata = entry.metadata()?;
                total_size += metadata.len();
            } else if entry.file_type()?.is_dir() {
                // Рекурсивно подсчитываем размер вложенных файлов
                let subdir = Directory::new(&entry.path().to_string_lossy());
                total_size += subdir.size()?;
            }
        }
        Ok(total_size)
    }

    /// Получение пути директории
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Установка нового пути для директории
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    /// Копирование директории (рекурсивно)
    pub fn copy_to(&self, destination: &str) -> io::Result<()> {
        use std::fs;
        use std::path::Path;

        let src_path = Path::new(&self.path);
        let dest_path = Path::new(destination);

        // Создаем целевую директорию
        fs::create_dir_all(dest_path)?;

        // Рекурсивно копируем содержимое
        for entry in fs::read_dir(src_path)? {
            let entry = entry?;
            let src_entry_path = entry.path();
            let dest_entry_path = dest_path.join(entry.file_name());

            if entry.file_type()?.is_dir() {
                // Рекурсивно копируем поддиректорию
                let subdir = Directory::new(&src_entry_path.to_string_lossy());
                subdir.copy_to(&dest_entry_path.to_string_lossy())?;
            } else {
                // Копируем файл
                fs::copy(&src_entry_path, &dest_entry_path)?;
            }
        }

        Ok(())
    }

    /// Перемещение директории
    pub fn move_to(&mut self, destination: &str) -> io::Result<()> {
        fs::rename(&self.path, destination)?;
        self.path = destination.to_string();
        Ok(())
    }

    /// Подсчет количества файлов в директории (рекурсивно)
    pub fn count_files(&self) -> io::Result<usize> {
        let mut count = 0;
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                count += 1;
            } else if entry.file_type()?.is_dir() {
                // Рекурсивно считаем файлы во вложенных директориях
                let subdir = Directory::new(&entry.path().to_string_lossy());
                count += subdir.count_files()?;
            }
        }
        Ok(count)
    }

    /// Подсчет количества поддиректорий
    pub fn count_subdirectories(&self) -> io::Result<usize> {
        let mut count = 0;
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                count += 1;
            }
        }
        Ok(count)
    }
}

/// Тип файла для определения файл/директория
#[derive(Debug, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
}
