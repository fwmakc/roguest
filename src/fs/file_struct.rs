use std::fs;
use std::io;
use std::path::Path;

/// Структура для работы с файлом
pub struct File {
    path: String,
    content: Option<String>,
}

impl File {
    /// Создание нового объекта файла
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            content: None,
        }
    }

    /// Загрузка содержимого файла в память
    pub fn load(&mut self) -> io::Result<()> {
        let content = fs::read_to_string(&self.path)?;
        self.content = Some(content);
        Ok(())
    }

    /// Сохранение содержимого файла на диск
    pub fn save(&self) -> io::Result<()> {
        if let Some(content) = &self.content {
            fs::write(&self.path, content)?;
        } else {
            // Если контента нет, создаем пустой файл
            fs::write(&self.path, "")?;
        }
        Ok(())
    }

    /// Создание файла
    pub fn create(&self) -> io::Result<()> {
        fs::write(&self.path, "")
    }

    /// Создание файла с содержимым
    pub fn create_with_content(&self, content: &str) -> io::Result<()> {
        fs::write(&self.path, content)
    }

    /// Чтение содержимого файла
    pub fn read(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Запись в файл (перезапись)
    pub fn write(&self, content: &str) -> io::Result<()> {
        fs::write(&self.path, content)
    }

    /// Добавление в конец файла
    pub fn append(&self, content: &str) -> io::Result<()> {
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.path)?;

        use std::io::Write;
        file.write_all(content.as_bytes())
    }

    /// Удаление файла
    pub fn delete(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }

    /// Переименование/перемещение файла
    pub fn rename(&mut self, new_path: &str) -> io::Result<()> {
        fs::rename(&self.path, new_path)?;
        self.path = new_path.to_string();
        Ok(())
    }

    /// Получение информации о файле
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.path)
    }

    /// Проверка существования файла
    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Проверка является ли путь файлом
    pub fn is_file(&self) -> bool {
        Path::new(&self.path).is_file()
    }

    /// Получение размера файла в байтах
    pub fn size(&self) -> io::Result<u64> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len())
    }

    /// Получение пути файла
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Установка нового пути для файла
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    /// Получение содержимого файла из памяти (если загружено)
    pub fn get_content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    /// Установка содержимого файла в памяти
    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    /// Очистка содержимого файла в памяти
    pub fn clear_content(&mut self) {
        self.content = None;
    }
}
