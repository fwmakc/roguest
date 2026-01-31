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
